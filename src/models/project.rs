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
    pub fn load_for_locale(locale: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        use std::fs;
        use std::path::PathBuf;

        let file_name = format!("projects-{}.toml", locale);
        let file_path = PathBuf::from("src/content").join(&file_name);

        let projects_content = fs::read_to_string(&file_path)?;
        let config: ProjectsConfig = toml::from_str(&projects_content)?;
        Ok(config)
    }
}
