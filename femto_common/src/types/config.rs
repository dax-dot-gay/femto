use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TemplateMetadata {
    pub version: Option<String>,
    pub tags: Vec<String>,
    pub languages: Vec<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TemplateField {
    Text {
        key: String,
        prompt: String,
        help: Option<String>,
    },
    Number {
        key: String,
        prompt: String,
        help: Option<String>,
    },
    Confirm {
        key: String,
        prompt: String,
        help: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemplateConfig {
    pub slug: String,
    pub name: String,
    pub metadata: Option<TemplateMetadata>,
}
