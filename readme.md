# ntime

`ntime` is a Rust-based alternative to the Unix `time` command, designed specifically for Windows systems. It allows users to measure the execution time of commands and provides detailed performance metrics.

## Features

- Measure execution time of any command or program
- Display user CPU time, system CPU time, and real elapsed time
- Easy-to-use command-line interface
- Lightweight and fast, written in Rust

## Installation

### Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

To install Rust and Cargo, visit the [official Rust website](https://www.rust-lang.org/tools/install) and follow the installation instructions for Windows.

### Building from source

1. Clone the repository:
   ```
   git clone https://github.com/Rinechxn/ntime.git
   cd ntime
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The compiled binary will be available in `target/release/ntime.exe`.

## Usage

```
ntime [OPTIONS] COMMAND [ARGS]...
```

For example:
```
ntime ping google.com -n 4
```

This will run the `ping` command and display its execution time statistics.

## Options

- `-h, --help`: Display help information
- `-v, --version`: Display version information

## Output

`ntime` provides the following timing information:

- Real time: The total elapsed wall-clock time
- User CPU time: The amount of CPU time spent in user-mode code
- System CPU time: The amount of CPU time spent in kernel-mode code

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the Unix `time` command
- Built with Rust 🦀