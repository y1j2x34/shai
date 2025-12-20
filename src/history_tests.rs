#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // Helper function to create a test history with custom storage path
    fn setup_test_history() -> (History, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let history = History::new();
        
        // Override storage path for testing
        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
        
        (history, temp_dir)
    }

    #[test]
    fn test_add_command() {
        let (history, _temp_dir) = setup_test_history();
        
        let command = Command::new(
            "list files".to_string(),
            "ls -la".to_string(),
        );
        
        let result = history.add(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_commands() {
        let (history, _temp_dir) = setup_test_history();
        
        // Add some commands
        for i in 0..5 {
            let command = Command::new(
                format!("command {}", i),
                format!("echo {}", i),
            );
            history.add(command).unwrap();
        }
        
        let commands = history.list(Some(3)).unwrap();
        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_search_commands() {
        let (history, _temp_dir) = setup_test_history();
        
        // Add commands with different descriptions
        let command1 = Command::new("find docker images".to_string(), "docker images".to_string());
        let command2 = Command::new("list files".to_string(), "ls -la".to_string());
        let command3 = Command::new("remove docker container".to_string(), "docker rm".to_string());
        
        history.add(command1).unwrap();
        history.add(command2).unwrap();
        history.add(command3).unwrap();
        
        let results = history.search("docker").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_clear_history() {
        let (history, _temp_dir) = setup_test_history();
        
        // Add a command
        let command = Command::new("test".to_string(), "echo test".to_string());
        history.add(command).unwrap();
        
        // Clear history
        history.clear().unwrap();
        
        let commands = history.list(None).unwrap();
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_history_limit() {
        let (history, _temp_dir) = setup_test_history();
        
        // Add more than 1000 commands
        for i in 0..1100 {
            let command = Command::new(
                format!("command {}", i),
                format!("echo {}", i),
            );
            history.add(command).unwrap();
        }
        
        let commands = history.list(None).unwrap();
        assert!(commands.len() <= 1000);
    }
}

