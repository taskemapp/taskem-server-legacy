use crate::common::Error;
use crate::domain::repositories::file::FileRepository;
use autometrics::autometrics;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{BucketInfo, BucketType, ChecksumMode};
use aws_sdk_s3::Client;
use derive_new::new;
use sha2::{Digest, Sha256};
use std::sync::Arc;

#[derive(new)]
pub struct FileRepositoryImpl {
    client: Arc<Client>,
}

#[async_trait::async_trait]
#[autometrics]
impl FileRepository for FileRepositoryImpl {
    async fn upload(&self, bucket: &str, key: &str, data: &[u8]) -> crate::common::Result<()> {
        let client = self.client.clone();
        let hash = Sha256::digest(b"hello world");

        let checksum = base16ct::lower::encode_string(&hash);

        let result = client
            .put_object()
            .bucket(bucket)
            .key(key)
            .checksum_sha256(checksum.clone())
            .body(ByteStream::from(data.to_vec()))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to upload file: {:?}", e);
                Error::Repository
            })?;

        let result_checksum = result.checksum_sha256.ok_or(Error::Checksum)?;

        if checksum.ne(&result_checksum) {}

        Ok(())
    }

    async fn download(&self, bucket: &str, key: &str) -> crate::common::Result<Vec<u8>> {
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
                Error::Repository
            })?;

        let bytes = file.body.collect().await.map_err(|e| {
            tracing::error!("Failed to read file: {:?}", e);
            Error::Repository
        })?;

        Ok(bytes.to_vec())
    }

    async fn delete(&self, bucket: &str, key: &str) -> crate::common::Result<()> {
        let client = self.client.clone();

        client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to upload file: {:?}", e);
                Error::Repository
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Error;
    use crate::domain::repositories::file::{FileRepository, MockFileRepository};
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_upload_success() {
        let mut mock = MockFileRepository::default();

        mock.expect_upload().times(1).returning(|_, _, _| Ok(()));

        let result = mock.upload("test-bucket", "test.txt", b"test text").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_download_success() {
        let mut mock = MockFileRepository::default();

        mock.expect_download()
            .with(eq("test-bucket"), eq("test.txt"))
            .times(1)
            .returning(|_, _| Ok(b"test text".to_vec()));

        let result = mock.download("test-bucket", "test.txt").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"test text".to_vec());

        mock.expect_download()
            .with(eq("test-bucket"), eq("notfound.txt"))
            .times(1)
            .returning(|_, _| Err(Error::File));

        let result_failed = mock.download("test-bucket", "notfound.txt").await;
        assert!(result_failed.is_err());
    }

    #[tokio::test]
    async fn test_delete_success() {
        let mut mock = MockFileRepository::default();

        mock.expect_delete()
            .with(eq("test-bucket"), eq("test.txt"))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = mock.delete("test-bucket", "test.txt").await;
        assert!(result.is_ok());
    }
}
