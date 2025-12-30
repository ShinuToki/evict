# evict

![Release](https://img.shields.io/github/v/release/ShinuToki/evict?color=brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)

A fast, lightweight command-line tool for Windows that identifies and terminates processes using specific TCP ports.

> [!NOTE]  
> This tool forcefully terminates processes. Use with caution and ensure you're terminating the correct process.

## Demo

[![asciicast](https://asciinema.org/a/jt7Aj3oW4uSzLRLhoPnuZNw4O.svg)](https://asciinema.org/a/jt7Aj3oW4uSzLRLhoPnuZNw4O)

Instead of manually hunting down the process ID and killing it, just use `evict`.

## Features

- **Fast**: Instantly identifies which process is using a port
- **Simple**: One command to free any port
- **Safe**: Shows you what process will be terminated before doing it
- **Native**: Uses Windows APIs for reliable port detection
- **Lightweight**: Single executable, no dependencies

## Installation

### Pre-built Binaries

Pre-compiled binaries are available in the [Releases](https://github.com/ShinuToki/evict/releases) section.

### From Source

```bash
git clone https://github.com/ShinuToki/evict.git
cd evict
cargo install --path .
```

## Usage

### Basic Usage

```bash
evict <PORT>
```

### Examples

Free port 8080:

```bash
evict 8080
```

Output:

```text
Found process using port:
  PID: 12345
  Name: node.exe

Terminating process...
Port 8080 is now free
```

> [!TIP]  
> If you receive an 'Access Denied' error, try running your terminal as Administrator.

## Requirements

- **Operating System**: Windows (uses Windows-specific APIs)
- **Privileges**: May require administrator privileges to terminate certain processes
- **Rust**: 1.70+ (for building from source)

## Testing

The project includes comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.
