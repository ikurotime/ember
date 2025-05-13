# Ember - A Terminal Text Editor in Rust

A simple terminal-based text editor written in Rust for learning purposes. This project is an educational exercise to understand terminal manipulation, text editing concepts, and Rust programming.

## Features

- Terminal-based user interface [x]
- Basic text file loading and display [x]
- Welcome message with version information [x]
- Line truncation for terminal width [x]
- Empty line visualization with `~` characters [x]
- Terminal resizing support [x]
- Text editing functionality [ ]

## Prerequisites

- Rust (latest stable version recommended)
- A terminal that supports ANSI escape codes

## Building

```bash
# Clone the repository
git clone https://github.com/yourusername/ember.git
cd ember

# Build the project
cargo build --release

# Run the editor
cargo run
```

## Project Structure

- `src/editor/view.rs` - Main editor view implementation
- `src/editor/buffer.rs` - Text buffer management
- `src/editor/terminal.rs` - Terminal interaction utilities

## Dependencies

- `crossterm` (v0.28.1) - For terminal manipulation and input handling

## Learning Goals

This project serves as a learning exercise for:
- Terminal manipulation and ANSI escape codes
- Rust ownership and borrowing concepts
- Error handling in Rust
- Terminal-based UI development
- Text editor concepts and implementation

## Contributing

This is a learning project, but if you find any bugs or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License

This project is open source and available under the MIT License.

## Acknowledgments

- Inspired by various terminal-based editors like Vim and Nano
- Built with the help of the Rust community and documentation 