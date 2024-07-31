# OMPM ( One More Project Manager)

OMPM is a command-line tool written in Rust to help you initialize and manage project repositories for multiple programming languages. It supports creating a new project with Git initialization, adding default files, and creating language-specific project setups.

## Features

- Initialize a new project with Git repository.
- Add default files like `README.md` and `.gitignore`.
- Support for multiple programming languages: Rust, Python, Node.js (JavaScript).
- Option to create a full project setup with additional files and directories.

## Installation

First, clone the repository:

```sh
git clone https://github.com/your-username/project_manager.git
cd project_manager
```

Then, build the project using Cargo:

```sh
cargo build --release
```

This will create an executable in the `target/release` directory.

## Usage

### Initialize a New Project

To initialize a new project, use the `init` command with the project name. You can also specify the programming language and whether to create a full setup.

#### Basic Initialization

```sh
cargo run -- init my_project
```

This will create a new project named `my_project` with default Rust setup.

#### Specify Language

```sh
cargo run -- init my_project --language python
```

This will create a new project named `my_project` with default Python setup.

#### Full Setup

```sh
cargo run -- init my_project --language node --full
```

This will create a new project named `my_project` with a full Node.js setup, including additional files and directories.

### Add Default Files

You can add default files to your project using the `add` command:

```sh
cargo run -- add readme
```

This will create a `README.md` file in the current directory.

## Supported Languages

- Rust
- Python
- Node.js (JavaScript)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

Things to be implemented

- Add more languages

- Make each project more personalizable ( add individual commands to choose speicifc files you want inside the rep)

- Try to create a base makefile

- More modular and organized code

## Acknowledgements

- [clap](https://crates.io/crates/clap) for command-line argument parsing.
- [git2](https://crates.io/crates/git2) for Git repository management.
```