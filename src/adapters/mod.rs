mod iced_notification;

use std::sync::Arc;
use tokio::sync::mpsc;

use crate::ports::Ports;
use iced_notification::{NotificationDaemon, new_iced_notification_adapter};

pub fn adapters_channel() -> (Ports, NotificationDaemon) {
    let (sender, receiver) = mpsc::unbounded_channel();

    let (notification_adapter, daemon) = new_iced_notification_adapter((sender, receiver));

    (
        Ports {
            notification_port: Arc::new(notification_adapter),
        },
        daemon,
    )
}
