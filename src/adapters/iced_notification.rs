use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use iced::futures::{SinkExt, StreamExt};
use iced::widget::{button, column, container, text};
use iced::{daemon, window, Element, Fill, Result, Subscription, Task};
use iced::stream;
use tokio::sync::{mpsc, oneshot};

use crate::ports::notification::{NotificationPort, ValidationRequest};

#[derive(Clone)]
struct ValidationCommand {
    request: ValidationRequest,
    respond_to: ResponseHandle,
}

#[derive(Clone)]
struct PendingRequest {
    request: ValidationRequest,
    respond_to: ResponseHandle,
}

#[derive(Clone)]
pub struct IcedNotificationAdapter {
    sender: mpsc::UnboundedSender<ValidationCommand>,
}

pub struct NotificationDaemon {
    receiver: ReceiverHandle,
}

pub fn channel() -> (IcedNotificationAdapter, NotificationDaemon) {
    let (sender, receiver) = mpsc::unbounded_channel();
    let adapter = IcedNotificationAdapter { sender };
    let daemon = NotificationDaemon {
        receiver: ReceiverHandle::new(receiver),
    };

    (adapter, daemon)
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

impl NotificationDaemon {
    pub fn run(self) -> Result {
        let receiver = self.receiver;

        daemon(move || boot(receiver.clone()), update, view)
            .subscription(subscription)
            .title(|state: &RuntimeState, _| {
                state
                    .current
                    .as_ref()
                    .map(|pending| pending.request.title.clone())
                    .unwrap_or_else(|| "Validation".to_string())
            })
            .run()
    }
}

#[derive(Clone)]
enum RuntimeMessage {
    Incoming(ValidationCommand),
    Approved,
    Rejected,
}

struct RuntimeState {
    receiver: ReceiverHandle,
    current: Option<PendingRequest>,
    queue: VecDeque<PendingRequest>,
    window_id: Option<window::Id>,
}

fn boot(receiver: ReceiverHandle) -> (RuntimeState, Task<RuntimeMessage>) {
    (
        RuntimeState {
            receiver,
            current: None,
            queue: VecDeque::new(),
            window_id: None,
        },
        Task::none(),
    )
}

fn update(
    state: &mut RuntimeState,
    message: RuntimeMessage,
) -> Task<RuntimeMessage> {
    match message {
        RuntimeMessage::Incoming(command) => {
            if state.current.is_none() {
                state.current = Some(PendingRequest {
                    request: command.request,
                    respond_to: command.respond_to,
                });

                if state.window_id.is_none() {
                    let (id, task) = open_window();
                    state.window_id = Some(id);
                    return task;
                }
            } else {
                state.queue.push_back(PendingRequest {
                    request: command.request,
                    respond_to: command.respond_to,
                });
            }

            Task::none()
        }
        RuntimeMessage::Approved => handle_decision(state, true),
        RuntimeMessage::Rejected => handle_decision(state, false),
    }
}

fn view(
    state: &RuntimeState,
    _window: window::Id,
) -> Element<'_, RuntimeMessage> {
    let content = if let Some(pending) = &state.current {
        column![
            text(&pending.request.message),
            button("Approve").on_press(RuntimeMessage::Approved),
            button("Reject").on_press(RuntimeMessage::Rejected),
        ]
        .spacing(12)
    } else {
        column![text("No pending validation.")]
    };

    container(content)
        .padding(20)
        .center_x(Fill)
        .center_y(Fill)
        .into()
}

fn subscription(state: &RuntimeState) -> Subscription<RuntimeMessage> {
    Subscription::batch(vec![
        channel_subscription(state.receiver.clone()),
        window::close_requests().map(|_| RuntimeMessage::Rejected),
    ])
}

fn channel_subscription(
    receiver: ReceiverHandle,
) -> Subscription<RuntimeMessage> {
    Subscription::run_with(receiver, receive_requests)
}

fn receive_requests(
    receiver: &ReceiverHandle,
) -> iced::futures::stream::BoxStream<'static, RuntimeMessage> {
    let receiver = receiver.take();

    let stream = stream::channel(100, move |mut output: iced::futures::channel::mpsc::Sender<RuntimeMessage>| async move {
        let Some(mut receiver) = receiver else {
            return;
        };

        while let Some(command) = receiver.recv().await {
            if output
                .send(RuntimeMessage::Incoming(command))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    stream.boxed()
}

fn handle_decision(
    state: &mut RuntimeState,
    approved: bool,
) -> Task<RuntimeMessage> {
    if let Some(current) = state.current.take() {
        current.respond_to.respond(approved);
    }

    if let Some(next) = state.queue.pop_front() {
        state.current = Some(next);
        Task::none()
    } else if let Some(id) = state.window_id.take() {
        window::close::<RuntimeMessage>(id)
    } else {
        Task::none()
    }
}

fn open_window() -> (window::Id, Task<RuntimeMessage>) {
    let mut settings = window::Settings::default();
    settings.exit_on_close_request = false;

    let (id, task) = window::open(settings);
    (id, task.discard())
}

#[derive(Clone)]
struct ReceiverHandle {
    id: u64,
    receiver: Arc<Mutex<Option<mpsc::UnboundedReceiver<ValidationCommand>>>>,
}

impl ReceiverHandle {
    fn new(receiver: mpsc::UnboundedReceiver<ValidationCommand>) -> Self {
        Self {
            id: rand_id(),
            receiver: Arc::new(Mutex::new(Some(receiver))),
        }
    }

    fn take(&self) -> Option<mpsc::UnboundedReceiver<ValidationCommand>> {
        let mut guard = self.receiver.lock().ok()?;
        guard.take()
    }
}

impl Hash for ReceiverHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn rand_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

#[derive(Clone)]
struct ResponseHandle {
    sender: Arc<Mutex<Option<oneshot::Sender<bool>>>>,
}

impl ResponseHandle {
    fn new(sender: oneshot::Sender<bool>) -> Self {
        Self {
            sender: Arc::new(Mutex::new(Some(sender))),
        }
    }

    fn respond(&self, approved: bool) {
        if let Ok(mut guard) = self.sender.lock() {
            if let Some(sender) = guard.take() {
                let _ = sender.send(approved);
            }
        }
    }
}
