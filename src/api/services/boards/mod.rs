use std::{collections::HashMap, path::PathBuf, time::Duration};

use bevy_math::Rect;
use chrono::{TimeDelta, Utc};
use lazy_static::lazy_static;
use rand::Rng;
use tokio::sync::{mpsc, oneshot};
use tracing::info;

use self::entities::{Board, BoardDetails, BoardState, Job, JobAction};

pub mod entities;

lazy_static! {
    static ref CONFIG_FOLDER: PathBuf = "boards".into();
}

struct Actor {
    receiver: mpsc::Receiver<Message>,
    boards: HashMap<String, Board>,
    job_callbacks: HashMap<String, Vec<oneshot::Sender<Job>>>,
    pending_jobs: HashMap<String, Vec<Job>>,
}

impl Actor {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
        Self {
            receiver,
            boards: HashMap::new(),
            job_callbacks: HashMap::new(),
            pending_jobs: HashMap::new(),
        }
    }
    pub async fn handle(&mut self, message: Message) {
        match message {
            Message::GetBoard { id, respond_to } => {
                let board = self.boards.get(&id);

                if let Some(board) = board {
                    let board = board.clone();
                    _ = respond_to.send(board);
                    return;
                }

                info!("Board {} connected", id);

                let details = load_detals(&id);
                let board = Board {
                    id: id.clone(),
                    state: BoardState::Unknown,
                    available: (
                        0.0,
                        0.0,
                        details.dimensions.width as f32,
                        details.dimensions.height as f32,
                    ),
                    last_update: Utc::now(),
                    details,
                };

                self.boards.insert(id, board.clone());

                _ = respond_to.send(board);
            }
            Message::SetBoardDetails { id, details } => {
                save_details(&id, &details);
            }
            Message::Cleanup => {
                let mut to_remove: Vec<String> = vec![];
                for (key, board) in &self.boards {
                    if board.state != BoardState::Disconnected
                        && board.last_update - Utc::now() > TimeDelta::seconds(30)
                    {
                        to_remove.push(key.clone());
                    }
                }

                for key in to_remove {
                    self.boards.remove(&key);
                }
            }
            Message::GetJob { id, respond_to } => {
                let Some(board) = self.boards.get_mut(&id) else {
                    return;
                };

                let jobs = self.pending_jobs.get_mut(&id);

                board.last_update = Utc::now();
                board.state = BoardState::Ready;

                let Some(jobs) = jobs else {
                    let callbacks = self.job_callbacks.get_mut(&id);

                    if let Some(callbacks) = callbacks {
                        callbacks.push(respond_to);
                    } else {
                        self.job_callbacks.insert(id, vec![respond_to]);
                    }
                    return;
                };

                if jobs.is_empty() {
                    let callbacks = self.job_callbacks.get_mut(&id);

                    if let Some(callbacks) = callbacks {
                        callbacks.push(respond_to);
                    } else {
                        self.job_callbacks.insert(id, vec![respond_to]);
                    }

                    return;
                }

                let job = jobs.remove(0);

                board.state = BoardState::Working(job.clone());

                _ = respond_to.send(job);
            }
            Message::JobAck { id, _job } => {
                let board = self.boards.get_mut(&id);
                if let Some(board) = board {
                    board.state = BoardState::Ready;
                    board.last_update = Utc::now();
                }
            }
            Message::AddJob { id, job } => {
                let callbacks = self.job_callbacks.remove(&id);

                if let Some(callbacks) = callbacks {
                    for i in callbacks {
                        _ = i.send(job.clone());
                    }
                    return;
                };

                let pending = self.pending_jobs.get_mut(&id);

                if let Some(pending) = pending {
                    pending.push(job);
                } else {
                    self.pending_jobs.insert(id, vec![job]);
                }
            }
            Message::List { respond_to } => {
                let mut res = vec![];
                for (key, board) in &mut self.boards {
                    let details = load_detals(key.as_str());
                    board.details = details;
                    res.push(board.clone());
                }

                _ = respond_to.send(res);
            }
            Message::ListPendingJobs { respond_to, id } => {
                let jobs = self.pending_jobs.get(&id).cloned().unwrap_or_default();
                _ = respond_to.send(jobs);
            }
            Message::ReportSpaceTaken { id, space } => {
                let Some(board) = self.boards.get_mut(&id) else {
                    return;
                };

                board.available.0 += space.max.x + 10.0; // padding
            }
            Message::ClearSpace { id } => {
                let Some(board) = self.boards.get_mut(&id) else {
                    return;
                };

                board.available.0 = 0.0;
            }
        }
    }
}

