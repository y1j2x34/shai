use clap::{Parser, Subcommand};
use inquire::{Select, Text};
use openai_api_rs::v1::api::OpenAIClient;
use shai::{Config, History, Suggestion, Command, get_command_suggestion, Bookmark, BookmarkItem, generate_bookmark_info};

#[derive(Parser)]
#[command(name = "shai")]
#[command(about = "Shell AI Assistant - Convert natural language to shell commands", long_about = None)]
struct Cli {
    /// Command description for AI to convert
    #[arg(value_name = "DESCRIPTION")]
    description: Option<String>,

    /// Enable verbose output (shows endpoint, model, etc.)
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Subcommand)]
enum CliCommand {
    /// View and manage command history
    History {
        /// Number of recent commands to show
        #[arg(short, long, default_value = "2")]
        limit: usize,
        
        /// Search for commands containing this text
        #[arg(short, long)]
        search: Option<String>,
        
        /// Clear all history
        #[arg(short, long)]
        clear: bool,
    },
    /// Manage command bookmarks
    Bookmark {
        #[command(subcommand)]
        action: BookmarkAction,
    },
}

#[derive(Subcommand)]
enum BookmarkAction {
    /// Add a new bookmark
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        command: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    /// List all bookmarks
    List {
        #[arg(short, long)]
        tag: Option<String>,
    },
    /// Get a specific bookmark
    Get {
        name: String,
    },
    /// Remove a bookmark
    Remove {
        name: String,
    },
    /// Search bookmarks
    Search {
        query: String,
    },
    /// Smart save: Save last command as bookmark with AI-generated metadata
    Save,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        match command {
            CliCommand::History { limit, search, clear } => {
                return handle_history(limit, search, clear);
            }
            CliCommand::Bookmark { action } => {
                return handle_bookmark(action, cli.verbose).await;
            }
        }
    }

    // Handle main command suggestion flow
    if let Some(description) = cli.description {
        handle_suggest(&description, cli.verbose).await?;
    } else {
        println!("Usage: shai \"<command description>\"");
        println!("       shai history [OPTIONS]");
        println!("       shai bookmark [SUBCOMMAND]");
        println!("\nRun 'shai --help' for more information.");
    }

    Ok(())
}

