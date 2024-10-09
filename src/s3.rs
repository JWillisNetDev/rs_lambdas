use async_trait::async_trait;
use lambda_http::tracing;
use aws_sdk_s3::primitives::ByteStream;

pub use aws_sdk_s3::Client as S3Client;


#[async_trait]
pub trait PutFile {
    async fn put_file(&self, bucket: &str, key: &str, content_type: &str, content_encoding: &str, bytes: Vec<u8>) -> Result<(), String>;
}

#[async_trait]
impl PutFile for S3Client {
    async fn put_file(&self, bucket: &str, key: &str, content_type: &str, content_encoding: &str, bytes: Vec<u8>) -> Result<(), String> {
        tracing::info!("put file: bucket={}, key={}", bucket, key);
        let bytes = ByteStream::from(bytes);
        self.put_object()
            .bucket(bucket)
            .key(key)
            .body(bytes)
            .content_type(content_type)
            .content_encoding(content_encoding)
            .send()
            .await
            .map_err(|e| format!("put error: {}", e.to_string()))?;
        Ok(())
    }
}