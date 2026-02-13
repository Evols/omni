use crate::ports::database::DatabasePort;
use crate::ports::notification::NotificationPort;
use std::sync::Arc;

pub mod database;
pub mod notification;

#[derive(Clone)]
pub struct Ports {
    pub notification_port: Arc<dyn NotificationPort>,
    pub database_port: Arc<dyn DatabasePort>,
}
