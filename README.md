# Pick-a-File

`pick-a-file` is a command-line utility written in Rust that allows users to select a random file from a specified directory based on one or more given file extensions. It supports an optional command-line switch to control output behavior.

## Features

- **File Selection**: Choose a file from a specified directory that matches one or more given file extensions.
- **Output Control**: Use the `--bare` flag to return only the selected file's path to `stdout`. This is great for command substitution or integration into scripts.

## Usage

```bash
cargo run [--bare] <path> <file_extension1> <file_extension2> ...
```

- `<path>`: The directory to search for files.
- `<file_extension1>`, `<file_extension2>`: The file extensions to filter files by (e.g., `.txt`, `.jpg`, `.git`).

### Example

To run the program and select files with `.txt` and `.jpg` extensions from the `documents` directory:

```bash
cargo run /path/to/documents .txt .jpg
```

To return only the selected file's path using the `--bare` option:

```bash
cargo run --bare /path/to/documents .txt .jpg
```

## Integration with Unix/Linux Terminals

This tool is particularly handy when used in conjunction with Unix(-like) terminals. It allows for seamless integration into scripts or workflows, for when you want to test another tool with a random file of a specific type.

### Examples

1. **Direct Invocation**: You can invoke `pick-a-file` directly in the terminal to select a random file of a specified type from your home directory:
   ```bash
   pick-a-file ~ .ext
   ```

2. **Command Substitution**: Use command substitution to pass the selected file as an argument to another command. For example, if you want to open a random `.txt` file with `nano`:
   ```bash
   nano "$(pick-a-file --bare ~/documents .txt)"
   ```

Or, if you prefer the backtick syntax:
   ```bash
   nano "`pick-a-file --bare ~/documents .txt`"
   ```

Note that double quotes will make sure paths with spaces are handled correctly.

3. **Scripting**: Incorporate `pick-a-file` into shell scripts for automated tasks. For instance, you can create a script that processes a random image file:
   ```bash
   #!/bin/bash
   image="$(pick-a-file --bare ~/images .jpg)"
   display "$image"  # Assuming you have a command to display images
   ```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/pick-a-file.git
   ```

2. Navigate to the project directory:
   ```bash
   cd pick-a-file
   ```

3. Build the project:
   ```bash
   cargo build
   ```
