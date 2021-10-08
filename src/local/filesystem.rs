use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use async_trait::async_trait;
use bytes::Bytes;

use crate::util::Node;
use crate::Filesystem;

#[derive(Clone, Default)]
pub struct LocalFilesystem {
    prefix: Option<String>,
    file_permissions: Option<u32>,
    directory_permissions: Option<u32>,
}

impl LocalFilesystem {
    pub fn new(
        prefix: Option<String>,
        file_permissions: Option<u32>,
        directory_permissions: Option<u32>,
    ) -> Self {
        Self {
            prefix,
            file_permissions,
            directory_permissions,
        }
    }

    pub fn new_boxed(
        prefix: Option<String>,
        file_permissions: Option<u32>,
        directory_permissions: Option<u32>,
    ) -> Box<Self> {
        Box::new(LocalFilesystem::new(
            prefix,
            file_permissions,
            directory_permissions,
        ))
    }

    fn build_path(&self, path: &String) -> PathBuf {
        let mut path_buffer = PathBuf::new();

        if let Some(prefix) = &self.prefix {
            path_buffer.push(&prefix);
        }

        path_buffer.push(&path);

        path_buffer
    }
}

#[async_trait]
impl Filesystem for LocalFilesystem {
    async fn delete_file(&self, path: &String) -> anyhow::Result<()> {
        Ok(std::fs::remove_file(self.build_path(path).as_path())?)
    }

    async fn delete_directory(&self, path: &String) -> anyhow::Result<()> {
        Ok(std::fs::remove_dir_all(self.build_path(path).as_path())?)
    }

    async fn list_directory(&self, path: &String) -> anyhow::Result<Vec<Node>> {
        let nodes = std::fs::read_dir(self.build_path(path).as_path())?
            .into_iter()
            .map(|file| Node(file.unwrap().path().as_path().to_str().unwrap().into()))
            .collect::<Vec<Node>>();

        Ok(nodes)
    }

    async fn create_directory(&self, path: &String) -> anyhow::Result<()> {
        Ok(std::fs::create_dir_all(self.build_path(path).as_path())?)
    }

    async fn write_file(&self, path: &String, contents: Option<&Bytes>) -> anyhow::Result<()> {
        Ok(File::create(self.build_path(path).as_path())?.write_all(contents.unwrap().as_ref())?)
    }
}
