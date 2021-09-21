use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;

use crate::util::Node;

#[async_trait]
pub trait Filesystem {
    async fn delete_file(&self, path: &String);
    async fn delete_directory(&self, path: &String);

    // required for s3, used mostly in Self::delete_directory
    async fn list_directory(&self, path: &String) -> Result<Vec<Node>>;

    // local-fs only: fs::create_dir_all
    async fn create_directory(&self, path: &String);

    //
    async fn write_file(&self, path: &String, contents: &Option<Bytes>);
}
