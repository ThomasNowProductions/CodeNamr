# Codenamr

Codenamr is a tool for generating memorable code names in various formats. It creates random verb-noun combinations that are easy to remember and perfect for naming projects, branches, or any other code-related entities.

## Installation

To install the Codenamr CLI, run the following command:

```bash
curl -fsSL https://codenamr.vercel.app/codenamr.sh | bash -s -- install
```

This script will:
- Check for Rust installation
- Clone the repository
- Build the CLI in release mode
- Install the executable to `/usr/local/bin/codenamr`

## Update

To update the Codenamr CLI to the latest version:

```bash
curl -fsSL https://codenamr.vercel.app/codenamr.sh | bash -s -- update
```

This script will pull the latest changes, rebuild, and reinstall the CLI.

## Uninstall

To uninstall the Codenamr CLI:

```bash
curl -fsSL https://codenamr.vercel.app/codenamr.sh | bash -s -- uninstall
```

This will remove the `codenamr` executable from `/usr/local/bin`.

## Usage

Generate a single code name:

```bash
codenamr
```

Output:
```
searching laptop
```

### Options

#### Format (`-f, --format`)

Specify the output format for the generated names. Available formats:

- `normal` - Space-separated words (default): `searching laptop`
- `kebab` - Kebab case: `searching-laptop`
- `snake` - Snake case: `searching_laptop`
- `constant` - Constant case: `SEARCHING_LAPTOP`
- `camel` - Camel case: `searchingLaptop`
- `pascal` - Pascal case: `SearchingLaptop`

```bash
codenamr --format kebab
codenamr -f snake
```

#### Count (`-n, --count`)

Generate multiple names at once:

```bash
codenamr --count 5
```

Output:
```
searching laptop
talking phone
walking garden
eating river
running mountain
```

#### Copy (`-c, --copy`)

Copy the first generated name (or all names if count > 1) directly to clipboard:

```bash
codenamr --copy
codenamr -n 10 --copy
```

#### Seed (`-s, --seed`)

Use a specific random seed for reproducible name generation:

```bash
codenamr --seed 12345
```

Running this command multiple times with the same seed will always produce the same name.

#### Prefix (`-p, --prefix`)

Add a custom prefix to each generated name:

```bash
codenamr --prefix myproject
```

Output:
```
myproject searching laptop
```

```bash
codenamr -f kebab --prefix myproject
```

Output:
```
myproject-searching-laptop
```

#### Suffix (`-u, --suffix`)

Add a custom suffix to each generated name:

```bash
codenamr --suffix v1
```

Output:
```
searching laptop v1
```

#### Score (`--score`)

Display memorability scores for each generated name (0-100 scale):

```bash
codenamr --count 3 --score
```

Output:
```
searching laptop (score: 78.5)
talking phone (score: 75.0)
walking garden (score: 80.0)
```

### Combining Options

You can combine multiple options for powerful name generation:

```bash
codenamr -f pascal -n 5 -p myapp -s 42 --copy --score
```

This generates 5 Pascal-cased names with the prefix "myapp", using seed 42 for reproducibility, copies them to clipboard, and shows memorability scores.

### Examples

**Generate 10 kebab-case names for feature branches:**
```bash
codenamr -f kebab -n 10
```

**Generate a single camelCase name with prefix and copy to clipboard:**
```bash
codenamr -f camel -p user --copy
```

**Generate reproducible names for documentation:**
```bash
codenamr --seed 99999 -f constant
```

**Create PascalCase class names with suffix:**
```bash
codenamr -f pascal -u Service -n 3
```

### Limitations

- Maximum count: 1,000,000 names per command
- Names are generated from a curated list of 140 verbs and 200+ nouns
- Memorability scoring considers length, word count, phonetics, repetition, common words, and alliteration
