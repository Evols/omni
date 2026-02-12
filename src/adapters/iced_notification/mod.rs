mod adapter;
mod daemon;
mod rng;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use daemon::{ReceiverHandle, ValidationCommand};

pub use adapter::IcedNotificationAdapter;
pub use daemon::NotificationDaemon;

pub fn new_iced_notification_adapter(
    (sender, receiver): (
        UnboundedSender<ValidationCommand>,
        UnboundedReceiver<ValidationCommand>,
    ),
) -> (IcedNotificationAdapter, NotificationDaemon) {
    let adapter = IcedNotificationAdapter { sender };
    let daemon = NotificationDaemon {
        receiver: ReceiverHandle::new(receiver),
    };

    (adapter, daemon)
}