fn load_detals(id: &str) -> BoardDetails {
    if !CONFIG_FOLDER.exists() {
        std::fs::create_dir_all(CONFIG_FOLDER.clone()).unwrap();
    }

    let mut path = CONFIG_FOLDER.clone();
    path.push(format!("{}.yaml", id));

    if path.exists() {
        let yaml = std::fs::read_to_string(path).unwrap();
        return serde_yaml::from_str(&yaml).unwrap();
    } else {
        info!(
            "Created default board details for board {} in {}",
            id,
            path.display()
        );

        let details = BoardDetails::default();

        let yaml = serde_yaml::to_string(&details).unwrap();

        std::fs::write(path, yaml).unwrap();

        return details;
    }
}

fn save_details(id: &str, details: &BoardDetails) {
    let yaml = serde_yaml::to_string(&details).unwrap();

    let mut path = CONFIG_FOLDER.clone();
    path.push(format!("{}.yaml", id));

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
    GetJob {
        id: String,
        respond_to: oneshot::Sender<Job>,
    },
    JobAck {
        id: String,
        _job: u32,
    },
    AddJob {
        id: String,
        job: Job,
    },
    List {
        respond_to: oneshot::Sender<Vec<Board>>,
    },
    ListPendingJobs {
        id: String,
        respond_to: oneshot::Sender<Vec<Job>>,
    },
    ReportSpaceTaken {
        id: String,
        space: Rect,
    },
    ClearSpace {
        id: String,
    },
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

    pub async fn list(&self) -> Vec<Board> {
        let (tx, rx) = oneshot::channel();

        _ = self.sender.send(Message::List { respond_to: tx }).await;

        rx.await.expect("Actor closed")
    }

    pub async fn list_pending_jobs(&self, id: impl Into<String>) -> Vec<Job> {
        let (tx, rx) = oneshot::channel();

        _ = self
            .sender
            .send(Message::ListPendingJobs {
                respond_to: tx,
                id: id.into(),
            })
            .await;

        rx.await.expect("Actor closed")
    }

    pub async fn report_space_taken(&self, id: impl Into<String>, space: Rect) {
        let msg = Message::ReportSpaceTaken {
            id: id.into(),
            space,
        };
        _ = self.sender.send(msg).await;
    }

    pub async fn clear_space(&self, id: impl Into<String>) {
        let msg = Message::ClearSpace { id: id.into() };
        _ = self.sender.send(msg).await;
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

    pub async fn get_job(&self, id: impl Into<String>) -> Job {
        let (tx, rx) = oneshot::channel();
        let id = id.into();

        let msg = Message::GetJob {
            id: id.clone(),
            respond_to: tx,
        };

        _ = self.sender.send(msg).await;

        let job = rx.await.expect("Actor disconnected");

        job
    }

    pub async fn add_job(&self, id: impl Into<String>, action: JobAction) -> Job {
        let job_id: u32 = rand::thread_rng().gen_range(1..3000);

        let job = Job { action, id: job_id };

        _ = self
            .sender
            .send(Message::AddJob {
                id: id.into(),
                job: job.clone(),
            })
            .await;

        job
    }

    pub async fn ack_job(&self, id: impl Into<String>, job: u32) {
        _ = self
            .sender
            .send(Message::JobAck {
                id: id.into(),
                _job: job,
            })
            .await;
    }
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
