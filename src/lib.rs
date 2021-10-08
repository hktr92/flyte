#[cfg(feature = "local")]
pub mod local;
#[cfg(feature = "s3")]
pub mod s3;

mod core;
mod util;

pub use crate::core::Filesystem;
pub use crate::core::FilesystemChain;
