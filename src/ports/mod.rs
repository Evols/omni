use std::sync::Arc;

use crate::ports::notification::NotificationPort;

mod database;
pub mod notification;

#[derive(Clone)]
pub struct Ports {
    pub notification_port: Arc<dyn NotificationPort>,
}
