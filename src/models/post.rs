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
    use super::*;
    use pulldown_cmark::{html, Options, Parser};
    use std::fs;
    use std::path::Path;

    pub fn parse_post_file(
        file_path: &Path,
    ) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
        let raw = fs::read_to_string(file_path)?;

        let parts: Vec<&str> = raw.splitn(3, "+++").collect();

        if parts.len() != 3 {
            return Err("Invalid frontmatter format".into());
        }

        let frontmatter_str = parts[1].trim();
        let content = parts[2].trim();

        let frontmatter: PostFrontmatter = toml::from_str(frontmatter_str)?;
        let html_content = render_markdown(content);

        Ok(Post {
            frontmatter,
            content: content.to_string(),
            html_content,
        })
    }

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

    pub fn load_all_posts_for_locale(
        posts_dir: &Path,
        locale: Option<&str>,
    ) -> Result<Vec<Post>, Box<dyn std::error::Error + Send + Sync>> {
        let mut posts = Vec::new();

        if !posts_dir.exists() {
            return Ok(posts);
        }

        // If locale is specified, look in locale subdirectory
        let search_dir = if let Some(loc) = locale {
            posts_dir.join(loc)
        } else {
            posts_dir.to_path_buf()
        };

        if !search_dir.exists() {
            return Ok(posts);
        }

        for entry in fs::read_dir(search_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "md") {
                match parse_post_file(&path) {
                    Ok(post) => {
                        if post.frontmatter.status == "published" {
                            posts.push(post);
                        }
                    }
                    Err(e) => eprintln!("Error parsing {:?}: {}", path, e),
                }
            }
        }

        posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

        Ok(posts)
    }

    pub fn load_post_by_slug_and_locale(
        posts_dir: &Path,
        slug: &str,
        locale: &str,
    ) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
        // Look in locale-specific subdirectory (e.g., "posts/pl/slug.md")
        let locale_file_path = posts_dir.join(locale).join(format!("{}.md", slug));
        if locale_file_path.exists() {
            return parse_post_file(&locale_file_path);
        }

        // Fallback to root directory
        let file_path = posts_dir.join(format!("{}.md", slug));
        parse_post_file(&file_path)
    }
}
