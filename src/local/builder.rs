use crate::local::LocalFilesystem;

#[derive(Default)]
pub struct LocalFilesystemBuilder {
    file_permissions: Option<u32>,
    directory_permissions: Option<u32>,
}

impl LocalFilesystemBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_file_permissions(&mut self, value: u32) -> &mut Self {
        self.file_permissions = Some(value);
        self
    }

    pub fn with_directory_permissions(&mut self, value: u32) -> &mut Self {
        self.directory_permissions = Some(value);
        self
    }

    pub fn into_boxed(self) -> Box<LocalFilesystem> {
        Box::new(self.into())
    }
}

impl Into<LocalFilesystem> for LocalFilesystemBuilder {
    fn into(self) -> LocalFilesystem {
        LocalFilesystem::new(self.file_permissions, self.directory_permissions)
    }
}
