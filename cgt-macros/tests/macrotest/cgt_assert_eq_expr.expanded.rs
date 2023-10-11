pub fn main() {
    if (1 + 2) != (3) {
        return cgt_core::TestResult::Failure(
            cgt_core::TestError::NotEqual(
                {
                    let res = ::alloc::fmt::format(format_args!("{0:#?}", 1 + 2));
                    res
                },
                {
                    let res = ::alloc::fmt::format(format_args!("{0:#?}", 3));
                    res
                },
            ),
        );
    }
}
