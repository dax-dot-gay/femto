use femto_common::{Error, config::TemplateConfig, schemars::schema_for};
use std::{path::PathBuf, str::FromStr};

use clap::ArgMatches;

pub struct HandleSchema {
    template_path: PathBuf,
    project_path: PathBuf,
}

impl HandleSchema {
    pub fn new(matches: &ArgMatches) -> Self {
        Self {
            template_path: matches
                .get_one::<PathBuf>("config")
                .unwrap_or(&PathBuf::from_str("femto-template.schema.json").unwrap())
                .clone(),
            project_path: matches
                .get_one::<PathBuf>("project")
                .unwrap_or(&PathBuf::from_str("femto-project.schema.json").unwrap())
                .clone(),
        }
    }

    pub fn handle(&self) -> femto_common::FResult<()> {
        let template_schema = schema_for!(TemplateConfig);
        std::fs::write(
            self.template_path.clone(),
            serde_json::to_vec_pretty(&template_schema)
                .or_else(|e| Err(Error::serialization(e)))?,
        )
        .or_else(|e| Err(Error::io(e)))?;

        Ok(())
    }
}
