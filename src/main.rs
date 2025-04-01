use anyhow::{Context, Result};
use serde_json::Value;
use std::io::{self, BufRead, BufReader, Read};
use std::fs::File;
use std::env;
use std::collections::HashSet;

const DEFAULT_MIN_SIZE: usize = 10; // Minimum size in bytes to consider a string worth printing

fn process_json_value(value: &Value, min_size: usize, field_names: &Option<HashSet<String>>, current_field: Option<&String>) {
    // Check if we should process this field based on field_names filter
    let should_process = field_names.as_ref().map_or(true, |names| {
        current_field.map_or(false, |field| names.contains(field))
    });

    match value {
        Value::String(s) => {
            if should_process && s.len() >= min_size {
                println!("{}", s);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                process_json_value(item, min_size, field_names, current_field);
            }
        }
        Value::Object(obj) => {
            for (key, val) in obj {
                process_json_value(val, min_size, field_names, Some(key));
            }
        }
        _ => {}
    }
}

fn process_stream<R: Read>(reader: R, min_size: usize, field_names: &Option<HashSet<String>>) -> Result<()> {
    let reader = BufReader::new(reader);
    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<Value>();
    
    for value in stream {
        let value = value.context("Failed to parse JSON value")?;
        process_json_value(&value, min_size, field_names, None);
    }
    
    Ok(())
}

fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    
    // Initialize variables with default values
    let mut input_source = String::from("-");
    let mut min_size = DEFAULT_MIN_SIZE;
    let mut field_names: Option<HashSet<String>> = None;
    
    // Process command line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" | "--fields" => {
                if i + 1 < args.len() {
                    field_names = Some(
                        args[i + 1]
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect()
                    );
                    i += 2;
                } else {
                    eprintln!("Error: --fields option requires an argument");
                    std::process::exit(1);
                }
            }
            "-s" | "--size" => {
                if i + 1 < args.len() {
                    if let Ok(size) = args[i + 1].parse() {
                        min_size = size;
                        i += 2;
                    } else {
                        eprintln!("Error: Invalid minimum size");
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("Error: --size option requires an argument");
                    std::process::exit(1);
                }
            }
            _ => {
                // Assume it's the input file
                input_source = args[i].clone();
                i += 1;
            }
        }
    }
    
    if input_source != "-" {
        // Read from file
        let file = File::open(&input_source).context("Failed to open input file")?;
        process_stream(file, min_size, &field_names)?;
    } else {
        // Read from stdin
        process_stream(io::stdin(), min_size, &field_names)?;
    }
    
    Ok(())
}
