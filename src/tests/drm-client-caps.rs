use super::prelude::*;

// Test that we can request a Stereo 3D capability
#[cgt_test_with_fd(capabilities = [Stereo3d])]
fn test_drm_client_caps_stereo_3d(_: BorrowedFd<'_>) -> Result<(), TestError> {
    Ok(())
}

// Test that we can request the Universal Planes capability
#[cgt_test_with_fd(capabilities = [UniversalPlanes])]
fn test_drm_client_caps_universal_planes(_: BorrowedFd<'_>) -> Result<(), TestError> {
    Ok(())
}

// Test that we can request the Atomic capability
#[cgt_test_with_fd(capabilities = [Atomic])]
fn test_drm_client_caps_atomic(_: BorrowedFd<'_>) -> Result<(), TestError> {
    Ok(())
}

// Test that we can request the Aspect Ratio capability
#[cgt_test_with_fd(capabilities = [AspectRatio])]
fn test_drm_client_caps_aspect_ratio(_: BorrowedFd<'_>) -> Result<(), TestError> {
    Ok(())
}

// Test that we can request the Writeback capability if the Atomic Capability is set
#[cgt_test_with_fd(capabilities = [Atomic, WritebackConnectors])]
fn test_drm_client_caps_writeback_with_atomic(_: BorrowedFd<'_>) -> Result<(), TestError> {
    Ok(())
}

// Test that we can't request the Writeback capability if the Atomic Capability is not set
#[cgt_test_with_fd]
fn test_drm_client_caps_writeback_without_atomic(fd: BorrowedFd<'_>) -> Result<(), TestError> {
    let res = set_client_capability(fd, WritebackConnectors);

    cgt_assert_err!(res);

    Ok(())
}
