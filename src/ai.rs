use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest, MessageRole, Content};
use os_type;

pub async fn get_command_suggestion(
    client: &OpenAIClient,
    model: &str,
    user_input: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let os = os_type::current_platform();
    
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
    
    let user_message = format!("Here's what I'm trying to do: {}", user_input);
    
    let messages = vec![
        chat_completion::ChatCompletionMessage {
            role: MessageRole::system,
            content: Content::Text(system_message),
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

