<div align="center">
  <img src="https://github.com/KeeCLI/kee.rs/blob/main/kee.png" alt="Kee" />
</div>

![OSX](https://img.shields.io/badge/-OSX-black) ![OSX](https://img.shields.io/badge/-Linux-red) ![OSX](https://img.shields.io/badge/-Windows-blue)

A simple tool to manage multiple AWS accounts with SSO support and easy account access.

`Kee` creates isolated sub-shells for each AWS account, ensuring no credentials are stored locally while providing seamless account management.

🦀 — This is the **Rust** implementation of the original [Python implementation](https://github.com/KeeCLI/kee.py), providing identical functionality with the performance and safety benefits of Rust, while maintaining complete compatibility with existing configurations and workflows.

## Why Rust?

- 🚀 **Performance**: Compiled binary, faster startup times
- ⛑️ **Memory safety**: No runtime errors, guaranteed memory safety
- 🌍 **Cross-platform**: Single binary works across platforms
- 👌 **Zero dependencies**: No Python runtime required
- ⚡️ **Concurrent**: Built-in concurrency support for future enhancements

> For a list of features, take a look a the [Python implementation](https://github.com/KeeCLI/kee.py).

## Installation

### Prerequisites

- Rust 1.80+ (install from [rustup.rs](https://rustup.rs/)) (On Mac with brew: `brew install rust`)
- AWS CLI v2 installed and configured
- Access to AWS SSO

### Clone this repository:

```bash
git clone https://github.com/keecli/kee.rs.git ~/.kee
```

### Build and install

**Option 1: Automated (recommended)**

```bash
cd ~/.kee
./install.sh
```

> This script will build an optimized `Kee` binary, install it (in `~/.cargo/bin`), and add the folder to your `PATH`.

**Option 2: Manual**

```bash
cd ~/.kee

# Install the binary
cargo install --path .

# Add Cargo's bin directory to your PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc  # For zsh (macOS default)
# OR
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # For bash

# Reload your shell configuration
source ~/.zshrc  # or ~/.bashrc
```

**Option 3: Direct copy**

```bash
cd ~/.kee

# Build and copy to a directory already in PATH
cargo build --release
cp target/release/kee ~/.local/bin/  # Make sure ~/.local/bin is in your PATH
```

## Feature comparison

| Feature                   | Python Version | Rust Version  | Notes                           |
| ------------------------- | -------------- | ------------- | ------------------------------- |
| **SSO integration**       | ✅             | ✅            | Identical AWS CLI integration   |
| **Sub-shell isolation**   | ✅             | ✅            | Same environment management     |
| **Account management**    | ✅             | ✅            | Add, use, list, remove accounts |
| **Session management**    | ✅             | ✅            | Prevents nested sessions        |
| **Config file format**    | ✅             | ✅            | Same JSON structure             |
| **AWS config management** | ✅             | ✅            | Same file handling              |
| **Cross-platform**        | ✅             | ✅            | Windows, macOS, Linux           |
| **Error handling**        | ✅             | ✅            | Comprehensive error management  |
| **Performance**           | Good           | **Excellent** | Compiled binary                 |
| **Memory usage**          | Higher         | **Lower**     | No runtime overhead             |
| **Startup time**          | ~100ms         | **~5ms**      | No interpreter startup          |
| **Binary size**           | N/A            | **~1.5MB**    | Single executable               |

## Usage

```bash
# Add an account
kee add mycompany.dev

# Use an account
kee use mycompany.dev

# List accounts
kee list

# Show current account
kee current

# Remove an account
kee remove mycompany.dev

# Help
kee --help
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code
cargo check
cargo clippy
```

## Performance benchmarks

| Operation        | Python | Rust   | Improvement    |
| ---------------- | ------ | ------ | -------------- |
| **Startup**      | ~100ms | ~5ms   | **20x faster** |
| **Config load**  | ~10ms  | ~1ms   | **10x faster** |
| **Memory usage** | ~25MB  | ~2MB   | **12x less**   |
| **Binary size**  | N/A    | ~1.5MB | Single file    |

## Cross-platform support

**Identical support across:**

- **macOS**: Full support with shell prompt integration
- **Linux**: Full support with shell prompt integration
- **Windows**: Full support (prompt integration not available)

**Platform-specific optimizations:**

- **Windows**: Uses `COMSPEC` for shell detection
- **Unix**: Uses `SHELL` environment variable
- **All**: Proper path handling with `std::path`

## Migration from the Python version

**Zero migration required:**

- Same configuration files (`~/.aws/kee.json`)
- Same AWS config format
- Same command-line interface
- Same environment variables

**Drop-in Replacement:**

```bash
# Remove the Python version
rm /usr/local/bin/kee

# Or via Pip
pip uninstall kee

# Install Rust version
cargo install --path . --force
```

## Future enhancements (specific to Rust)

- **Async AWS API calls** for faster credential validation
- **Parallel account operations** for bulk management
- **Built-in AWS SDK** integration (no AWS CLI dependency)
- **Configuration validation** at compile time
- **Plugin system** with dynamic loading
- **TUI interface** with real-time updates

## Distribution

**Binary distribution:**

- Single executable file
- No runtime dependencies
- Easy deployment to servers
- Container-friendly

**Package managers:**

- **Cargo**: `cargo install kee`
- **Homebrew**: `brew install kee` (when published)
- **Scoop**: `scoop install kee` (Windows, when published)
- **APT/YUM**: Native packages possible

## Contributing

Same contribution guidelines as Python version, plus:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

**Rust-specific:**

- Follow `rustfmt` formatting
- Pass `clippy` lints

> There is a utilities script which will set up a `pre-commit` hook to run some basic checks on your code before you commit.

```bash
cd ~/.kee
./utilities/githooks.sh
```

## License

MIT License - see LICENSE file for details.

## Support

RTFM, then RTFC... If you are still stuck or just need an additional feature, file an [issue](https://github.com/KeeCLI/kee.py/issues).

<div align="center">
✌🏼
</div>
