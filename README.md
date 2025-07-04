# json2bytes

A command-line utility that extracts string values from JSON data that meet a specified minimum length requirement. Commonly used with `-f text` to extract only 'text' fields from JSON objects.

## Features

- Extract all string values from JSON files or stdin
- Filter strings by minimum length (default: 0)
- Filter by specific field names (e.g., `-f text`)
- Process JSON streams (newline-delimited JSON)
- Configurable record separator (default: ASCII record separator `\x1e`)
- Support for multiple input files
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
# Process a JSON file extracting all strings (default minimum length is 0)
json2bytes input.json

# Process multiple JSON files
json2bytes file1.json file2.json file3.json

# Process a JSON file with custom minimum string length
json2bytes --size 10 input.json

# Extract only from 'text' fields (common usage)
json2bytes -f text input.json

# Process JSON from stdin
cat input.json | json2bytes -

# Extract strings only from specific fields
json2bytes --fields "bio,email" input.json

# Combine options
json2bytes --fields "description,body" --size 15 file1.json file2.json
```

### Command Line Options

- `-f, --fields <field_list>`: Only extract strings from specified fields (comma-separated). By default, extracts from ALL string fields. Common usage: `-f text`
- `-s, --size <size>`: Minimum string length to extract (default: 0)
- `--separator <separator>`: Record separator to use (default: ASCII record separator `\x1e`). Supports hex notation like `\x1e`, `\x00`

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

Output (with default min length 0, all strings extracted):
```
John Doe
Software developer with 5+ years of experience
rust
javascript
python
john.doe@example.com
123-456-7890
```

Output (with min length 10):
```
Software developer with 5+ years of experience
john.doe@example.com
123-456-7890
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

#### Using Custom Separator

Command:
```bash
# Use null byte as separator
json2bytes --separator '\x00' -f email input.json | hexdump -C
```

Output: (String followed by null byte)
```
00000000  6a 6f 68 6e 2e 64 6f 65  40 65 78 61 6d 70 6c 65  |john.doe@example|
00000010  2e 63 6f 6d 00                                    |.com.|
```

#### Common Usage: Extract 'text' Fields

For JSON data with 'text' fields:
```json
[
  {"id": 1, "text": "Hello world", "metadata": "ignore this"},
  {"id": 2, "text": "Another message", "timestamp": "2024-01-01"}
]
```

Command:
```bash
json2bytes -f text input.json
```

Output:
```
Hello worldAnother message
```

## How It Works

The tool reads JSON data from files or stdin, traverses all values, and outputs any string values that are at least as long as the specified minimum length. By default, it extracts from all string fields, but you can filter to specific field names using the `-f` option. Each extracted string is followed by a configurable separator (default: ASCII record separator `\x1e`).

## License

[MIT License](LICENSE)
