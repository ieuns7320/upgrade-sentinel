use crate::finding::Finding;
use crate::model::{AstRoot, ContractNode, Expression, ModifierName, SourceUnitNode, Statement};

pub fn run_all_rules(ast: &AstRoot) -> Vec<Finding> {
    let mut findings = Vec::new();
    findings.extend(detect_initializer_exposure(ast));
    findings.extend(detect_unlocked_implementation_candidate(ast));
    findings.extend(detect_upgrade_access_control_candidate(ast));
    findings.extend(detect_low_level_call_candidates(ast));
    findings.extend(detect_destructive_call_candidates(ast));
    findings
}

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

            let function_name_lower = func.name.trim().to_lowercase();
            if function_name_lower != "initialize" && function_name_lower != "init" {
                continue;
            }

            let visibility = func
                .visibility
                .clone()
                .unwrap_or_else(|| "unknown".to_string());

            let modifier_names: Vec<String> = func
                .modifiers
                .iter()
                .filter_map(|m| match &m.modifier_name {
                    Some(ModifierName::Identifier { name }) => Some(name.clone()),
                    _ => None,
                })
                .collect();

            let has_initializer_modifier = modifier_names.iter().any(|m| {
                let lower = m.to_lowercase();
                lower == "initializer" || lower == "reinitializer"
            });

            let severity = match visibility.as_str() {
                "public" | "external" => {
                    if has_initializer_modifier {
                        "LOW"
                    } else {
                        "HIGH"
                    }
                }
                "internal" | "private" => "LOW",
                _ => "MEDIUM",
            };

            let title = if has_initializer_modifier {
                "Initializer-like function found with initializer-style modifier"
            } else {
                "Initializer-like function may be insufficiently protected"
            };

            let description = if has_initializer_modifier {
                "A function named initialize/init was found with an initializer-like modifier. Review whether initialization is properly restricted and whether the implementation contract is locked."
            } else {
                "A function named initialize/init was found without a recognized initializer-style modifier. Review whether the function is externally reachable and properly protected."
            };

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

            if has_initializer_modifier {
                evidence.push("Initializer-style modifier detected".to_string());
            } else {
                evidence.push("Initializer-style modifier not detected".to_string());
            }

            findings.push(Finding {
                id: "UPG-001".to_string(),
                severity: severity.to_string(),
                title: title.to_string(),
                description: description.to_string(),
                contract_name: contract.name.clone(),
                function_name: Some(func.name.clone()),
                evidence,
            });
        }
    }

    findings
}

