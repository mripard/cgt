mod prelude {
    pub use std::os::fd::{AsRawFd, BorrowedFd};

    pub use cgt_core::TestError;
    pub use cgt_macros::*;
    pub use drm_helpers::*;
    pub use drm_uapi::{ClientCapability::*, *};
}

automod::dir!("src/tests");