async fn handle_suggest(user_input: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    
    // Output verbose information if requested
    if verbose {
        println!("=== Verbose Mode ===");
        println!("Endpoint: {}", config.endpoint);
        println!("Model: {}", config.model);
        println!("Suggestion count: {}", config.suggestion_count);
        println!("User input: {}", user_input);
        println!("===================\n");
    }
    
    // Initialize OpenAI client
    let client = OpenAIClient::builder()
        .with_header("HTTP-Referer", "http://github.com/y1j2x34/shai")
        .with_header("X-Title", "SHAI")
        .with_endpoint(&config.endpoint)
        .with_api_key(&config.api_key)
        .build()?;

    let mut suggestions: Vec<Suggestion> = Vec::new();
    
    for i in 0..config.suggestion_count {
        if verbose {
            println!("Requesting suggestion {} of {}...", i + 1, config.suggestion_count);
        }
        
        let mut retry_count = 3;
        while retry_count > 0 {
            let result = get_command_suggestion(&client, &config.model, user_input).await?;
            
            if verbose {
                println!("Raw AI response: {}", result);
            }
            
            let result = result.trim().trim_matches(&['`', '\n', '\r']).to_string();
            let command = result.replace("command: ", "").trim_start().to_string();
            
            if command.is_empty() {
                println!("Invalid command: {}", command);
                retry_count -= 1;
                if verbose {
                    println!("Retrying... ({} attempts left)", retry_count);
                }
            } else {
                let suggestion = Suggestion::new(command);
                suggestions.push(suggestion);
                if verbose {
                    println!("✓ Suggestion generated successfully\n");
                }
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
    
    if verbose {
        println!("\n=== Execution Info ===");
        println!("Command to execute: {}", final_suggestion.command);
        println!("======================\n");
    }
    
    // Save to history before execution
    let history = History::new();
    let history_entry = Command::new(user_input.to_string(), command);
    history.add(history_entry)?;
    
    if verbose {
        println!("✓ Command saved to history");
        println!("Executing command...\n");
    }
    
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

async fn handle_bookmark(action: BookmarkAction, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let bookmark = Bookmark::new();
    
    match action {
        BookmarkAction::Add { name, command, description, tags } => {
            let item = BookmarkItem {
                name: name.clone(),
                command,
                description: description.unwrap_or_default(),
                tags,
                created_at: chrono::Utc::now().timestamp(),
            };
            bookmark.add(item)?;
            println!("Bookmark '{}' added successfully!", name);
        }
        BookmarkAction::List { tag } => {
            let items = bookmark.list(tag)?;
            if items.is_empty() {
                println!("No bookmarks found.");
                return Ok(());
            }
            
            println!("\n{:<20} {:<40} {}", "Name", "Command", "Tags");
            println!("{}", "-".repeat(100));
            
            for item in items {
                let name_display = if item.name.len() > 17 {
                    format!("{}...", &item.name[..17])
                } else {
                    item.name.clone()
                };
                
                let command_display = if item.command.len() > 37 {
                    format!("{}...", &item.command[..37])
                } else {
                    item.command.clone()
                };
                
                let tags_display = item.tags.join(", ");
                let tags_display = if tags_display.len() > 37 {
                    format!("{}...", &tags_display[..37])
                } else {
                    tags_display
                };
                
                println!("{:<20} {:<40} {}", name_display, command_display, tags_display);
            }
        }
        BookmarkAction::Get { name } => {
            if let Some(item) = bookmark.get(&name)? {
                println!("\nBookmark: {}", item.name);
                println!("Command: {}", item.command);
                println!("Description: {}", item.description);
                println!("Tags: {}", item.tags.join(", "));
                
                let datetime = chrono::DateTime::from_timestamp(item.created_at, 0)
                    .unwrap_or_default()
                    .format("%Y-%m-%d %H:%M:%S");
                println!("Created: {}", datetime);
            } else {
                println!("Bookmark '{}' not found", name);
            }
        }
        BookmarkAction::Remove { name } => {
            bookmark.remove(&name)?;
            println!("Bookmark '{}' removed", name);
        }
        BookmarkAction::Search { query } => {
            let items = bookmark.search(&query)?;
            if items.is_empty() {
                println!("No bookmarks found matching '{}'", query);
                return Ok(());
            }
            
            println!("\n{:<20} {:<40} {}", "Name", "Command", "Tags");
            println!("{}", "-".repeat(100));
            
            for item in items {
                let name_display = if item.name.len() > 17 {
                    format!("{}...", &item.name[..17])
                } else {
                    item.name.clone()
                };
                
                let command_display = if item.command.len() > 37 {
                    format!("{}...", &item.command[..37])
                } else {
                    item.command.clone()
                };
                
                let tags_display = item.tags.join(", ");
                
                println!("{:<20} {:<40} {}", name_display, command_display, tags_display);
            }
        }
        BookmarkAction::Save => {
            // Get last command from history
            let history = History::new();
            let commands = history.list(Some(1))?;
            
            if commands.is_empty() {
                println!("No command history found. Run a command first.");
                return Ok(());
            }
            
            let last_command = &commands[0];
            
            if verbose {
                println!("=== Verbose Mode ===");
                println!("Command to bookmark: {}", last_command.command);
                println!("Generating bookmark metadata with AI...");
                println!("===================\n");
            } else {
                println!("Generating bookmark metadata for: {}", last_command.command);
            }
            
            // Load config and initialize AI client
            let config = Config::from_env()?;
            let client = OpenAIClient::builder()
                .with_header("HTTP-Referer", "http://github.com/y1j2x34/shai")
                .with_header("X-Title", "SHAI")
                .with_endpoint(&config.endpoint)
                .with_api_key(&config.api_key)
                .build()?;
            
            // Generate bookmark metadata with AI
            let metadata = generate_bookmark_info(&client, &config.model, &last_command.command).await?;
            
            if verbose {
                println!("AI generated metadata:");
                println!("  Name: {}", metadata.name);
                println!("  Description: {}", metadata.description);
                println!("  Tags: {:?}\n", metadata.tags);
            }
            
            // Check if bookmark already exists
            if let Some(_existing) = bookmark.get(&metadata.name)? {
                println!("⚠ Bookmark '{}' already exists. Please use 'bookmark remove' first or choose a different name.", metadata.name);
                return Ok(());
            }
            
            // Create and save the bookmark
            let item = BookmarkItem {
                name: metadata.name.clone(),
                command: last_command.command.clone(),
                description: metadata.description,
                tags: metadata.tags,
                created_at: chrono::Utc::now().timestamp(),
            };
            
            bookmark.add(item)?;
            println!("✓ Bookmark '{}' saved successfully!", metadata.name);
        }
    }
    
    Ok(())
}
