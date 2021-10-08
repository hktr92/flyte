use std::path::Path;

use async_trait::async_trait;
use bytes::Bytes;
use rusoto_s3::{
    Delete, DeleteObjectRequest, DeleteObjectsRequest, ListObjectsV2Request, Object,
    ObjectIdentifier, PutObjectRequest, S3Client, S3,
};

use crate::core::Filesystem;
use crate::s3::util::S3Node;
use crate::util::Node;

#[derive(Clone, Default)]
pub struct S3Filesystem {
    bucket: String,
    client: S3Client,

    // TODO -- unify this with Filesystem
    acl: Option<String>,
    cache_control: Option<String>,
    content_type: Option<String>,
}

impl S3Filesystem {
    pub fn new(
        bucket: String,
        client: S3Client,
        acl: Option<String>,
        cache_control: Option<String>,
        content_type: Option<String>,
    ) -> Self {
        Self {
            bucket,
            client,
            acl,
            cache_control,
            content_type,
        }
    }

    pub fn new_boxed(
        bucket: String,
        client: S3Client,
        acl: Option<String>,
        cache_control: Option<String>,
        content_type: Option<String>,
    ) -> Box<Self> {
        Box::new(S3Filesystem::new(
            bucket,
            client,
            acl,
            cache_control,
            content_type,
        ))
    }

    async fn do_list_nodes(
        &self,
        path: &String,
        continuation_token: Option<String>,
    ) -> anyhow::Result<S3Node> {
        let req = ListObjectsV2Request {
            bucket: self.bucket.clone().into(),
            prefix: Some(path.into()),
            continuation_token,
            ..Default::default()
        };

        let response = self.client.list_objects_v2(req).await?;
        let nodes = response
            .contents
            .unwrap_or(vec![])
            .into_iter()
            .map(|object| Node(object.key?))
            .collect::<Vec<Node>>();

        Ok(S3Node {
            nodes,
            truncated: response.is_truncated.unwrap_or(false),
            continuation_token: response.next_continuation_token,
        })
    }
}

#[async_trait]
impl Filesystem for S3Filesystem {
    async fn delete_file(&self, path: &String) -> anyhow::Result<()> {
        let req = DeleteObjectRequest {
            bucket: self.bucket.clone().into(),
            key: path.into(),
            ..Default::default()
        };

        self.client.delete_object(req).await?;

        Ok(())
    }

    async fn delete_directory(&self, path: &String) -> anyhow::Result<()> {
        let nodes = self
            .list_directory(path)
            .await?
            .into_iter()
            .map(|node| ObjectIdentifier {
                key: node.into(),
                ..Default::default()
            })
            .collect::<Vec<ObjectIdentifier>>();

        let req = DeleteObjectsRequest {
            bucket: self.bucket.clone().into(),
            delete: Delete {
                objects: nodes,
                quiet: Some(true),
            },
            ..Default::default()
        };

        self.client.delete_objects(req).await?;

        Ok(())
    }

    async fn list_directory(&self, path: &String) -> anyhow::Result<Vec<Node>> {
        let mut nodes: Vec<Node> = vec![];
        let mut result = self.do_list_nodes(path, None).await?;

        while result.truncated {
            nodes.append(&mut result.nodes);
            result = self.do_list_nodes(path, result.continuation_token)?;
        }

        // TODO: maybe this is not required
        nodes.append(&mut result.nodes);

        Ok(nodes)
    }

    // aws does not support directories. it's a plain object storage.
    async fn create_directory(&self, path: &String) -> anyhow::Result<()> {
        Ok(())
    }

    async fn write_file(&self, path: &String, contents: Bytes) -> anyhow::Result<()> {
        let req = PutObjectRequest {
            bucket: self.bucket.clone().into(),
            key: path.into(),
            body: Some(contents.into()),
            acl: self.acl.into(),
            cache_control: self.cache_control.into(),
            content_type: self.content_type.into(),
            ..Default::default()
        };

        self.client.put_object(req).await?;

        Ok(())
    }
}
