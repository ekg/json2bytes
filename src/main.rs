use anyhow::{Context, Result};
use serde_json::Value;
use std::io::{self, BufRead, BufReader, Read};
use std::fs::File;
use std::env;

const DEFAULT_MIN_SIZE: usize = 10; // Minimum size in bytes to consider a string worth printing

fn process_json_value(value: &Value, min_size: usize) {
    match value {
        Value::String(s) => {
            if s.len() >= min_size {
                println!("{}", s);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                process_json_value(item, min_size);
            }
        }
        Value::Object(obj) => {
            for (_, val) in obj {
                process_json_value(val, min_size);
            }
        }
        _ => {}
    }
}

fn process_stream<R: Read>(reader: R, min_size: usize) -> Result<()> {
    let reader = BufReader::new(reader);
    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<Value>();
    
    for value in stream {
        let value = value.context("Failed to parse JSON value")?;
        process_json_value(&value, min_size);
    }
    
    Ok(())
}

fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    let min_size = args.get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(DEFAULT_MIN_SIZE);
    
    if args.len() > 1 && args[1] != "-" {
        // Read from file
        let file = File::open(&args[1]).context("Failed to open input file")?;
        process_stream(file, min_size)?;
    } else {
        // Read from stdin
        process_stream(io::stdin(), min_size)?;
    }
    
    Ok(())
}
