use aws_sdk_s3::primitives::ByteStream;

use crate::domain::error::Result;

pub trait FileRepository: Send + Sync {
    async fn upload(&self, bucket: &str, key: &str, data: ByteStream) -> Result<()>;
    async fn download(&self, bucket: &str, key: &str) -> Result<ByteStream>;
    async fn delete(&self, bucket: &str, key: &str) -> Result<()>;
}
