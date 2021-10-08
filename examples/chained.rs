use std::path::PathBuf;

use bytes::Bytes;
use tokio;

use flyte::{local::LocalFilesystem, local::LocalFilesystemBuilder, Filesystem, FilesystemChain};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let strict_fs = LocalFilesystemBuilder::new()
        .with_prefix("secret".into())
        .with_directory_permissions(0o700)
        .with_file_permissions(0o644)
        .into_boxed();

    let public_fs = LocalFilesystemBuilder::new()
        .with_prefix("public".into())
        .with_file_permissions(0o777)
        .into_boxed();

    let chain = FilesystemChain::new(vec![strict_fs, public_fs]);

    let tmp_path = String::from("test");
    chain.create_directory(&tmp_path).await?;

    for i in 1..=10 {
        chain
            .write_file(
                &format!("{}/foo{}.txt", tmp_path, i),
                Some(&Bytes::from(format!("bar{}", i))),
            )
            .await?;
    }

    let nodes = chain.list_directory(&tmp_path).await?;
    for set in nodes {
        for node in set {
            println!("- {}", node);
        }
    }

    chain.delete_directory(&tmp_path).await?;

    Ok(())
}
