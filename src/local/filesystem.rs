use std::path::Path;

use async_trait::async_trait;
use bytes::Bytes;

use crate::util::Node;

use crate::core::Filesystem;
use crate::Filesystem;

#[derive(Clone, Default)]
pub struct LocalFilesystem {
    file_permissions: Option<u32>,
    directory_permissions: Option<u32>,
}

impl LocalFilesystem {
    pub fn new(file_permissions: Option<u32>, directory_permissions: Option<u32>) -> Self {
        Self {
            file_permissions,
            directory_permissions,
        }
    }

    pub fn new_boxed(
        file_permissions: Option<u32>,
        directory_permissions: Option<u32>,
    ) -> Box<Self> {
        Box::new(LocalFilesystem::new(
            file_permissions,
            directory_permissions,
        ))
    }
}

#[async_trait]
impl Filesystem for LocalFilesystem {
    async fn delete_file(&self, path: &String) {
        std::fs::remove_file(Path::new(path))?
    }

    async fn delete_directory(&self, path: &String) {
        std::fs::remove_dir_all(Path::new(path))?
    }

    async fn list_directory(&self, path: &String) -> anyhow::Result<Vec<Node>> {
        let nodes = std::fs::read_dir(Path::new(path))?
            .into_iter()
            .map(|file| Node(file.unwrap().path().into()))
            .collect::<Vec<Node>>();

        Ok(nodes)
    }

    async fn create_directory(&self, path: &String) {
        std::fs::create_dir_all(Path::new(path))?
    }

    async fn write_file(&self, path: &String, contents: Bytes) {
        std::fs::write(path, contents)?
    }
}