pub fn detect_unlocked_implementation_candidate(ast: &AstRoot) -> Vec<Finding> {
    let mut findings = Vec::new();

    for node in &ast.nodes {
        let SourceUnitNode::ContractDefinition(contract) = node else {
            continue;
        };

        let mut has_initializer_like_function = false;
        let mut constructor_found = false;
        let mut disable_initializers_found = false;

        for inner in &contract.nodes {
            let ContractNode::FunctionDefinition(func) = inner else {
                continue;
            };

            let function_name_lower = func.name.trim().to_lowercase();

            if function_name_lower == "initialize" || function_name_lower == "init" {
                has_initializer_like_function = true;
            }

            if matches!(func.kind.as_deref(), Some("constructor")) {
                constructor_found = true;

                if let Some(body) = &func.body {
                    for stmt in &body.statements {
                        let Statement::ExpressionStatement(expr_stmt) = stmt else {
                            continue;
                        };

                        let Expression::FunctionCall(call) = &expr_stmt.expression else {
                            continue;
                        };

                        match call.expression.as_ref() {
                            Expression::Identifier { name } if name == "_disableInitializers" => {
                                disable_initializers_found = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if has_initializer_like_function && !disable_initializers_found {
            let mut evidence = vec![
                format!("Contract '{}'", contract.name),
                "Initializer-like function detected in contract".to_string(),
            ];

            if constructor_found {
                evidence.push("Constructor found".to_string());
            } else {
                evidence.push("Constructor not found".to_string());
            }

            evidence.push(if disable_initializers_found {
                "_disableInitializers() call detected in constructor".to_string()
            } else {
                "_disableInitializers() call not detected in constructor".to_string()
            });

            findings.push(Finding {
                id: "UPG-002".to_string(),
                severity: "HIGH".to_string(),
                title: "Implementation lock may be missing".to_string(),
                description: "An initializer-like function was found, but a constructor-level _disableInitializers() pattern was not detected. Review whether the implementation contract is properly locked.".to_string(),
                contract_name: contract.name.clone(),
                function_name: None,
                evidence,
            });
        }
    }

    findings
}

pub fn detect_upgrade_access_control_candidate(ast: &AstRoot) -> Vec<Finding> {
    let mut findings = Vec::new();

    for node in &ast.nodes {
        let SourceUnitNode::ContractDefinition(contract) = node else {
            continue;
        };

        for inner in &contract.nodes {
            let ContractNode::FunctionDefinition(func) = inner else {
                continue;
            };

            let function_name_lower = func.name.trim().to_lowercase();

            let is_upgrade_like = matches!(
                function_name_lower.as_str(),
                "upgrade" | "upgradeto" | "upgradetoandcall" | "setimplementation"
            );

            if !is_upgrade_like {
                continue;
            }

            let visibility = func
                .visibility
                .clone()
                .unwrap_or_else(|| "unknown".to_string());

            let modifier_names: Vec<String> = func
                .modifiers
                .iter()
                .filter_map(|m| match &m.modifier_name {
                    Some(ModifierName::Identifier { name }) => Some(name.clone()),
                    _ => None,
                })
                .collect();

            let has_access_control_modifier = modifier_names.iter().any(|m| {
                let lower = m.to_lowercase();
                lower.contains("onlyowner")
                    || lower.contains("onlyadmin")
                    || lower.contains("adminonly")
                    || lower.contains("onlyrole")
                    || lower.contains("auth")
            });

            let externally_reachable = matches!(visibility.as_str(), "public" | "external");

            if externally_reachable && !has_access_control_modifier {
                let mut evidence = vec![
                    format!("Contract '{}'", contract.name),
                    format!("Function '{}'", func.name),
                    "Name matches upgrade-like sensitive function pattern".to_string(),
                    format!("Visibility '{}'", visibility),
                ];

                if modifier_names.is_empty() {
                    evidence.push("No recognized modifier names found".to_string());
                } else {
                    evidence.push(format!("Modifiers: {}", modifier_names.join(", ")));
                }

                evidence.push("No recognized access-control modifier detected".to_string());

                findings.push(Finding {
                    id: "UPG-003".to_string(),
                    severity: "HIGH".to_string(),
                    title: "Upgrade-like function may lack access control".to_string(),
                    description: "An externally reachable upgrade-like function was found without a recognized access-control modifier. Review whether upgrade authorization is properly enforced.".to_string(),
                    contract_name: contract.name.clone(),
                    function_name: Some(func.name.clone()),
                    evidence,
                });
            }
        }
    }

    findings
}

pub fn detect_low_level_call_candidates(ast: &AstRoot) -> Vec<Finding> {
    let mut findings = Vec::new();

    for node in &ast.nodes {
        let SourceUnitNode::ContractDefinition(contract) = node else {
            continue;
        };

        for inner in &contract.nodes {
            let ContractNode::FunctionDefinition(func) = inner else {
                continue;
            };

            let Some(body) = &func.body else {
                continue;
            };

            for stmt in &body.statements {
                let Statement::ExpressionStatement(expr_stmt) = stmt else {
                    continue;
                };

                let Expression::FunctionCall(call) = &expr_stmt.expression else {
                    continue;
                };

                let low_level_member = match call.expression.as_ref() {
                    Expression::MemberAccess(member)
                        if member.member_name == "delegatecall" || member.member_name == "call" =>
                    {
                        Some(member.member_name.clone())
                    }
                    _ => None,
                };

                let Some(member_name) = low_level_member else {
                    continue;
                };

                let severity = if member_name == "delegatecall" {
                    "HIGH"
                } else {
                    "MEDIUM"
                };

                findings.push(Finding {
                    id: "UPG-004".to_string(),
                    severity: severity.to_string(),
                    title: format!("Low-level {} usage detected", member_name),
                    description: format!(
                        "A low-level {} usage pattern was detected. Review whether the target, input data, return handling, and authorization checks are safe.",
                        member_name
                    ),
                    contract_name: contract.name.clone(),
                    function_name: Some(func.name.clone()),
                    evidence: vec![
                        format!("Contract '{}'", contract.name),
                        format!("Function '{}'", func.name),
                        format!("Low-level member access '{}'", member_name),
                    ],
                });
            }
        }
    }

    findings
}

pub fn detect_destructive_call_candidates(ast: &AstRoot) -> Vec<Finding> {
    let mut findings = Vec::new();

    for node in &ast.nodes {
        let SourceUnitNode::ContractDefinition(contract) = node else {
            continue;
        };

        for inner in &contract.nodes {
            let ContractNode::FunctionDefinition(func) = inner else {
                continue;
            };

            let Some(body) = &func.body else {
                continue;
            };

            for stmt in &body.statements {
                let Statement::ExpressionStatement(expr_stmt) = stmt else {
                    continue;
                };

                let Expression::FunctionCall(call) = &expr_stmt.expression else {
                    continue;
                };

                let destructive_name = match call.expression.as_ref() {
                    Expression::Identifier { name }
                        if name == "selfdestruct" || name == "suicide" =>
                    {
                        Some(name.clone())
                    }
                    _ => None,
                };

                let Some(name) = destructive_name else {
                    continue;
                };

                findings.push(Finding {
                    id: "UPG-005".to_string(),
                    severity: "HIGH".to_string(),
                    title: format!("Destructive {} usage detected", name),
                    description: format!(
                        "A destructive {} usage pattern was detected. Review whether contract destruction is intended, authorized, and safe in the upgradeable architecture.",
                        name
                    ),
                    contract_name: contract.name.clone(),
                    function_name: Some(func.name.clone()),
                    evidence: vec![
                        format!("Contract '{}'", contract.name),
                        format!("Function '{}'", func.name),
                        format!("Destructive call '{}'", name),
                    ],
                });
            }
        }
    }

    findings
}
