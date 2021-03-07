use crate::database::models::UserModel;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::ws::Message;

#[derive(Clone)]
pub struct Storage {
    pub connections: HashMap<Uuid, mpsc::UnboundedSender<Result<Message, warp::Error>>>,
    pub me: UserModel,
}

pub type StorageType = Arc<RwLock<Storage>>;

impl Storage {
    pub fn default() -> StorageType {
        Arc::new(RwLock::new(Self {
            connections: HashMap::new(),
            me: UserModel::default(),
        }))
    }
}
