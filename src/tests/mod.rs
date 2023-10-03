mod prelude {
    pub use cgt_core::TestError;
    pub use cgt_macros::*;
}

automod::dir!("src/tests");
