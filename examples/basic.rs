use flyte::local::LocalFilesystem;
use flyte::{local::LocalFilesystemBuilder, Filesystem};
use std::path::PathBuf;

fn main() {
    let local_fs: LocalFilesystem = LocalFilesystemBuilder::new().into();
    local_fs.create_directory("test".into());
}
