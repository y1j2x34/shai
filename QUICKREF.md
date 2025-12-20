# SHAI Quick Reference

## Installation

```bash
cargo build --release
sudo mv target/release/shai /usr/local/bin/
```

## Configuration

```bash
export SHAI_API_KEY="your-api-key"
export SHAI_API_ENDPOINT="https://openrouter.ai/api/v1"  # optional
export SHAI_MODEL="meta-llama/llama-3.1-70b-instruct:free"  # optional
export SHAI_SUGGESTION_COUNT=2  # optional
```

## Usage

### Generate Commands

```bash
# Basic usage
shai "your command description"

# Examples
shai "find all Python files modified in the last 7 days"
shai "compress all log files older than 30 days"
shai "show me the top 10 largest directories"
```

### History Management

```bash
# View recent history (default: 20 items)
shai history

# Show specific number of items
shai history --limit 10
shai history -l 50

# Search history
shai history --search docker
shai history -s "git commit"

# Clear all history
shai history --clear
shai history -c
```

### Help

```bash
shai --help
shai history --help
```

## File Locations

- Configuration: Environment variables or `.env` file
- History: `~/.shai/history.json`
- Future bookmarks: `~/.shai/bookmarks.json`

## Common Workflows

### 1. Find and reuse a command
```bash
# Search for a previous docker command
shai history --search docker

# Copy the command you want and run it
```

### 2. Check what you did yesterday
```bash
# View recent history
shai history --limit 20
```

### 3. Clean up after experimentation
```bash
# Clear history of test commands
shai history --clear
```

## Exit Codes

- `0`: Success
- `1`: Error (check error message)

## Tips

1. **Be specific**: More detailed descriptions lead to better command suggestions
2. **Review before executing**: Always check the suggested command before running
3. **Modify as needed**: You can edit the command before execution
4. **Use history**: Search history to find and reuse previous commands
5. **Tag your descriptions**: Use clear descriptions for easier searching

## Examples by Category

### File Management
```bash
shai "find duplicate files by MD5 hash"
shai "rename all .txt files to .md"
shai "find empty directories"
```

### System Administration
```bash
shai "show processes using more than 1GB of memory"
shai "check disk space by directory"
shai "find files owned by user john"
```

### Development
```bash
shai "find all TODO comments in Python files"
shai "count lines of code in src directory"
shai "find git repositories in current directory"
```

### Docker
```bash
shai "remove all stopped containers"
shai "show container resource usage"
shai "export docker image to tar file"
```

### Git
```bash
shai "show commits from last week"
shai "find large files in git history"
shai "list branches sorted by last commit date"
```

## Troubleshooting

### Command not found
```bash
# Ensure shai is in your PATH
echo $PATH
which shai
```

### API key not set
```bash
# Set the API key
export SHAI_API_KEY="your-key"

# Or add to ~/.bashrc or ~/.zshrc
echo 'export SHAI_API_KEY="your-key"' >> ~/.bashrc
```

### History not saving
```bash
# Check if directory is writable
ls -la ~/.shai/
chmod 755 ~/.shai/
```

### No suggestions generated
1. Check internet connection
2. Verify API key is valid
3. Check API endpoint is accessible
4. Try a more specific description

## Advanced Usage

### Custom AI Models
```bash
# Use different model for better results
export SHAI_MODEL="anthropic/claude-3-sonnet"
```

### More Suggestions
```bash
# Get more options to choose from
export SHAI_SUGGESTION_COUNT=5
```

### Script Integration
```bash
#!/bin/bash
# Use in scripts (requires non-interactive mode - future feature)
# For now, best for interactive use
```

## Future Features

Coming soon (easy to add with current architecture):
- Bookmarks with tags
- Command templates
- Dangerous command detection
- Dry-run mode
- Command explanations
- Export/import functionality

## Getting Help

1. GitHub Issues: https://github.com/y1j2x34/shai/issues
2. Documentation: See ARCHITECTURE.md and TESTING.md
3. Examples: See README.md

