use std::path::PathBuf;

use bytes::Bytes;
use tokio;

use flyte::{local::LocalFilesystem, local::LocalFilesystemBuilder, Filesystem};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let local_fs: LocalFilesystem = LocalFilesystemBuilder::new().into();
    let tmp_path = String::from("test");
    local_fs.create_directory(&tmp_path).await?;

    for i in 1..=10 {
        local_fs
            .write_file(
                &format!("{}/foo{}.txt", tmp_path, i),
                Some(&Bytes::from(format!("bar{}", i))),
            )
            .await?;
    }

    let nodes = local_fs.list_directory(&tmp_path).await?;
    for node in nodes {
        println!("- {}", node);
    }

    local_fs.delete_directory(&tmp_path).await?;

    Ok(())
}
