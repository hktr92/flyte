pub use self::builder::S3FilesystemBuilder;
pub use self::filesystem::S3Filesystem;

use crate::util::Node;

pub(crate) mod builder;
pub(crate) mod filesystem;

pub(self) mod util {
    use crate::util::Node;

    pub struct S3Node {
        pub nodes: Vec<Node>,
        pub truncated: bool,
        pub continuation_token: Option<String>,
    }
}
