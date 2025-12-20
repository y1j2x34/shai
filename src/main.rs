use clap::{Parser, Subcommand};
use inquire::{Select, Text};
use openai_api_rs::v1::api::OpenAIClient;
use shai::{Config, History, Suggestion, Command, get_command_suggestion};

#[derive(Parser)]
#[command(name = "shai")]
#[command(about = "Shell AI Assistant - Convert natural language to shell commands", long_about = None)]
struct Cli {
    /// Command description for AI to convert
    #[arg(value_name = "DESCRIPTION")]
    description: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

impl Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)
    }
}


async fn get_command_suggestion(
    client: &OpenAIClient,
    model: &str,
    user_input: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let os=  os_type::current_platform();
    
    let platform_info = format!("The system the shell command wil be executed on is {:?} {}", os.os_type, os.version);
    
    let system_message = format!(r#"
    You are an expert at using shell commands.
    I need you to provide a response in the format: ```command: your_shell_command_here```. 
    {} 
    Only provide a single executable ling of shell code as the value for the \"command\" key. Never output any text and code block outside the JSON structure.
    The command wil be directly executed in a shell.
    For example: 
    if the user asks to install Rust, respond with: ```command: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```;
    if the user asks to delete a directory, respond with: ```command: rm -rf /path/to/directory # add additional comments here if danger!```.
    "#, platform_info);
    
    let user_message = format!("Here's what I'm trying to do: {}", user_input.to_string());
    
    let messages = vec![
        chat_completion::ChatCompletionMessage {
            role: MessageRole::system,
            content: Content::Text(String::from(system_message)),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        chat_completion::ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text(user_message),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
    ];

    let req = ChatCompletionRequest::new(model.to_string(), messages);
    let result = client.chat_completion(req).await?;
    
    let suggestion = result.choices[0].message.content.clone().unwrap_or_default();
    Ok(suggestion)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        match command {
            Commands::History { limit, search, clear } => {
                return handle_history(limit, search, clear);
            }
        }
    }

    // Handle main command suggestion flow
    if let Some(description) = cli.description {
        handle_suggest(&description).await?;
    } else {
        println!("Usage: shai \"<command description>\"");
        println!("       shai history [OPTIONS]");
        println!("\nRun 'shai --help' for more information.");
    }

    Ok(())
}

async fn handle_suggest(user_input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    
    // Initialize OpenAI client
    let client = OpenAIClient::builder()
        .with_header("HTTP-Referer", "http://github.com/y1j2x34/shai")
        .with_header("X-Title", "SHAI")
        .with_endpoint(&config.endpoint)
        .with_api_key(&config.api_key)
        .build()?;

    let mut suggestions: Vec<Suggestion> = Vec::new();
    
    for _ in 0..config.suggestion_count {
        let mut retry_count = 3;
        while retry_count > 0 {
            let result = get_command_suggestion(&client, &config.model, user_input).await?;
            
            let result = result.trim().trim_matches(&['`', '\n', '\r']).to_string();
            let command = result.replace("command: ", "").trim_start().to_string();
            
            if command.is_empty() {
                println!("Invalid command: {}", command);
                retry_count -= 1;
            } else {
                let suggestion = Suggestion::new(command);
                suggestions.push(suggestion);
                retry_count = 0;
            }
        }
    }
    
    if suggestions.is_empty() {
        println!("Failed to generate command suggestions.");
        return Ok(());
    }

    let selection = Select::new(
        &format!("Suggested commands for: {}", user_input),
        suggestions.clone(),
    ).prompt()?;
    
    let command = Text::new("")
        .with_help_message("Type to modify the suggested command, or press Enter to execute")
        .with_initial_value(&selection.command)
        .prompt()?;
    
    let final_suggestion = Suggestion::new(command.clone());
    
    // Save to history before execution
    let history = History::new();
    let history_entry = Command::new(user_input.to_string(), command);
    history.add(history_entry)?;
    
    // Execute the command
    final_suggestion.execute()?;
    
    Ok(())
}

fn handle_history(limit: usize, search: Option<String>, clear: bool) -> Result<(), Box<dyn std::error::Error>> {
    let history = History::new();
    
    if clear {
        history.clear()?;
        println!("History cleared.");
        return Ok(());
    }
    
    let commands = if let Some(query) = search {
        history.search(&query)?
    } else {
        history.list(Some(limit))?
    };
    
    if commands.is_empty() {
        println!("No history found.");
        return Ok(());
    }
    
    println!("\n{:<20} {:<40} {}", "Time", "Description", "Command");
    println!("{}", "-".repeat(100));
    
    for cmd in commands {
        let datetime = chrono::DateTime::from_timestamp(cmd.timestamp, 0)
            .unwrap_or_default()
            .format("%Y-%m-%d %H:%M:%S");
        
        let desc = if cmd.description.len() > 37 {
            format!("{}...", &cmd.description[..37])
        } else {
            cmd.description.clone()
        };
        
        let command_display = if cmd.command.len() > 37 {
            format!("{}...", &cmd.command[..37])
        } else {
            cmd.command.clone()
        };
        
        println!("{:<20} {:<40} {}", datetime, desc, command_display);
    }
    
    Ok(())
}
