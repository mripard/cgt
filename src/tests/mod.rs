mod prelude {
    pub use std::os::fd::{AsRawFd, BorrowedFd};

    pub use cgt_core::TestError;
    pub use cgt_macros::*;
}

automod::dir!("src/tests");
