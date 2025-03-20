use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct TemplateMetadata {
    pub version: Option<String>,
    pub tags: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum NumberType {
    #[serde(alias = "float")]
    SignedFloat,

    #[serde(alias = "+float")]
    UnsignedFloat,

    #[serde(alias = "int")]
    SignedInt,

    #[serde(alias = "+int")]
    UnsignedInt,
}

impl Default for NumberType {
    fn default() -> Self {
        Self::SignedFloat
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TemplateField {
    Text {
        key: String,
        prompt: String,
        #[serde(default)]
        help: Option<String>,

        #[serde(default)]
        default: Option<String>,

        #[serde(default)]
        placeholder: Option<String>,
    },
    Password {
        key: String,
        prompt: String,
        #[serde(default)]
        help: Option<String>,

        #[serde(default)]
        default: Option<String>,

        #[serde(default)]
        placeholder: Option<String>,
    },
    Number {
        key: String,
        prompt: String,

        #[serde(default)]
        help: Option<String>,

        #[serde(default)]
        default: Option<f64>,

        #[serde(default)]
        mode: NumberType,

        #[serde(default)]
        min: Option<f64>,

        #[serde(default)]
        max: Option<f64>,
    },
    Boolean {
        key: String,
        prompt: String,

        #[serde(default)]
        help: Option<String>,

        #[serde(default)]
        default: bool,
    },
    Select {
        key: String,
        prompt: String,
        options: Vec<String>,

        #[serde(default)]
        help: Option<String>,

        #[serde(default)]
        default: Option<String>,
    },
    SelectMultiple {
        key: String,
        prompt: String,
        options: Vec<String>,

        #[serde(default)]
        help: Option<String>,

        #[serde(default)]
        default: Option<Vec<String>>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TemplateCommon {
    key: String,
    prompt: String,

    #[serde(default)]
    help: Option<String>,
}

impl TemplateField {
    pub fn common(&self) -> TemplateCommon {
        match self {
            Self::Text {
                key, prompt, help, ..
            } => TemplateCommon {
                key: key.to_string(),
                prompt: prompt.to_string(),
                help: help.clone(),
            },
            Self::Password {
                key, prompt, help, ..
            } => TemplateCommon {
                key: key.to_string(),
                prompt: prompt.to_string(),
                help: help.clone(),
            },
            Self::Number {
                key, prompt, help, ..
            } => TemplateCommon {
                key: key.to_string(),
                prompt: prompt.to_string(),
                help: help.clone(),
            },
            Self::Boolean {
                key, prompt, help, ..
            } => TemplateCommon {
                key: key.to_string(),
                prompt: prompt.to_string(),
                help: help.clone(),
            },
            Self::Select {
                key, prompt, help, ..
            } => TemplateCommon {
                key: key.to_string(),
                prompt: prompt.to_string(),
                help: help.clone(),
            },
            Self::SelectMultiple {
                key, prompt, help, ..
            } => TemplateCommon {
                key: key.to_string(),
                prompt: prompt.to_string(),
                help: help.clone(),
            }
        }
    }

    pub fn key(&self) -> String {
        self.common().key.clone()
    }

    pub fn prompt(&self) -> String {
        self.common().prompt.clone()
    }

    pub fn help(&self) -> Option<String> {
        self.common().help.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TemplateStage {
    pub key: String,
    pub name: String,
    pub fields: Vec<TemplateField>,
    
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TemplatePackageManager {
    pub name: String,
    pub path: String,
    
    #[serde(alias = "add")]
    pub add_package: Vec<String>,

    #[serde(alias = "remove")]
    pub remove_package: Vec<String>,
    pub help: Vec<String>,
    pub other_commands: HashMap<String, (String, Vec<String>)>
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TemplateConfig {
    pub slug: String,
    pub name: String,

    #[serde(default)]
    pub metadata: Option<TemplateMetadata>,

    #[serde(default)]
    pub stages: Option<Vec<TemplateStage>>,

    #[serde(default)]
    pub package_managers: Option<Vec<TemplatePackageManager>>
}
