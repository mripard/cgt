#[cgt_macros::cgt_test_with_fd(capabilities=[Unknown])]
fn test(_: std::os::fd::BorrowedFd<'_>) -> Result<(), cgt_core::TestError> {
    Ok(())
}

fn main() {}