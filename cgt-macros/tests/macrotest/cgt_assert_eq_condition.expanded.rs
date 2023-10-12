pub fn main() {
    if (2 > 1) != (true) {
        return cgt_core::TestResult::Failure(
            cgt_core::TestError::NotEqual(
                {
                    let res = ::alloc::fmt::format(format_args!("{0:#?}", 2 > 1));
                    res
                },
                {
                    let res = ::alloc::fmt::format(format_args!("{0:#?}", true));
                    res
                },
            ),
        );
    }
}
