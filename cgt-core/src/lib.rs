use std::{
    collections::HashMap,
    fs::File,
    os::fd::{AsFd, AsRawFd, BorrowedFd},
    path::{Path, PathBuf},
};

use drm_helpers::{set_client_capability, set_master};
use glob::glob;
use testanything::tap_writer::TapWriter;
use thiserror::Error;

use drm_uapi::{drm_ioctl_version, drm_version, ClientCapability};

#[derive(Debug, Error)]
pub enum TestError {
    #[error("Condition {0} is not true")]
    ConditionUnmet(String),

    #[error("I/O Error")]
    Io(#[from] std::io::Error),

    #[error("Values {0} and {1} are not equal")]
    NotEqual(String, String),

    #[error("Result {0} isn't an error")]
    ResultNotError(String),

    #[error("Result {0} isn't a value")]
    ResultNotOk(String),

    #[error("Unknown Error")]
    Unspecified,
}

impl PartialEq for TestError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ConditionUnmet(l0), Self::ConditionUnmet(r0)) => l0 == r0,
            (Self::Io(l0), Self::Io(r0)) => l0.raw_os_error() == r0.raw_os_error(),
            (Self::NotEqual(l0, l1), Self::NotEqual(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::ResultNotError(l0), Self::ResultNotError(r0)) => l0 == r0,
            (Self::ResultNotOk(l0), Self::ResultNotOk(r0)) => l0 == r0,
            (Self::Unspecified, Self::Unspecified) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl From<nix::Error> for TestError {
    fn from(value: nix::Error) -> Self {
        let io_err: Result<std::io::Error, _> = value.try_into();

        if let Ok(err) = io_err {
            err.into()
        } else {
            TestError::Unspecified
        }
    }
}

#[derive(Clone, Debug)]
pub enum TestFunction {
    NoArg(fn() -> Result<(), TestError>),
    WithFd(fn(BorrowedFd) -> Result<(), TestError>),
    WithPath(fn(&Path) -> Result<(), TestError>),
}

#[derive(Clone, Debug)]
pub struct Test {
    pub module_name: &'static str,
    pub test_name: &'static str,
    pub test_fn: TestFunction,
    pub master: bool,
    pub client_capabilities: [Option<ClientCapability>; 8],
}

inventory::collect!(Test);

fn get_test_suites() -> HashMap<String, Vec<Test>> {
    let mut map = HashMap::new();

    for test in inventory::iter::<Test> {
        if !map.contains_key(test.module_name) {
            map.insert(test.module_name.to_string(), Vec::new());
        }

        map.get_mut(test.module_name).unwrap().push(test.clone());
    }

    map
}

pub enum DeviceSpecifier {
    ModuleName(String),
    Path(PathBuf),
}

fn find_device(dev: DeviceSpecifier) -> Result<PathBuf, TestError> {
    match dev {
        DeviceSpecifier::ModuleName(module) => {
            for entry in glob("/dev/dri/card*").expect("Failed to read glob pattern") {
                match entry {
                    Ok(ref path) => {
                        let f = File::open(path)?;

                        let mut count = drm_version::default();

                        unsafe { drm_ioctl_version(f.as_fd().as_raw_fd(), &mut count) }?;

                        let mut name: Vec<u8> = Vec::with_capacity(count.name_len);

                        let mut data = drm_version {
                            name_len: count.name_len,
                            name: name.as_mut_ptr() as u64,

                            ..Default::default()
                        };

                        unsafe {
                            drm_ioctl_version(f.as_fd().as_raw_fd(), &mut data)?;
                            name.set_len(data.name_len);
                        };

                        if String::from_utf8_lossy(&name) == module {
                            return Ok(path.clone());
                        }
                    }

                    Err(e) => return Err(e.into_error().into()),
                }
            }

            Err(nix::errno::Errno::ENODEV.into())
        }
        DeviceSpecifier::Path(p) => Ok(p),
    }
}

pub fn run_all(dev: DeviceSpecifier) {
    let path = find_device(dev).unwrap();

    for (test_module, tests) in get_test_suites() {
        let writer = TapWriter::new(&test_module);
        let mut num = 0;

        writer.name();

        for test in tests {
            num += 1;

            let res = match test.test_fn {
                TestFunction::NoArg(f) => f(),
                TestFunction::WithFd(f) => {
                    File::open(&path).map_err(|e| e.into()).and_then(|file| {
                        let fd = file.as_fd();

                        if test.master {
                            set_master(fd)?;
                        }

                        for cap in test.client_capabilities.into_iter().flatten() {
                            set_client_capability(fd, cap)?;
                        }

                        f(file.as_fd())
                    })
                }
                TestFunction::WithPath(f) => f(&path),
            };

            match res {
                Ok(_) => writer.ok(num, test.test_name),
                Err(e) => {
                    writer.not_ok(num, test.test_name);
                    writer.diagnostic(&e.to_string());
                }
            }
        }

        writer.plan(1, num);
    }
}
