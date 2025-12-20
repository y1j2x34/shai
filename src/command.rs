use std::fmt::Display;
use cmd_lib::run_cmd;

#[derive(Debug, Clone, Default)]
pub struct Suggestion {
    pub command: String,
}

impl Suggestion {
    pub fn new(command: String) -> Self {
        Self { command }
    }

    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let command = self.command.as_str();
        if cfg!(windows) {
            run_cmd!(cmd.exe /C "$command")?;
        } else {
            run_cmd!(bash -c "$command")?;
        }
        Ok(())
    }
}

impl Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Command {
    pub description: String,
    pub command: String,
    pub timestamp: i64,
    pub executed: bool,
}

impl Command {
    pub fn new(description: String, command: String) -> Self {
        Self {
            description,
            command,
            timestamp: chrono::Utc::now().timestamp(),
            executed: false,
        }
    }

    pub fn with_executed(mut self, executed: bool) -> Self {
        self.executed = executed;
        self
    }
}

