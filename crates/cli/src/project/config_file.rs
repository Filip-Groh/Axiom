use serde::{Deserialize, Serialize};
use crate::ProjectType;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub package: Package,
    pub build: Build
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub r#type: ProjectType
}

#[derive(Serialize, Deserialize)]
pub struct Build {
    pub target: Target,
}

#[derive(Serialize, Deserialize)]
pub enum Target {
    Linux,
    //Windows
}