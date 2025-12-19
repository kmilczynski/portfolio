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
    pub fn load() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let site_content = include_str!("../content/site.toml");
        let config: SiteConfig = toml::from_str(&site_content)?;
        Ok(config)
    }
}
