use std::{env, fs};

use serde_json::Value;

fn main() {
    let args :Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path-to-json>");
        std::process::exit(1);
    }

    let path = &args[1];

    let content = match fs::read_to_string(path) {
        Ok(content) => content,

        Err(err) => {
            eprintln!("Failed to read file '{}' : {}", path, err);
            std::process::exit(1);
        }
    };

    let parsed: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Invalid JSON in '{}': {}", path, err);
            std::process::exit(1);
        }
    };

    println!("Loaded JSON file: {}", path);

    if let Some(obj) = parsed.as_object() {
        println!("Top-level keys: {}", obj.len());
        for key in obj.keys() {
            println!("- {}", key);
        }
    } else {
        println!("Top-level JSON is not an object");
    }
}
