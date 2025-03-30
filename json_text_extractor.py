#!/usr/bin/env python3
"""
JSON Text Extractor

Streams JSON data and extracts substantive text content (without loading the entire file into memory).
Outputs extracted text to stdout with newlines between fields.
"""

import argparse
import ijson
import sys
import os

def is_substantive(value, min_length=100):
    """Check if a value contains substantive text content."""
    if isinstance(value, str):
        # Check if string is longer than minimum length
        return len(value) >= min_length
    return False

def extract_text_from_json(file_obj, min_length=100):
    """
    Stream JSON and extract substantive text content.
    
    Args:
        file_obj: File-like object containing JSON data
        min_length: Minimum length for text to be considered substantive
    """
    # Track the prefix/path we're currently at
    current_prefix = []
    
    # Use ijson to stream through the JSON
    parser = ijson.parse(file_obj)
    
    for prefix, event, value in parser:
        if event == 'string' or event == 'number':
            if is_substantive(value, min_length):
                # Output the value with newline
                sys.stdout.write(str(value))
                sys.stdout.write("\n\n")
                sys.stdout.flush()

def main():
    parser = argparse.ArgumentParser(description='Extract substantive text from JSON files')
    parser.add_argument('file', nargs='?', type=argparse.FileType('r'), default=sys.stdin,
                        help='JSON file to process (defaults to stdin)')
    parser.add_argument('--min-length', type=int, default=100,
                        help='Minimum length for text to be considered substantive (default: 100)')
    
    args = parser.parse_args()
    
    try:
        extract_text_from_json(args.file, args.min_length)
    except BrokenPipeError:
        # Handle case when piping to another command that terminates early
        sys.stderr.close()
        sys.exit(0)
    finally:
        if args.file is not sys.stdin:
            args.file.close()

if __name__ == '__main__':
    main()
