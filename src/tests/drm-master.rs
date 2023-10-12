use super::prelude::*;

// Test that if we are a master already, we can successfully ask to be a master again.
#[cgt_test_with_fd(master)]
fn test_drm_master_again(fd: BorrowedFd<'_>) -> Result<(), TestError> {
    let res = set_master(fd);

    cgt_assert_ok!(res);

    Ok(())
}

// Test that calling a DRM_MASTER only operation succeeds if we are a master.
#[cgt_test_with_fd(master)]
fn test_drm_master_ops(fd: BorrowedFd<'_>) -> Result<(), TestError> {
    let res = unsafe { drm_ioctl_attach_mode(fd.as_raw_fd()) };

    cgt_assert_ok!(res);

    Ok(())
}

// Test that calling a DRM_MASTER only operation fails if we are no longer a master.
#[cgt_test_with_fd]
fn test_drm_master_ops_not_master(fd: BorrowedFd<'_>) -> Result<(), TestError> {
    drop_master(fd)?;

    let res = unsafe { drm_ioctl_attach_mode(fd.as_raw_fd()) };

    cgt_assert_err!(res);

    Ok(())
}

// Test that if there's already a master registered, a separate fd can't try to claim it.
#[cgt_test_with_path]
fn test_drm_master_multiple_masters(path: &Path) -> Result<(), TestError> {
    let master = File::open(path)?;

    cgt_assert_ok!(set_master(master.as_fd()));

    let second = File::open(path)?;

    cgt_assert_err!(set_master(second.as_fd()));

    Ok(())
}
