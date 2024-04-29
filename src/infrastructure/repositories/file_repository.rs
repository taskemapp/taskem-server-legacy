use std::sync::Arc;

use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ChecksumMode;
use aws_sdk_s3::Client;
use tonic::codegen::Body;

use crate::domain::error::Error;
use crate::domain::repositories::file::FileRepository;

struct FileRepositoryImpl {
    client: Arc<Client>,
}

impl FileRepository for FileRepositoryImpl {
    async fn upload(
        &self,
        bucket: &str,
        key: &str,
        data: ByteStream,
    ) -> crate::domain::error::Result<()> {
        let client = self.client.clone();

        client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(data)
            .content_type("image/jpeg")
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to upload file: {:?}", e);
                Error::RepositoryError
            })?;
        Ok(())
    }

    async fn download(&self, bucket: &str, key: &str) -> crate::domain::error::Result<ByteStream> {
        let client = self.client.clone();

        let file = client
            .get_object()
            .bucket(bucket)
            .key(key)
            .checksum_mode(ChecksumMode::Enabled)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to upload file: {:?}", e);
                Error::RepositoryError
            })?;
        Ok(file.body)
    }

    async fn delete(&self, bucket: &str, key: &str) -> crate::domain::error::Result<()> {
        let client = self.client.clone();

        client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to upload file: {:?}", e);
                Error::RepositoryError
            })?;
        Ok(())
    }
}
