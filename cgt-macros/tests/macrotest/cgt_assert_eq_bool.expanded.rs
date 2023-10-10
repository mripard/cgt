pub fn main() {
    if (true) != (true) {
        return cgt_core::TestResult::Failure(
            cgt_core::TestError::NotEqual(
                {
                    let res = ::alloc::fmt::format(format_args!("{0:#?}", true));
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
