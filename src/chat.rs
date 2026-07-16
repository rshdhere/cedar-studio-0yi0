use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

const HISTORY_LIMIT: usize = 100;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatEvent {
    Message {
        id: u64,
        user: String,
        text: String,
        ts: u64,
    },
    Join {
        user: String,
        ts: u64,
    },
    Leave {
        user: String,
        ts: u64,
    },
    History {
        messages: Vec<ChatEvent>,
    },
    Error {
        message: String,
    },
}

#[derive(Clone)]
pub struct ChatState {
    tx: broadcast::Sender<String>,
    history: Arc<Mutex<VecDeque<String>>>,
    next_id: Arc<Mutex<u64>>,
}

impl ChatState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(256);
        Self {
            tx,
            history: Arc::new(Mutex::new(VecDeque::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    fn push_history(&self, payload: String) {
        let mut history = self.history.lock().expect("history lock");
        history.push_back(payload);
        while history.len() > HISTORY_LIMIT {
            history.pop_front();
        }
    }

    fn next_message_id(&self) -> u64 {
        let mut id = self.next_id.lock().expect("id lock");
        let current = *id;
        *id += 1;
        current
    }

    fn publish(&self, event: ChatEvent) {
        if let Ok(payload) = serde_json::to_string(&event) {
            self.push_history(payload.clone());
            let _ = self.tx.send(payload);
        }
    }

    pub async fn ws_handler(
        State(state): State<ChatState>,
        ws: WebSocketUpgrade,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| handle_socket(state, socket))
    }
}

async fn handle_socket(state: ChatState, socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let history: Vec<ChatEvent> = state
        .history
        .lock()
        .expect("history lock")
        .iter()
        .filter_map(|raw| serde_json::from_str(raw).ok())
        .filter(|event| matches!(event, ChatEvent::Message { .. }))
        .collect();

    let history_event = ChatEvent::History { messages: history };
    if let Ok(payload) = serde_json::to_string(&history_event) {
        if sender.send(Message::Text(payload)).await.is_err() {
            return;
        }
    }

    let mut username: Option<String> = None;

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Text(text))) => {
                        if let Err(message) = handle_client_message(&state, &mut username, &text).await {
                            let err = ChatEvent::Error { message };
                            if let Ok(payload) = serde_json::to_string(&err) {
                                let _ = sender.send(Message::Text(payload)).await;
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            broadcast = rx.recv() => {
                match broadcast {
                    Ok(payload) => {
                        if sender.send(Message::Text(payload)).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }

    if let Some(user) = username {
        state.publish(ChatEvent::Leave {
            user,
            ts: now_millis(),
        });
    }
}

async fn handle_client_message(
    state: &ChatState,
    username: &mut Option<String>,
    text: &str,
) -> Result<(), String> {
    let incoming: ClientMessage =
        serde_json::from_str(text).map_err(|_| "invalid message format".to_string())?;

    match incoming {
        ClientMessage::Join { user } => {
            let user = sanitize_username(&user)?;
            if username.is_some() {
                return Err("already joined".to_string());
            }
            *username = Some(user.clone());
            state.publish(ChatEvent::Join {
                user,
                ts: now_millis(),
            });
        }
        ClientMessage::Chat { text } => {
            let user = username
                .clone()
                .ok_or_else(|| "join before sending messages".to_string())?;
            let text = sanitize_message(&text)?;
            state.publish(ChatEvent::Message {
                id: state.next_message_id(),
                user,
                text,
                ts: now_millis(),
            });
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMessage {
    Join { user: String },
    Chat { text: String },
}

fn sanitize_username(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("username cannot be empty".to_string());
    }
    if trimmed.len() > 24 {
        return Err("username must be 24 characters or fewer".to_string());
    }
    if trimmed.chars().any(|c| c.is_control()) {
        return Err("username contains invalid characters".to_string());
    }
    Ok(trimmed.to_string())
}

fn sanitize_message(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("message cannot be empty".to_string());
    }
    if trimmed.len() > 2000 {
        return Err("message must be 2000 characters or fewer".to_string());
    }
    Ok(trimmed.to_string())
}

fn now_millis() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
