#[cgt_macros::cgt_test_with_path(attribute)]
fn test(_: &Path) -> cgt_core::TestResult {
    cgt_core::TestResult::Success
}

fn main() {}
