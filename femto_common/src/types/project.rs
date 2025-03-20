use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use url::Url;

use crate::Error;

fn is_github(ghs: impl Into<String>) -> bool {
    regex::RegexBuilder::new(r"^[\w\.-]+$")
        .case_insensitive(true)
        .build()
        .unwrap()
        .is_match(&ghs.into())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProjectSource {
    LocalFolder(PathBuf),
    GitRepository(Url),
    GithubRepository(String, String),
}

impl FromStr for ProjectSource {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from_str(s).unwrap();
        if path.exists()
            && (path.join("template.femto.toml").exists() || path.join("template.femto").exists())
        {
            return Ok(Self::LocalFolder(path));
        }

        if let Ok(parsed_url) = Url::parse(s) {
            if let Some(segments) = parsed_url.path_segments() {
                if let Some(filename) = segments.last() {
                    if filename.ends_with(".git") {
                        return Ok(Self::GitRepository(parsed_url));
                    }
                }
            }
        }

        if let Some((org, repo)) = s.split_once("/") {
            if is_github(org.to_string()) && is_github(repo.to_string()) {
                return Ok(Self::GithubRepository(org.to_string(), repo.to_string()));
            }
        }

        Err(Error::ProjectSource(s.to_string()))
    }
}
