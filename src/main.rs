use std::{env, io};
use std::fmt::Display;
use std::process::{Command, ExitStatus};
extern crate os_type;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use inquire::Select;
use inquire::Text;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest, MessageRole, Content};


// async fn ask_ai_question(
//     client: &OpenAIClient,
//     model: &str,
//     system_message: &str,
//     user_input: &str
// ) -> Result<Option<String>, Box<dyn std::error::Error>> {
//     let messages = vec![
//         ChatCompletionMessage {
//             role: MessageRole::system,
//             content: Content::Text(String::from(system_message)),
//             name: None,
//             tool_calls: None,
//             tool_call_id: None,
//         },
//         ChatCompletionMessage {
//             role: MessageRole::user,
//             content: Content::Text(user_input.to_string()),
//             name: None,
//             tool_calls: None,
//             tool_call_id: None,
//         },
//     ];
//     let req = ChatCompletionRequest::new(model.to_string(), messages);
//     let result = client.chat_completion(req).await?;
//     let content = result.choices[0].message.content.clone();
//     Ok(content)
// }

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Suggestion {
    pub command: String,
}
impl Suggestion {
    fn execute(&self) -> io::Result<ExitStatus> {
        let parts: Vec<&str> = self.command.split_whitespace().collect();
        if parts.is_empty() {
            println!("Invalid command");
            return Ok(ExitStatus::default());
        }
    
        let program = parts[0];
        let args = &parts[1..];
    
        Command::new(program)
        .args(args)
        .status()
    }
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
    if the user asks to delete a directory, respond with: ```command: rm -rf /path/to/directory # add additional commands here if danger!```.
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
    dotenv().ok();
    
    // Get OpenAI API key from environment
    let api_key = env::var("SHAI_API_KEY")
        .expect("SHAI_API_KEY must be set");
    let endpoint = env::var("SHAI_API_ENDPOINT").unwrap_or("https://openrouter.ai/api/v1".to_string());
    let model = env::var("SHAI_MODEL").unwrap_or("meta-llama/llama-3.1-70b-instruct:free".to_string());
    let suggestion_count: i32 = env::var("SHAI_SUGGESTION_COUNT")
        .unwrap_or("2".to_string())
        .parse()
        .unwrap_or(2);
    
    // Initialize OpenAI client
    let client = OpenAIClient::builder()
        .with_header("HTTP-Referer", "http://github.com/y1j2x34/shai")
        .with_header("X-Title", "SHAI")
        .with_endpoint(endpoint)
        .with_api_key(api_key)
        .build()?;

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: shai \"<command description>\"");
        return Ok(());
    }

    let user_input = &args[1];
    
    println!("user input: {} ", user_input);
    
    let mut suggestions: Vec<Suggestion> = Vec::new();
    for _ in 0..suggestion_count {
        let mut retry_count = 3;
        while retry_count > 0 {
            
            let result = get_command_suggestion(&client, model.as_str(), user_input).await?;
            
            let result = result.trim().trim_matches(&['`', '\n', '\r']).to_string();
            
            let command = result.replace("command: ", "").trim_start().to_string();
            
            if command.is_empty() {
                println!("Invalid command: {}", command);
                retry_count = retry_count - 1
            } else {
                retry_count = 0
            }
            
            let suggestion = Suggestion { command };
            
            suggestions.push(suggestion);
        }
    }
    
    let selection = Select::new(
        &format!("Suggested commands for: {}", user_input),
        suggestions,
    ).prompt()?;

    println!("Executing: {}", selection);
    
    let command = Text::new("")
        .with_help_message("Type to modify the suggested command, or press Enter to execute the command")
        .with_initial_value(&selection.command)
        .prompt()?;
    
    let suggestion = Suggestion { command };
    
    match suggestion.execute() {
        Ok(status) => {
            if status.success() {
                println!("Command executed successfully");
            } else {
                println!("Command failed with status: {}", status);
            }
        }
        Err(e) => {
            println!("Comand execution failed: {:?}", e);
        },
    }
    Ok(())
}
