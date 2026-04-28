use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AstRoot {
    pub nodes: Vec<SourceUnitNode>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum SourceUnitNode {
    ContractDefinition(ContractDefinition),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct ContractDefinition {
    pub name: String,
    pub nodes: Vec<ContractNode>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ContractNode {
    FunctionDefinition(FunctionDefinition),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub modifiers: Vec<ModifierInvocation>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierInvocation {
    #[serde(default)]
    pub modifierName: Option<ModifierName>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ModifierName {
    Identifier {
        name: String,
    },
    #[serde(other)]
    Other,
}
