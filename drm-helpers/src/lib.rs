use std::os::fd::{AsRawFd, BorrowedFd};

use drm_uapi::{
    drm_ioctl_drop_master, drm_ioctl_mode_getplaneresources, drm_ioctl_set_client_cap,
    drm_ioctl_set_master, drm_mode_get_plane_res, drm_setclientcap, ClientCapability,
};
use strum::IntoEnumIterator;

pub fn set_master(fd: BorrowedFd<'_>) -> Result<(), std::io::Error> {
    unsafe { drm_ioctl_set_master(fd.as_raw_fd()) }?;

    Ok(())
}

pub fn drop_master(fd: BorrowedFd<'_>) -> Result<(), std::io::Error> {
    unsafe { drm_ioctl_drop_master(fd.as_raw_fd()) }?;

    Ok(())
}

fn toggle_client_capability(
    fd: BorrowedFd<'_>,
    cap: ClientCapability,
    enable: bool,
) -> Result<(), std::io::Error> {
    let caps = drm_setclientcap {
        capability: cap as u64,
        value: if enable { 1 } else { 0 },
    };

    unsafe { drm_ioctl_set_client_cap(fd.as_raw_fd(), &caps) }?;

    Ok(())
}

pub fn set_client_capability(
    fd: BorrowedFd<'_>,
    cap: ClientCapability,
) -> Result<(), std::io::Error> {
    toggle_client_capability(fd, cap, true)
}

pub fn clear_client_capabilities(fd: BorrowedFd<'_>) -> Result<(), std::io::Error> {
    for cap in ClientCapability::iter() {
        if cap == ClientCapability::WritebackConnectors {
            continue;
        }

        toggle_client_capability(fd, cap, false)?;
    }

    Ok(())
}

pub fn find_planes(dev: BorrowedFd<'_>) -> Result<impl Iterator<Item = u32>, std::io::Error> {
    let mut count = drm_mode_get_plane_res::default();

    unsafe { drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut count) }?;

    let mut plane_ids: Vec<u32> = Vec::with_capacity(count.count_planes as usize);

    let mut ids = drm_mode_get_plane_res {
        count_planes: count.count_planes,
        plane_id_ptr: plane_ids.as_mut_ptr() as u64,
    };

    unsafe {
        drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut ids)?;

        plane_ids.set_len(ids.count_planes as usize);
    }

    Ok(plane_ids.into_iter())
}
