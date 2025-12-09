# CodeNamr-CLI

A blazing fast CLI for generating code names, inspired by the Gfycat-style memorable identifiers. Written in Rust.

## Quick Installation (Linux)

For a quick installation on Linux, you can use the provided `install.sh` script.

1.  **Clone the Repository:**
    ```bash
    git clone https://github.com/your-username/CodeNamr.git
    cd CodeNamr
    ```
    *(Note: You will need to replace `your-username` with the actual username after you push this to a git hosting provider like GitHub.)*

2.  **Run the Installer Script:**
    ```bash
    ./install.sh
    ```
    This script will check for Rust, build the CLI, and move the executable to `/usr/local/bin`. You might be prompted for your `sudo` password.

## Updating

To update the CLI to the latest version, run the `update.sh` script from the root of the repository:
```bash
./update.sh
```
This will pull the latest changes and reinstall the CLI.

## Uninstalling

To uninstall the CLI, run the `uninstall.sh` script from the root of the repository:
```bash
./uninstall.sh
```
This will remove the `codenamr` executable from `/usr/local/bin`. You might be prompted for your `sudo` password.

## Linux Installation and Usage

### Prerequisites
- Git
- Rust (and Cargo)

If you don't have Rust installed, you can install it with the following command:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 1. Clone the Repository
Clone this repository to your local machine:
```bash
git clone https://github.com/your-username/CodeNamr.git
cd CodeNamr/codenamr-cli
```
*(Note: You will need to replace `your-username` with the actual username after you push this to a git hosting provider like GitHub.)*

### 2. Build the CLI
Build the release version of the CLI using Cargo:
```bash
cargo build --release
```
The executable will be located at `target/release/codenamr-cli`.

### 3. Install (Optional)
You can move the executable to a directory in your system's `PATH` for easy access from anywhere.
```bash
sudo mv target/release/codenamr-cli /usr/local/bin/codenamr
```

### 4. Usage
Run the CLI to generate a code name.
```bash
codenamr
```

#### Output Formats
You can specify the output format using the `--format` (or `-f`) flag. The following formats are available:

- `normal` (default): `verb noun`
- `dash`: `verb-noun`
- `upper`: `VERB_NOUN`
- `lower`: `verb_noun`
- `camel`: `verbNoun`
- `pascal`: `VerbNoun`

**Examples:**
```bash
codenamr -f dash
# Outputs something like: talking-keyboard

codenamr --format camel
# Outputs something like: walkingLaptop
```
