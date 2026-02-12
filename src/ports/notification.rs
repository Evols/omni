use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct ValidationRequest {
    pub title: String,
    pub message: String,
}

#[async_trait]
pub trait NotificationPort: Send + Sync {
    async fn request_validation(&self, request: ValidationRequest) -> bool;
}
