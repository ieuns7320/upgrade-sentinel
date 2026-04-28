use crate::finding::Finding;
use crate::model::{AstRoot, ContractNode, ModifierName, SourceUnitNode};

pub fn detect_initializer_exposure(ast: &AstRoot) -> Vec<Finding> {
    let mut findings = Vec::new();

    for node in &ast.nodes {
        let SourceUnitNode::ContractDefinition(contract) = node else {
            continue;
        };

        for inner in &contract.nodes {
            let ContractNode::FunctionDefinition(func) = inner else {
                continue;
            };

            let name = func.name.trim().to_lowercase();

            if name == "initialize" || name == "init" {
                let visibility = func
                    .visibility
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string());

                let modifier_names: Vec<String> = func
                    .modifiers
                    .iter()
                    .filter_map(|m| match &m.modifierName {
                        Some(ModifierName::Identifier { name }) => Some(name.clone()),
                        _ => None,
                    })
                    .collect();

                let mut evidence = vec![
                    format!("Contract '{}'", contract.name),
                    format!("Function '{}'", func.name),
                    "Name matches initializer-like pattern".to_string(),
                    format!("Visibility '{}'", visibility),
                ];

                if modifier_names.is_empty() {
                    evidence.push("No recognized modifier names found".to_string());
                } else {
                    evidence.push(format!("Modifiers: {}", modifier_names.join(", ")));
                }

                findings.push(Finding {
                    id: "UPG-001".to_string(),
                    severity: "MEDIUM".to_string(),
                    title: "Initializer-like function is externally visible candidate".to_string(),
                    description: "A function named initialize/init was found. This may indicate an initializer exposure risk and should be reviewed.".to_string(),
                    contract_name: contract.name.clone(),
                    function_name: Some(func.name.clone()),
                    evidence,
                });
            }
        }
    }

    findings
}
