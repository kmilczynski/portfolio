use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub image: String,
    pub tags: Vec<String>,
    pub github: String,
    pub demo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsConfig {
    pub projects: Option<Vec<Project>>,
}

impl ProjectsConfig {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = std::fs::read_to_string("content/projects.toml")?;
        let config: ProjectsConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
