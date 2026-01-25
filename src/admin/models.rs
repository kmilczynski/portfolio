use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbPost {
    pub id: i64,
    pub slug: String,
    pub title_pl: String,
    pub excerpt_pl: String,
    pub content_pl: String,
    pub title_en: String,
    pub excerpt_en: String,
    pub content_en: String,
    pub date: String,
    pub status: String,
    pub tags: String,
    pub image: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl DbPost {
    pub fn tags_vec(&self) -> Vec<String> {
        serde_json::from_str(&self.tags).unwrap_or_default()
    }

    pub fn title(&self, locale: &str) -> &str {
        match locale {
            "en" => &self.title_en,
            _ => &self.title_pl,
        }
    }

    pub fn excerpt(&self, locale: &str) -> &str {
        match locale {
            "en" => &self.excerpt_en,
            _ => &self.excerpt_pl,
        }
    }

    pub fn content(&self, locale: &str) -> &str {
        match locale {
            "en" => &self.content_en,
            _ => &self.content_pl,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostApiResponse {
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub html_content: String,
    pub date: String,
    pub status: String,
    pub tags: Vec<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostForm {
    pub slug: String,
    pub title_pl: String,
    pub excerpt_pl: String,
    pub content_pl: String,
    pub title_en: String,
    pub excerpt_en: String,
    pub content_en: String,
    pub date: String,
    pub status: String,
    pub tags: String,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostForm {
    pub slug: String,
    pub title_pl: String,
    pub excerpt_pl: String,
    pub content_pl: String,
    pub title_en: String,
    pub excerpt_en: String,
    pub content_en: String,
    pub date: String,
    pub status: String,
    pub tags: String,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginForm {
    pub password: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AdminSession {
    pub id: String,
    pub created_at: String,
    pub expires_at: String,
}
