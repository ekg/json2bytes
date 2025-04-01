# json2bytes

A command-line utility that extracts string values from JSON data that meet a specified minimum length requirement.

## Features

- Extract all string values from JSON files or stdin
- Filter strings by minimum length
- Process JSON streams (newline-delimited JSON)
- Simple command-line interface

## Installation

### Prerequisites

- Rust and Cargo (install from [rust-lang.org](https://www.rust-lang.org/tools/install))

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/json2bytes.git
cd json2bytes

# Build the project
cargo build --release
```

The compiled binary will be available at `target/release/json2bytes`.

## Usage

```bash
# Process a JSON file with default minimum string length
json2bytes input.json

# Process multiple JSON files
json2bytes file1.json file2.json file3.json

# Process a JSON file with custom minimum string length
json2bytes --size 10 input.json

# Process JSON from stdin
cat input.json | json2bytes -

# Extract strings only from specific fields
json2bytes --fields "bio,email" input.json

# Combine options
json2bytes --fields "description,body" --size 15 file1.json file2.json
```

### Command Line Options

- `-f, --fields <field_list>`: Only extract strings from specified fields (comma-separated)
- `-s, --size <size>`: Minimum string length to extract (default: 10)

### Examples

#### Basic Example

Input JSON:
```json
{
  "name": "John Doe",
  "age": 30,
  "bio": "Software developer with 5+ years of experience",
  "tags": ["rust", "javascript", "python"],
  "contact": {
    "email": "john.doe@example.com",
    "phone": "123-456-7890"
  }
}
```

Output (with min length 10):
```
Software developer with 5+ years of experience
john.doe@example.com
```

#### Using Field Filters

Command:
```bash
json2bytes --fields "email" input.json
```

Output:
```
john.doe@example.com
```

## How It Works

The tool reads JSON data from a file or stdin, traverses all values, and outputs any string values that are at least as long as the specified minimum length.

## License

[MIT License](LICENSE)
