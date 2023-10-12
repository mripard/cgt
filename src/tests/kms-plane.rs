use super::prelude::*;

// Test that the ioctl to get planes resources succeeds but returns no planes when the Universal
// Planes capability is missing.
#[cgt_test_with_fd]
fn test_get_plane_resources_no_cap(dev: BorrowedFd<'_>) -> Result<(), TestError> {
    let mut data = drm_mode_get_plane_res::default();

    unsafe { drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut data) }?;

    cgt_assert_eq!(data.count_planes, 0);
    cgt_assert_eq!(data.plane_id_ptr, 0);

    Ok(())
}

// Test that the DRM_IOCTL_MODE_GETPLANERESOURCES ioctl is reporting some planes when the Universal
// Planes capability is set.
#[cgt_test_with_fd(capabilities = [UniversalPlanes])]
fn test_get_plane_resources_with_cap(dev: BorrowedFd<'_>) -> Result<(), TestError> {
    let mut data = drm_mode_get_plane_res::default();

    unsafe { drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut data) }?;

    cgt_assert!(data.count_planes > 0);
    cgt_assert_eq!(data.plane_id_ptr, 0);

    Ok(())
}

// Test that we can query the planes IDs through DRM_IOCTL_MODE_GETPLANERESOURCES of all the planes
// listed.
#[cgt_test_with_fd(capabilities = [UniversalPlanes])]
fn test_get_plane_resources_planes_id(dev: BorrowedFd<'_>) -> Result<(), TestError> {
    let mut count = drm_mode_get_plane_res::default();

    unsafe { drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut count) }?;

    cgt_assert!(count.count_planes > 0);

    let mut plane_ids: Vec<u32> = Vec::with_capacity(count.count_planes as usize);

    let mut ids = drm_mode_get_plane_res {
        count_planes: count.count_planes,
        plane_id_ptr: plane_ids.as_mut_ptr() as u64,
    };

    unsafe {
        drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut ids)?;

        plane_ids.set_len(ids.count_planes as usize);
    };

    for plane_id in plane_ids {
        cgt_assert!(plane_id > 0);
    }

    Ok(())
}

// Test that we can call DRM_IOCTL_MODE_GETPLANE on all the planes listed by
// DRM_IOCTL_MODE_GETPLANERESOURCES.
#[cgt_test_with_fd(capabilities = [UniversalPlanes])]
fn test_get_plane_resources_get_plane(dev: BorrowedFd<'_>) -> Result<(), TestError> {
    let mut count = drm_mode_get_plane_res::default();

    unsafe { drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut count) }?;

    cgt_assert!(count.count_planes > 0);

    let mut plane_ids: Vec<u32> = Vec::with_capacity(count.count_planes as usize);

    let mut ids = drm_mode_get_plane_res {
        count_planes: count.count_planes,
        plane_id_ptr: plane_ids.as_mut_ptr() as u64,
    };

    unsafe {
        drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut ids)?;

        plane_ids.set_len(ids.count_planes as usize);
    };

    for plane_id in plane_ids {
        let mut plane = drm_mode_get_plane {
            plane_id,

            ..Default::default()
        };

        unsafe { drm_ioctl_mode_getplane(dev.as_raw_fd(), &mut plane) }?;

        cgt_assert_eq!(plane_id, plane.plane_id);
        cgt_assert!(plane.possible_crtcs > 0);
        cgt_assert!(plane.count_format_types > 0);
    }

    Ok(())
}

// Test that each plane exposed by DRM_IOCTL_MODE_GETPLANE supports XRGB8888
#[cgt_test_with_fd(capabilities = [UniversalPlanes])]
fn test_get_plane_resources_get_plane_formats(dev: BorrowedFd<'_>) -> Result<(), TestError> {
    let mut count = drm_mode_get_plane_res::default();

    unsafe { drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut count) }?;

    cgt_assert!(count.count_planes > 0);

    let mut plane_ids: Vec<u32> = Vec::with_capacity(count.count_planes as usize);

    let mut ids = drm_mode_get_plane_res {
        count_planes: count.count_planes,
        plane_id_ptr: plane_ids.as_mut_ptr() as u64,
    };

    unsafe {
        drm_ioctl_mode_getplaneresources(dev.as_raw_fd(), &mut ids)?;

        plane_ids.set_len(ids.count_planes as usize);
    };

    for plane_id in plane_ids {
        let mut count = drm_mode_get_plane {
            plane_id,

            ..Default::default()
        };

        unsafe { drm_ioctl_mode_getplane(dev.as_raw_fd(), &mut count) }?;

        cgt_assert!(count.count_format_types > 0);

        let mut formats: Vec<u32> = Vec::with_capacity(count.count_format_types as usize);

        let mut plane = drm_mode_get_plane {
            plane_id,
            count_format_types: count.count_format_types,
            format_type_ptr: formats.as_mut_ptr() as u64,

            ..Default::default()
        };

        unsafe {
            drm_ioctl_mode_getplane(dev.as_raw_fd(), &mut plane)?;

            formats.set_len(plane.count_format_types as usize);
        };

        cgt_assert!(formats.contains(&Format::XRGB8888.into_u32()));
    }

    Ok(())
}
