use std::collections::HashMap;

use testanything::tap_writer::TapWriter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TestError {
    #[error("Condition {0} is not true")]
    ConditionUnmet(String),

    #[error("Unknown Error")]
    Unspecified,
}

impl PartialEq for TestError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ConditionUnmet(l0), Self::ConditionUnmet(r0)) => l0 == r0,
            (Self::Unspecified, Self::Unspecified) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TestFunction {
    NoArg(fn() -> Result<(), TestError>),
}

#[derive(Clone, Debug)]
pub struct Test {
    pub module_name: &'static str,
    pub test_name: &'static str,
    pub test_fn: TestFunction,
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

pub fn run_all() {
    for (test_module, tests) in get_test_suites() {
        let writer = TapWriter::new(&test_module);
        let mut num = 0;

        writer.name();

        for test in tests {
            num += 1;

            let res = match test.test_fn {
                TestFunction::NoArg(f) => f(),
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
