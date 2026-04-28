mod ast;
mod finding;
mod model;
mod rules;

use std::env;

use ast::load_ast;
use rules::detect_initializer_exposure;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path-to-ast-json>");
        std::process::exit(1);
    }

    let path = &args[1];

    let ast = match load_ast(path) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let findings = detect_initializer_exposure(&ast);

    if findings.is_empty() {
        println!("No findings.");
        return;
    }

    println!("Findings:");
    for finding in &findings {
        println!(
            "- [{}] {} :: {}",
            finding.id, finding.contract_name, finding.title
        );
        if let Some(function_name) = &finding.function_name {
            println!("  function: {}", function_name);
        }
        for evidence in &finding.evidence {
            println!("  evidence: {}", evidence);
        }
    }

    println!("\nJSON:");
    match serde_json::to_string_pretty(&findings) {
        Ok(json) => println!("{}", json),
        Err(err) => {
            eprintln!("failed to serialize findings: {}", err);
            std::process::exit(1);
        }
    }
}
