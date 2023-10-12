#[cgt_macros::cgt_test_with_path(attribute)]
fn test(_: &std::path::Path) -> Result<(), cgt_core::TestError> {
    Ok(())
}

fn main() {}
