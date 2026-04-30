use serde::Deserialize;

// Solidity 컴파일러가 만든 AST JSON의 최상위 구조 중 분석에 필요한 nodes만 표현한다.
#[derive(Debug, Deserialize)]
pub struct AstRoot {
    pub nodes: Vec<SourceUnitNode>,
}

// SourceUnit 하위 노드는 nodeType 값으로 구분한다.
// 현재 분석기는 ContractDefinition만 사용하고, 나머지는 Other로 안전하게 무시한다.
#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum SourceUnitNode {
    ContractDefinition(ContractDefinition),
    #[serde(other)]
    Other,
}

// 컨트랙트 이름과 내부 노드 목록만 보존한다.
#[derive(Debug, Deserialize)]
pub struct ContractDefinition {
    pub name: String,
    pub nodes: Vec<ContractNode>,
}

// 컨트랙트 내부 노드 중 함수 정의만 분석 대상으로 삼는다.
#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ContractNode {
    FunctionDefinition(FunctionDefinition),
    #[serde(other)]
    Other,
}

// 함수명, 함수 종류, visibility, modifier 목록처럼 규칙 판정에 필요한 필드만 정의한다.
#[derive(Debug, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub modifiers: Vec<ModifierInvocation>,
    #[serde(default)]
    pub body: Option<Block>,
}

// Solidity AST의 modifierName 필드를 Rust 네이밍 규칙에 맞게 modifier_name으로 받는다.
#[derive(Debug, Deserialize)]
pub struct ModifierInvocation {
    #[serde(default, rename = "modifierName")]
    pub modifier_name: Option<ModifierName>,
}

// modifier 이름이 Identifier 형태일 때만 실제 이름을 추출하고, 다른 형태는 무시한다.
#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ModifierName {
    Identifier {
        name: String,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    #[serde(default)]
    pub statements: Vec<Statement>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Statement {
    ExpressionStatement(ExpressionStatement),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Expression {
    FunctionCall(FunctionCall),
    Identifier {
        name: String,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct FunctionCall {
    pub expression: Box<Expression>,
}
