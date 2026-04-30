use crate::finding::Finding;
use crate::model::{AstRoot, ContractNode, Expression, ModifierName, SourceUnitNode, Statement};

pub fn run_all_rules(ast: &AstRoot) -> Vec<Finding> {
    let mut findings = Vec::new();
    findings.extend(detect_initializer_exposure(ast));
    findings.extend(detect_unlocked_implementation_candidate(ast));
    findings
}

pub fn detect_initializer_exposure(ast: &AstRoot) -> Vec<Finding> {
    // UPG-001 탐지 결과를 누적할 목록이다.
    let mut findings = Vec::new();

    // 최상위 AST 노드 중 컨트랙트 정의만 분석한다.
    for node in &ast.nodes {
        let SourceUnitNode::ContractDefinition(contract) = node else {
            continue;
        };

        // 컨트랙트 내부 노드 중 함수 정의만 분석한다.
        for inner in &contract.nodes {
            let ContractNode::FunctionDefinition(func) = inner else {
                continue;
            };

            // initialize/init 이름을 가진 함수만 initializer 후보로 본다.
            let function_name_lower = func.name.trim().to_lowercase();
            if function_name_lower != "initialize" && function_name_lower != "init" {
                continue;
            }

            // visibility가 AST에 없으면 unknown으로 처리해 과도한 확정을 피한다.
            let visibility = func
                .visibility
                .clone()
                .unwrap_or_else(|| "unknown".to_string());

            // modifier 호출 중 Identifier 형태로 표현된 이름만 추출한다.
            let modifier_names: Vec<String> = func
                .modifiers
                .iter()
                .filter_map(|m| match &m.modifier_name {
                    Some(ModifierName::Identifier { name }) => Some(name.clone()),
                    _ => None,
                })
                .collect();

            // OpenZeppelin 계열에서 흔히 쓰는 initializer/reinitializer modifier를 보호 신호로 본다.
            let has_initializer_modifier = modifier_names.iter().any(|m| {
                let lower = m.to_lowercase();
                lower == "initializer" || lower == "reinitializer"
            });

            // 외부에서 호출 가능한 initializer 후보가 modifier 없이 열려 있으면 더 높은 위험도로 본다.
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

            // modifier 존재 여부에 따라 finding 제목과 설명을 다르게 구성한다.
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

            // 리포트에서 사람이 판정 근거를 추적할 수 있도록 핵심 증거를 남긴다.
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

            // 하나의 initializer 후보를 UPG-001 finding으로 기록한다.
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
