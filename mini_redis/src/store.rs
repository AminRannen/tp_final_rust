
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};


#[derive(Debug, Clone)]
pub struct Entry {
    pub value: String,
    pub expires_at: Option<Instant>, 
}

impl Entry {
    pub fn new(value: String) -> Self {
        Entry { value, expires_at: None }
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(exp) => Instant::now() > exp,
            None => false, 
        }
    }

    pub fn set_expiry(&mut self, seconds: u64) {
        self.expires_at = Some(
            Instant::now() + Duration::from_secs(seconds)
        );
    }

    pub fn ttl(&self) -> i64 {
        match self.expires_at {
            None => -1, 
            Some(exp) => {
                let now = Instant::now();
                if now > exp {
                    -2 
                } else {
                    (exp - now).as_secs() as i64
                }
            }
        }
    }
}


#[derive(Clone)]
pub struct Store {
    inner: Arc<Mutex<HashMap<String, Entry>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut map = self.inner.lock().unwrap();
        map.insert(key, Entry::new(value));
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let map = self.inner.lock().unwrap();
        match map.get(key) {
            Some(entry) if !entry.is_expired() => Some(entry.value.clone()),
            _ => None, 
        }
    }

    pub fn del(&self, key: &str) -> u32 {
        let mut map = self.inner.lock().unwrap();
        if map.remove(key).is_some() { 1 } else { 0 }
    }

    pub fn keys(&self) -> Vec<String> {
        let map = self.inner.lock().unwrap();
        map.iter()
            .filter(|(_, entry)| !entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn expire(&self, key: &str, seconds: u64) -> bool {
        let mut map = self.inner.lock().unwrap();
        match map.get_mut(key) {
            Some(entry) => {
                entry.set_expiry(seconds);
                true 
            }
            None => false, 
        }
    }

    pub fn ttl(&self, key: &str) -> i64 {
        let map = self.inner.lock().unwrap();
        match map.get(key) {
            None => -2,           
            Some(entry) => entry.ttl(),
        }
    }

    pub fn incr(&self, key: &str) -> Result<i64, String> {
        let mut map = self.inner.lock().unwrap();
        let entry = map.entry(key.to_string())
            .or_insert(Entry::new("0".to_string()));

        match entry.value.parse::<i64>() {
            Ok(n) => {
                entry.value = (n + 1).to_string();
                Ok(n + 1)
            }
            Err(_) => Err("not an integer".to_string()),
        }
    }

    pub fn decr(&self, key: &str) -> Result<i64, String> {
        let mut map = self.inner.lock().unwrap();
        let entry = map.entry(key.to_string())
            .or_insert(Entry::new("0".to_string()));

        match entry.value.parse::<i64>() {
            Ok(n) => {
                entry.value = (n - 1).to_string();
                Ok(n - 1)
            }
            Err(_) => Err("not an integer".to_string()),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let map = self.inner.lock().unwrap();
        let data: HashMap<&String, &String> = map.iter()
            .map(|(k, v)| (k, &v.value))
            .collect();
        let json = serde_json::to_string(&data).unwrap();
        std::fs::write("dump.json", json)
            .map_err(|e| e.to_string())
    }

    pub fn cleanup_expired(&self) {
        let mut map = self.inner.lock().unwrap();
        map.retain(|_, entry| !entry.is_expired());
    }
}