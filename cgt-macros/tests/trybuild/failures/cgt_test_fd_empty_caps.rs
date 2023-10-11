#[cgt_macros::cgt_test_with_fd(capabilities)]
fn test(_: std::os::fd::BorrowedFd<'_>) -> cgt_core::TestResult {
    cgt_core::TestResult::Success
}

fn main() {}