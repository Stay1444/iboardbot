use std::{collections::HashMap, sync::Arc};

use tokio::sync::{mpsc, oneshot, Mutex};
use tracing::info;

pub struct Session {
    pub id: String,
}

struct SessionActor {
    sessions: HashMap<String, Arc<Mutex<Session>>>,
    receiver: mpsc::Receiver<Message>,
}

impl SessionActor {
    pub fn new(recv: mpsc::Receiver<Message>) -> Self {
        Self {
            sessions: HashMap::default(),
            receiver: recv,
        }
    }

    pub async fn handle(&mut self, message: Message) {
        match message {
            Message::Get { respond_to, id } => {
                let session = self.sessions.get(&id);

                _ = respond_to.send(session.cloned());
            }
            Message::Delete { id } => {
                self.sessions.remove(&id);
            }
            Message::Create { respond_to, id } => {
                let session = Session { id: id.clone() };
                let session = Arc::new(Mutex::new(session));

                self.sessions.insert(id, session.clone());

                _ = respond_to.send(session);
            }
        }
    }
}

enum Message {
    Get {
        respond_to: oneshot::Sender<Option<Arc<Mutex<Session>>>>,
        id: String,
    },
    Delete {
        id: String,
    },
    Create {
        respond_to: oneshot::Sender<Arc<Mutex<Session>>>,
        id: String,
    },
}

#[derive(Clone)]
pub struct Sessions {
    sender: mpsc::Sender<Message>,
}

impl Sessions {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(4);

        let actor = SessionActor::new(rx);

        tokio::spawn(run(actor));

        Self { sender: tx }
    }

    pub async fn get(&self, id: String) -> Option<Arc<Mutex<Session>>> {
        let (tx, rx) = oneshot::channel();
        let msg = Message::Get { id, respond_to: tx };

        _ = self.sender.send(msg).await;

        rx.await.expect("Actor was closed")
    }

    pub async fn delete(&self, id: String) {
        let msg = Message::Delete { id };

        _ = self.sender.send(msg).await;
    }

    pub async fn create(&self, id: String) -> Arc<Mutex<Session>> {
        info!("Created session {id}");

        let (tx, rx) = oneshot::channel();
        let msg = Message::Create { id, respond_to: tx };

        _ = self.sender.send(msg).await;

        rx.await.expect("Actor was closed")
    }
}

async fn run(mut actor: SessionActor) {
    while let Some(message) = actor.receiver.recv().await {
        actor.handle(message).await;
    }
}
