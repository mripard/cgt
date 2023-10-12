#[cgt_macros::cgt_test_with_fd(capabilities = "Wrong")]
fn test(_: std::os::fd::BorrowedFd<'_>) -> Result<(), TestResult> {
    Ok(())
}

fn main() {}