use rusoto_s3::S3Client;

use crate::s3::S3Filesystem;

#[derive(Default)]
pub struct S3FilesystemBuilder {
    bucket: Option<String>,
    client: Option<S3Client>,
    prefix: Option<String>,
    acl: Option<String>,
    cache_control: Option<String>,
    content_type: Option<String>,
}

impl S3FilesystemBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_bucket(mut self, value: String) -> S3FilesystemBuilder {
        self.bucket = Some(value);
        self
    }
    pub fn with_s3_client(mut self, value: S3Client) -> S3FilesystemBuilder {
        self.client = Some(value);
        self
    }
    pub fn with_prefix(mut self, value: String) -> S3FilesystemBuilder {
        self.prefix = Some(value);
        self
    }
    pub fn with_acl(mut self, value: String) -> S3FilesystemBuilder {
        self.acl = Some(value);
        self
    }
    pub fn with_cache_control(mut self, value: String) -> S3FilesystemBuilder {
        self.cache_control = Some(value);
        self
    }
    pub fn with_content_type(mut self, value: String) -> S3FilesystemBuilder {
        self.content_type = Some(value);
        self
    }

    pub fn into_boxed(self) -> Box<S3Filesystem> {
        Box::new(self.into())
    }
}

impl Into<S3Filesystem> for S3FilesystemBuilder {
    fn into(self) -> S3Filesystem {
        S3Filesystem::new(
            self.bucket.unwrap(),
            self.client.unwrap(),
            self.prefix,
            self.acl,
            self.cache_control,
            self.content_type,
        )
    }
}
