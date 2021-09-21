use anyhow::Result;
use bytes::Bytes;
use futures::future;

use crate::core::Filesystem;
use crate::util::Node;

pub struct FilesystemChain {
    systems: Vec<Box<dyn Filesystem>>,
}

impl FilesystemChain {
    pub fn new(systems: Vec<Box<dyn Filesystem>>) -> Self {
        Self { systems }
    }

    async fn delete_file(&self, path: &String) {
        let mut fut = vec![];
        for system in self.systems.iter() {
            fut.push(system.delete_file(path));
        }

        future::join_all(fut).await;
    }

    async fn delete_directory(&self, path: &String) {
        let mut fut = vec![];
        for system in self.systems.iter() {
            fut.push(system.delete_directory(path));
        }

        future::join_all(fut).await;
    }

    async fn list_directory(&self, path: &String) -> Result<Vec<Vec<Node>>> {
        let mut nodes = vec![];
        for system in self.systems.iter() {
            nodes.push(system.list_directory(path).await?);
        }

        Ok(nodes)
    }

    async fn create_directory(&self, path: &String) {
        let mut fut = vec![];
        for system in self.systems.iter() {
            fut.push(system.create_directory(path));
        }

        future::join_all(fut).await;
    }

    async fn write_file(&self, path: &String, contents: &Option<Bytes>) {
        let mut fut = vec![];
        for system in self.systems.iter() {
            fut.push(system.write_file(path, contents));
        }

        future::join_all(fut).await;
    }
}
