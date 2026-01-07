use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{sync::RwLock, time::Instant};

#[derive(Clone)]
pub struct Database {
    db: Arc<RwLock<HashMap<String, String>>>,
    expiry: Arc<RwLock<HashMap<String, Instant>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            db: Arc::new(RwLock::new(HashMap::new())),
            expiry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, key: String, value: String, ttl: Option<u64>) {
        let mut db_map = self.db.write().await;
        db_map.insert(key.clone(), value);

        if let Some(sec) = ttl {
            let exp_time = Instant::now() + Duration::from_secs(sec);
            let mut exp_map = self.expiry.write().await;
            exp_map.insert(key.clone(), exp_time);
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        if self.is_expired(key).await {
            self.delete(key).await;
            return None;
        }
        let db_map = self.db.read().await;
        db_map.get(key).cloned()
    }
    pub async fn is_expired(&self, key: &str) -> bool {
        let exp_map = self.expiry.read().await;
        if let Some(&exp_time) = exp_map.get(key) {
            return Instant::now() > exp_time;
        }
        return false;
    }
    pub async fn delete(&self, key: &str) -> bool {
        let mut db_map = self.db.write().await;
        let mut exp_map = self.expiry.write().await;

        exp_map.remove(key);
        return db_map.remove(key).is_some();
    }
}
