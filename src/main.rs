use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use std::io::{self, BufReader, Read, Write};
use std::fs::File;
use std::collections::HashSet;

const DEFAULT_MIN_SIZE: usize = 0; // Minimum size in bytes to consider a string worth printing

fn parse_separator(input: &str) -> Result<Vec<u8>> {
    if input.starts_with("\\x") || input.starts_with("\\X") {
        let hex_str = &input[2..];
        if hex_str.len() % 2 != 0 {
            return Err(anyhow::anyhow!("Hex string must have even length"));
        }
        
        let mut bytes = Vec::new();
        for i in (0..hex_str.len()).step_by(2) {
            let byte_str = &hex_str[i..i+2];
            let byte = u8::from_str_radix(byte_str, 16)
                .context(format!("Invalid hex byte: {}", byte_str))?;
            bytes.push(byte);
        }
        Ok(bytes)
    } else {
        Ok(input.as_bytes().to_vec())
    }
}

fn process_json_value(
    value: &Value,
    min_size: usize,
    field_names: &Option<HashSet<String>>,
    current_field: Option<&String>,
    separator: &[u8],
) {
    // Check if we should process this field based on field_names filter
    let should_process = field_names.as_ref().map_or(true, |names| {
        current_field.map_or(false, |field| names.contains(field))
    });

    match value {
        Value::String(s) => {
            if should_process && s.len() >= min_size {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                
                write!(handle, "{}", s).unwrap();
                handle.write_all(separator).unwrap();
            }
        }
        Value::Array(arr) => {
            for item in arr {
                process_json_value(item, min_size, field_names, current_field, separator);
            }
        }
        Value::Object(obj) => {
            for (key, val) in obj {
                process_json_value(val, min_size, field_names, Some(key), separator);
            }
        }
        _ => {}
    }
}

fn process_stream<R: Read>(
    reader: R,
    min_size: usize,
    field_names: &Option<HashSet<String>>,
    separator: &[u8],
) -> Result<()> {
    let reader = BufReader::new(reader);
    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<Value>();
    
    for value in stream {
        let value = value.context("Failed to parse JSON value")?;
        process_json_value(&value, min_size, field_names, None, separator);
    }
    
    Ok(())
}

/// CLI arguments for json2bytes
#[derive(Parser)]
#[command(
    name = "json2bytes",
    author = "json2bytes developers",
    version,
    about = "Extract string values from JSON that meet a minimum length requirement",
    long_about = None
)]
struct Args {
    /// JSON files to process (use '-' for stdin)
    #[arg(default_value = "-", num_args = 1..)]
    inputs: Vec<String>,

    /// Minimum string length to extract
    #[arg(short, long, default_value_t = DEFAULT_MIN_SIZE)]
    size: usize,

    /// Only extract strings from specified fields (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    fields: Option<Vec<String>>,
    
    /// Record separator to use (default: ASCII record separator \\x1e). Supports hex notation like \\x1e, \\x00
    #[arg(long, default_value = "\\x1e")]
    separator: String,
}

fn main() -> Result<()> {
    // Parse command-line arguments using clap
    let args = Args::parse();
    
    // Convert fields vector to HashSet if provided
    let field_names = args.fields.map(|fields| {
        fields.into_iter().collect::<HashSet<String>>()
    });
    
    // Parse the separator
    let separator = parse_separator(&args.separator)?;
    
    for input in &args.inputs {
        if input != "-" {
            // Read from file
            let file = File::open(input).context(format!("Failed to open input file: {}", input))?;
            process_stream(file, args.size, &field_names, &separator)?;
        } else {
            // Read from stdin
            process_stream(io::stdin(), args.size, &field_names, &separator)?;
        }
    }
    
    Ok(())
}
