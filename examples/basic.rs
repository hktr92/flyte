use std::path::PathBuf;

use bytes::Bytes;

use flyte::{local::LocalFilesystem, local::LocalFilesystemBuilder, Filesystem};

fn main() {
    let local_fs: LocalFilesystem = LocalFilesystemBuilder::new().into();
    local_fs.create_directory("test".into())?;

    for i in 1..=10 {
        local_fs.write_file(
            &format!("test/foo{}.txt", i),
            Bytes::from(format!("bar{}", i)),
        )?;
    }

    let nodes = local_fs.list_directory("test".into())?;
    for node in nodes {
        println!("- {}", node);
    }
}
