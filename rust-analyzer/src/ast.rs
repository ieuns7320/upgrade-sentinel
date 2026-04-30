use std::fs;

use crate::model::{AstRoot, ContractNode, SourceUnitNode};

pub fn load_ast(path: &str) -> Result<AstRoot, String> {
    // 지정된 경로의 AST JSON 파일을 문자열로 읽는다.
    let content =
        fs::read_to_string(path).map_err(|e| format!("failed to read '{}': {}", path, e))?;

    // JSON 내용을 분석기가 사용하는 최소 AST 모델로 변환한다.
    serde_json::from_str::<AstRoot>(&content)
        .map_err(|e| format!("failed to parse JSON '{}': {}", path, e))
}

/*
pub fn print_contracts_and_functions(ast: &AstRoot) {
    // AST 최상위 노드 중 컨트랙트 정의만 골라 이름을 출력한다.
    for node in &ast.nodes {
        if let SourceUnitNode::ContractDefinition(contract) = node {
            println!("Contract: {}", contract.name);

            // 컨트랙트 내부 노드 중 함수 정의만 골라 함수명을 출력한다.
            for inner in &contract.nodes {
                if let ContractNode::FunctionDefinition(func) = inner {
                    // constructor/fallback/receive는 AST에서 name이 비어 있을 수 있어 kind로 보정한다.
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
*/
