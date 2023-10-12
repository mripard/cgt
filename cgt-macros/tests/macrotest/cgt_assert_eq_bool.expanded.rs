pub fn main() {
    if (true) != (true) {
        return Err(
            TestError::NotEqual(
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
