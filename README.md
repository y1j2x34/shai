# SHAI - Shell AI Assistant

SHAI is a command-line tool that uses AI to suggest and execute shell commands based on natural language descriptions. Instead of remembering complex command syntax, simply describe what you want to do, and SHAI will suggest appropriate commands.

## Features

- Natural language to shell command conversion
- Multiple command suggestions for each query
- Interactive command selection and modification
- **Command history tracking** - automatically saves all executed commands
- **History search** - quickly find previously used commands
- Support for custom OpenAI API endpoints
- Cross-platform support (Linux, macOS, and Windows)
- Modular architecture for easy feature extensions

## Installation

### Pre-Built Binaries

Pre-built binaries for Linux, MacOS, and Windows can be found on the releases page.

You can use the following command on Linux, MacOS, or Windows to download the latest release, *shai* replace DEST with the directory where you'd like to put *shai*:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/y1j2x34/shai/refs/heads/master/install.sh | bash -s -- --to DEST
```

```sh
# create ~/bin
mkdir -p ~/bin

# download and extract shai to ~/bin/shai
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/y1j2x34/shai/refs/heads/master/install.sh | bash -s -- --tag v0.1.0 --to ~/bin

# add `~/bin` to the paths that your shell searches for executables
# this line should be added to your shells initialization file,
# e.g. `~/.bashrc` or `~/.zshrc`
export PATH="$PATH:$HOME/bin"

# shai should now be executable
shai --help
```

### Building from Source

1. Clone the repository:

    ```bash
    git clone https://github.com/y1j2x34/shai.git
    cd shai
    ```

2. Build and install:

    ```bash
    # For Unix-like systems
    cargo build --release
    sudo mv target/release/shai /usr/local/bin/

    # For Windows (PowerShell)
    cargo build --release
    Move-Item -Path .\target\release\shai.exe -Destination "$env:USERPROFILE\AppData\Local\Microsoft\WindowsApps\shai.exe"
    ```

## Configuration

SHAI requires environment variables for configuration. Create a `.env` file in your home directory or set the following environment variables:

```env
# Required: Your AI API key
SHAI_API_KEY=your-api-key-here

# Optional: Custom AI API endpoint (default: https://openrouter.ai/api/v1)
SHAI_API_ENDPOINT=https://openrouter.ai/api/v1

# Optional: AI model to use (default: meta-llama/llama-3.1-70b-instruct:free)
SHAI_MODEL=meta-llama/llama-3.1-70b-instruct:free

# Optional: Number of command suggestions to generate (default: 2)
SHAI_SUGGESTION_COUNT=2
```

## Usage

### Generate and Execute Commands

```bash
shai "your command description here"
```

The tool will:
1. Generate multiple command suggestions based on your description
2. Display them in an interactive selection menu
3. Allow you to modify the selected command before execution
4. Save the command to history
5. Execute the chosen command

### View Command History

```bash
# Show last 20 commands (default)
shai history

# Show last 10 commands
shai history --limit 10

# Search history for specific commands
shai history --search "docker"

# Clear all history
shai history --clear
```

### Examples

1. **Find large files:**

    ```bash
    shai "find files larger than 1GB in the current directory"
    ```

2. **Process text files:**

    ```bash
    shai "search for the word 'error' in all log files"
    ```

3. **System maintenance:**

    ```bash
    shai "clean up old docker images and containers"
    ```

4. **Review what you've done:**

    ```bash
    shai history --limit 5
    ```

5. **Find a previous Docker command:**

    ```bash
    shai history --search docker
    ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
