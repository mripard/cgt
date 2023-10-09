mod prelude {
    pub use std::{
        fs::File,
        os::fd::{AsFd, AsRawFd, BorrowedFd},
        path::Path,
    };

    pub use cgt_core::TestError;
    pub use cgt_macros::*;
    pub use drm_helpers::*;
    pub use drm_uapi::{ClientCapability::*, *};
}

automod::dir!("src/tests");
