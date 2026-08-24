use serde::{Serialize, de::DeserializeOwned};
use serde_json::{from_str, to_string};
use web_sys::Storage;

pub struct StorageService {
    storage: Storage,
    key: String,
}

impl StorageService {
    pub fn new(key: &str) -> Self {
        let storage = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap();
        
        Self {
            storage,
            key: key.to_string(),
        }
    }
    
    pub fn save<T: Serialize>(&self, value: &T) -> Result<(), String> {
        let json = to_string(value).map_err(|e| e.to_string())?;
        self.storage.set_item(&self.key, &json).map_err(|e| e.as_string().unwrap())
    }

    pub fn load<T: DeserializeOwned>(&self) -> Option<T> {
        let serialized = self.storage.get_item(&self.key).ok()??;
        let value: T = from_str(&serialized).ok()?;
        Some(value)
    }

    pub fn remove(&self) {
        let _ = self.storage.remove_item(&self.key);
    }

    pub fn exists(&self) -> bool {
        self.storage.get_item(&self.key).ok().flatten().is_some()
    }
}