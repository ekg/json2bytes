use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use std::io::{self, BufReader, Read, Write};
use std::fs::File;
use std::collections::HashSet;

const DEFAULT_MIN_SIZE: usize = 10; // Minimum size in bytes to consider a string worth printing

fn process_json_value(
    value: &Value,
    min_size: usize,
    field_names: &Option<HashSet<String>>,
    current_field: Option<&String>,
    null_terminate: bool,
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
                
                writeln!(handle, "{}", s).unwrap();
                
                // Add null byte termination if requested
                if null_terminate {
                    handle.write_all(&[0]).unwrap();
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                process_json_value(item, min_size, field_names, current_field, null_terminate);
            }
        }
        Value::Object(obj) => {
            for (key, val) in obj {
                process_json_value(val, min_size, field_names, Some(key), null_terminate);
            }
        }
        _ => {}
    }
}

fn process_stream<R: Read>(
    reader: R,
    min_size: usize,
    field_names: &Option<HashSet<String>>,
    null_terminate: bool,
) -> Result<()> {
    let reader = BufReader::new(reader);
    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<Value>();
    
    for value in stream {
        let value = value.context("Failed to parse JSON value")?;
        process_json_value(&value, min_size, field_names, None, null_terminate);
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
    
    /// Add a null byte after each extracted string
    #[arg(long)]
    null_delim: bool,
}

fn main() -> Result<()> {
    // Parse command-line arguments using clap
    let args = Args::parse();
    
    // Convert fields vector to HashSet if provided
    let field_names = args.fields.map(|fields| {
        fields.into_iter().collect::<HashSet<String>>()
    });
    
    for input in &args.inputs {
        if input != "-" {
            // Read from file
            let file = File::open(input).context(format!("Failed to open input file: {}", input))?;
            process_stream(file, args.size, &field_names, args.null_delim)?;
        } else {
            // Read from stdin
            process_stream(io::stdin(), args.size, &field_names, args.null_delim)?;
        }
    }
    
    Ok(())
}
