mod prelude {
    pub use std::{
        fs::File,
        os::fd::{AsRawFd, BorrowedFd},
        path::Path,
    };

    pub use cgt_core::TestResult;
    pub use cgt_macros::*;
    pub use drm_helpers::*;
    pub use drm_uapi::{ClientCapability::*, *};
}

automod::dir!("src/tests");
