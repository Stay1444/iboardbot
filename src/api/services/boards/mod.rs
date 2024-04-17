use std::{collections::HashMap, path::PathBuf, time::Duration};

use chrono::{DateTime, TimeDelta, Utc};
use lazy_static::lazy_static;
use tokio::sync::{mpsc, oneshot};

use self::entities::{Board, BoardDetails, BoardState};

pub mod entities;

lazy_static! {
    static ref CONFIG_FOLDER: PathBuf = "boards".into();
}

struct Actor {
    receiver: mpsc::Receiver<Message>,
    board_states: HashMap<String, (BoardState, DateTime<Utc>)>,
}

impl Actor {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
        Self {
            receiver,
            board_states: HashMap::new(),
        }
    }
    pub async fn handle(&mut self, message: Message) {
        match message {
            Message::GetBoard { id, respond_to } => {
                let state = match self.board_states.get(&id) {
                    Some(state) => state.0,
                    None => BoardState::Disconnected,
                };

                let board = Board {
                    id: id.clone(),
                    state,
                    details: load_detals(&id),
                };

                _ = respond_to.send(board);
            }
            Message::SetBoardDetails { id, details } => {
                save_details(&id, &details);
            }
            Message::Cleanup => {
                let mut to_remove: Vec<String> = vec![];
                for (key, (state, last_update)) in &self.board_states {
                    if *state != BoardState::Disconnected
                        && *last_update - Utc::now() > TimeDelta::seconds(30)
                    {
                        to_remove.push(key.clone());
                    }
                }

                for key in to_remove {
                    self.board_states.remove(&key);
                }
            }
        }
    }
}

fn load_detals(id: &str) -> BoardDetails {
    if !CONFIG_FOLDER.exists() {
        std::fs::create_dir_all(CONFIG_FOLDER.clone()).unwrap();
    }

    let mut path = CONFIG_FOLDER.clone();
    path.push(format!("/{}.yaml", id));

    if path.exists() {
        let yaml = std::fs::read_to_string(path).unwrap();
        return serde_yaml::from_str(&yaml).unwrap();
    } else {
        let details = BoardDetails::default();

        let yaml = serde_yaml::to_string(&details).unwrap();

        std::fs::write(path, yaml).unwrap();

        return details;
    }
}

fn save_details(id: &str, details: &BoardDetails) {
    let yaml = serde_yaml::to_string(&details).unwrap();

    let mut path = CONFIG_FOLDER.clone();
    path.push(format!("/{}.yaml", id));

    std::fs::write(path, yaml).unwrap();
}

enum Message {
    GetBoard {
        id: String,
        respond_to: oneshot::Sender<Board>,
    },
    SetBoardDetails {
        id: String,
        details: BoardDetails,
    },
    Cleanup,
}

#[derive(Clone)]
pub struct Boards {
    sender: mpsc::Sender<Message>,
}

impl Boards {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(4);

        let actor = Actor::new(rx);

        tokio::spawn(run(actor));
        tokio::spawn(cleanup(tx.clone()));

        Self { sender: tx }
    }

    pub async fn get(&self, id: impl Into<String>) -> Board {
        let (tx, rx) = oneshot::channel();
        let msg = Message::GetBoard {
            id: id.into(),
            respond_to: tx,
        };
        _ = self.sender.send(msg).await;

        rx.await.expect("Actor closed")
    }

    pub async fn set_details(&self, id: impl Into<String>, details: BoardDetails) {
        let msg = Message::SetBoardDetails {
            id: id.into(),
            details,
        };

        _ = self.sender.send(msg).await;
    }

    pub async fn pop_job(&self, id: impl Into<String>) -> Option<()> {
        None
    }

    pub async fn ack_job(&self, id: u32) {}
}

async fn run(mut actor: Actor) {
    while let Some(message) = actor.receiver.recv().await {
        actor.handle(message).await;
    }
}
async fn cleanup(tx: mpsc::Sender<Message>) {
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
        if let Err(_) = tx.send(Message::Cleanup).await {
            break;
        }
    }
}
