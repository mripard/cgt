#[test]
fn build() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/successes/*.rs");
    t.compile_fail("tests/trybuild/failures/*.rs");
}
