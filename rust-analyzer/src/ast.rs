use std::fs;

use crate::model::{AstRoot, ContractNode, SourceUnitNode};

pub fn load_ast(path: &str) -> Result<AstRoot, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("failed to read '{}': {}", path, e))?;

    serde_json::from_str::<AstRoot>(&content)
        .map_err(|e| format!("failed to parse JSON '{}': {}", path, e))
}

pub fn print_contracts_and_functions(ast: &AstRoot) {
    for node in &ast.nodes {
        if let SourceUnitNode::ContractDefinition(contract) = node {
            println!("Contract: {}", contract.name);

            for inner in &contract.nodes {
                if let ContractNode::FunctionDefinition(func) = inner {
                    let display_name = if func.name.is_empty() {
                        match func.kind.as_deref() {
                            Some("constructor") => "constructor",
                            Some("fallback") => "fallback",
                            Some("receive") => "receive",
                            _ => "<anonymous>",
                        }
                    } else {
                        &func.name
                    };

                    println!("  - Function: {}", display_name);
                }
            }
        }
    }
}
