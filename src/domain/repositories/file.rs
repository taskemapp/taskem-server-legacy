use crate::common::Result;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FileRepository: Send + Sync {
    async fn upload(&self, bucket: &str, key: &str, data: &[u8]) -> Result<()>;
    async fn download(&self, bucket: &str, key: &str) -> Result<Vec<u8>>;
    async fn delete(&self, bucket: &str, key: &str) -> Result<()>;
}
