use std::path::PathBuf;
use crate::command::Command;
use crate::storage::{Storage, get_data_dir};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HistoryData {
    pub commands: Vec<Command>,
}

impl Default for HistoryData {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

pub struct History;

impl History {
    pub fn new() -> Self {
        Self
    }

    pub fn add(&self, command: Command) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = self.load_data()?;
        data.commands.push(command);
        
        // Keep only last 1000 commands
        if data.commands.len() > 1000 {
            data.commands.drain(0..data.commands.len() - 1000);
        }
        
        self.save(&data)
    }

    pub fn list(&self, limit: Option<usize>) -> Result<Vec<Command>, Box<dyn std::error::Error>> {
        let data = self.load_data()?;
        let mut commands = data.commands;
        
        // Sort by timestamp, newest first
        commands.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(n) = limit {
            commands.truncate(n);
        }
        
        Ok(commands)
    }

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = HistoryData::default();
        self.save(&data)
    }

    pub fn search(&self, query: &str) -> Result<Vec<Command>, Box<dyn std::error::Error>> {
        let data = self.load_data()?;
        let query_lower = query.to_lowercase();
        
        let mut results: Vec<Command> = data.commands
            .into_iter()
            .filter(|cmd| {
                cmd.description.to_lowercase().contains(&query_lower)
                    || cmd.command.to_lowercase().contains(&query_lower)
            })
            .collect();
        
        // Sort by timestamp, newest first
        results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        Ok(results)
    }

    fn load_data(&self) -> Result<HistoryData, Box<dyn std::error::Error>> {
        match self.load::<HistoryData>() {
            Ok(data) => Ok(data),
            Err(_) => Ok(HistoryData::default()),
        }
    }
}

impl Storage for History {
    fn get_storage_path(&self) -> PathBuf {
        get_data_dir().join("history.json")
    }
}

