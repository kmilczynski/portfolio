use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub site: SiteInfo,
    pub social: SocialLinks,
    pub about: AboutSection,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfo {
    pub title: String,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLinks {
    pub github: String,
    pub linkedin: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AboutSection {
    pub greeting: String,
    pub name: String,
    pub tagline: String,
    pub bio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub name: String,
    pub technologies: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub period: String,
    pub description: String,
    pub stack: String,
    pub current: bool,
}

impl SiteConfig {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_for_locale(locale: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        use std::fs;
        use std::path::PathBuf;

        let file_name = format!("site-{}.toml", locale);
        let file_path = PathBuf::from("src/content").join(&file_name);

        let site_content = fs::read_to_string(&file_path)?;
        let config: SiteConfig = toml::from_str(&site_content)?;
        Ok(config)
    }
}
