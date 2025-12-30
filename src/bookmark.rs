use std::path::PathBuf;
use crate::storage::{Storage, get_data_dir};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BookmarkItem {
    pub name: String,
    pub command: String,
    pub description: String,
    pub tags: Vec<String>,
    pub created_at: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BookmarkData {
    pub bookmarks: Vec<BookmarkItem>,
}

impl Default for BookmarkData {
    fn default() -> Self {
        Self {
            bookmarks: Vec::new(),
        }
    }
}

pub struct Bookmark;

impl Bookmark {
    pub fn new() -> Self {
        Self
    }

    pub fn add(&self, item: BookmarkItem) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = self.load_data()?;
        
        // Check if bookmark with same name already exists
        if data.bookmarks.iter().any(|b| b.name == item.name) {
            return Err(format!("Bookmark '{}' already exists", item.name).into());
        }
        
        data.bookmarks.push(item);
        self.save(&data)
    }

    pub fn list(&self, tag: Option<String>) -> Result<Vec<BookmarkItem>, Box<dyn std::error::Error>> {
        let data = self.load_data()?;
        
        let bookmarks = if let Some(tag_filter) = tag {
            data.bookmarks
                .into_iter()
                .filter(|b| b.tags.contains(&tag_filter))
                .collect()
        } else {
            data.bookmarks
        };
        
        Ok(bookmarks)
    }

    pub fn get(&self, name: &str) -> Result<Option<BookmarkItem>, Box<dyn std::error::Error>> {
        let data = self.load_data()?;
        Ok(data.bookmarks.into_iter().find(|b| b.name == name))
    }

    pub fn remove(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = self.load_data()?;
        data.bookmarks.retain(|b| b.name != name);
        self.save(&data)
    }

    pub fn search(&self, query: &str) -> Result<Vec<BookmarkItem>, Box<dyn std::error::Error>> {
        let data = self.load_data()?;
        let query_lower = query.to_lowercase();
        
        let results: Vec<BookmarkItem> = data.bookmarks
            .into_iter()
            .filter(|b| {
                b.name.to_lowercase().contains(&query_lower)
                    || b.command.to_lowercase().contains(&query_lower)
                    || b.description.to_lowercase().contains(&query_lower)
                    || b.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect();
        
        Ok(results)
    }

    fn load_data(&self) -> Result<BookmarkData, Box<dyn std::error::Error>> {
        match self.load::<BookmarkData>() {
            Ok(data) => Ok(data),
            Err(_) => Ok(BookmarkData::default()),
        }
    }
}

impl Storage for Bookmark {
    fn get_storage_path(&self) -> PathBuf {
        get_data_dir().join("bookmarks.json")
    }
}

/* 
To integrate this feature into main.rs, add:

1. In lib.rs:
   pub mod bookmark;
   pub use bookmark::Bookmark;

2. In main.rs Commands enum:
   #[derive(Subcommand)]
   enum Commands {
       History { ... },
       Bookmark {
           #[command(subcommand)]
           action: BookmarkAction,
       },
   }

   #[derive(Subcommand)]
   enum BookmarkAction {
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
       List {
           #[arg(short, long)]
           tag: Option<String>,
       },
       Get {
           name: String,
       },
       Remove {
           name: String,
       },
       Search {
           query: String,
       },
   }

3. In main() match:
   Commands::Bookmark { action } => {
       return handle_bookmark(action);
   }

4. Add handler function:
   fn handle_bookmark(action: BookmarkAction) -> Result<(), Box<dyn std::error::Error>> {
       let bookmark = Bookmark::new();
       
       match action {
           BookmarkAction::Add { name, command, description, tags } => {
               let item = BookmarkItem {
                   name,
                   command,
                   description: description.unwrap_or_default(),
                   tags,
                   created_at: chrono::Utc::now().timestamp(),
               };
               bookmark.add(item)?;
               println!("Bookmark added successfully!");
           }
           BookmarkAction::List { tag } => {
               let items = bookmark.list(tag)?;
               for item in items {
                   println!("{}: {} (tags: {})", 
                       item.name, 
                       item.command, 
                       item.tags.join(", ")
                   );
               }
           }
           BookmarkAction::Get { name } => {
               if let Some(item) = bookmark.get(&name)? {
                   println!("Name: {}", item.name);
                   println!("Command: {}", item.command);
                   println!("Description: {}", item.description);
                   println!("Tags: {}", item.tags.join(", "));
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
               for item in items {
                   println!("{}: {}", item.name, item.command);
               }
           }
       }
       
       Ok(())
   }

Usage examples:
  shai bookmark add --name "docker-clean" --command "docker system prune -af" --tags docker cleanup
  shai bookmark list
  shai bookmark list --tag docker
  shai bookmark get docker-clean
  shai bookmark search docker
  shai bookmark remove docker-clean
*/

