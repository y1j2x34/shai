use std::env;
use std::env::current_dir;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub endpoint: String,
    pub model: String,
    pub suggestion_count: i32,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // First load .env from home directory (global config)
        if let Some(home) = env::var_os("HOME") {
            let env_path = std::path::PathBuf::from(home).join(".env");
            let _ = dotenvy::from_path(&env_path);
        }
        // Then load .env from current directory (can override global config)
        if let Ok(current_dir) = current_dir() {
            let _ = dotenvy::from_path_override(current_dir.join(".env"));
        }
        
        let api_key = env::var("SHAI_API_KEY")
            .expect("SHAI_API_KEY must be set");
        let endpoint = env::var("SHAI_API_ENDPOINT")
            .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());
        let model = env::var("SHAI_MODEL")
            .unwrap_or_else(|_| "meta-llama/llama-3.1-70b-instruct:free".to_string());
        let suggestion_count: i32 = env::var("SHAI_SUGGESTION_COUNT")
            .unwrap_or_else(|_| "2".to_string())
            .parse()
            .unwrap_or(2);

        Ok(Self {
            api_key,
            endpoint,
            model,
            suggestion_count,
        })
    }
}

