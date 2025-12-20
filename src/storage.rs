use std::fs;
use std::path::PathBuf;
use serde::{Serialize, de::DeserializeOwned};

pub trait Storage {
    fn get_storage_path(&self) -> PathBuf;
    
    fn ensure_storage_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.get_storage_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }
    
    fn load<T: DeserializeOwned>(&self) -> Result<T, Box<dyn std::error::Error>> {
        let path = self.get_storage_path();
        let content = fs::read_to_string(path)?;
        let data = serde_json::from_str(&content)?;
        Ok(data)
    }
    
    fn save<T: Serialize>(&self, data: &T) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_storage_dir()?;
        let path = self.get_storage_path();
        let content = serde_json::to_string_pretty(data)?;
        fs::write(path, content)?;
        Ok(())
    }
}

pub fn get_data_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    
    PathBuf::from(home).join(".shai")
}

