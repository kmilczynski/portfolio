use perseus::prelude::UnreactiveState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostSeo {
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostFrontmatter {
    pub title: String,
    pub slug: String,
    pub date: String,
    pub status: String,
    pub excerpt: String,
    pub tags: Vec<String>,
    pub image: Option<String>,
    pub seo: Option<PostSeo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, UnreactiveState)]
pub struct Post {
    pub frontmatter: PostFrontmatter,
    pub content: String,
    pub html_content: String,
}

impl Post {
    pub fn reading_time(&self) -> u32 {
        let word_count = self.content.split_whitespace().count();
        let minutes = (word_count / 200).max(1) as u32;
        minutes
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod loader {
    use pulldown_cmark::{html, Options, Parser};

    pub fn render_markdown(markdown: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }
}
