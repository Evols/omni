use async_trait::async_trait;
use tokio::sync::{mpsc, oneshot};

use crate::adapters::iced_notification::daemon::{ResponseHandle, ValidationCommand};
use crate::ports::notification::{NotificationPort, ValidationRequest};

#[derive(Clone)]
pub struct IcedNotificationAdapter {
    pub sender: mpsc::UnboundedSender<ValidationCommand>,
}

impl IcedNotificationAdapter {
    pub async fn request_validation(&self, request: ValidationRequest) -> bool {
        let (respond_to, response) = oneshot::channel();
        let command = ValidationCommand {
            request,
            respond_to: ResponseHandle::new(respond_to),
        };

        if self.sender.send(command).is_err() {
            return false;
        }

        response.await.unwrap_or(false)
    }
}

#[async_trait]
impl NotificationPort for IcedNotificationAdapter {
    async fn request_validation(&self, request: ValidationRequest) -> bool {
        IcedNotificationAdapter::request_validation(self, request).await
    }
}
