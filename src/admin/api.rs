use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;

use crate::admin::db::DbPool;
use crate::admin::models::{DbPost, PostApiResponse};

#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    pub locale: Option<String>,
    pub status: Option<String>,
}

fn render_markdown(markdown: &str) -> String {
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

pub async fn list_posts_api(
    State(pool): State<DbPool>,
    Query(query): Query<ListPostsQuery>,
) -> impl IntoResponse {
    let locale = query.locale.as_deref().unwrap_or("en");

    let posts: Vec<DbPost> = match &query.status {
        Some(status) => {
            sqlx::query_as("SELECT * FROM posts WHERE status = ? ORDER BY date DESC")
                .bind(status)
                .fetch_all(&pool)
                .await
        }
        None => {
            sqlx::query_as("SELECT * FROM posts ORDER BY date DESC")
                .fetch_all(&pool)
                .await
        }
    }
    .unwrap_or_default();

    let response: Vec<PostApiResponse> = posts
        .iter()
        .map(|post| {
            let content = post.content(locale);
            PostApiResponse {
                slug: post.slug.clone(),
                title: post.title(locale).to_string(),
                excerpt: post.excerpt(locale).to_string(),
                content: content.to_string(),
                html_content: render_markdown(content),
                date: post.date.clone(),
                status: post.status.clone(),
                tags: post.tags_vec(),
                image: post.image.clone(),
            }
        })
        .collect();

    Json(response)
}

#[derive(Debug, Deserialize)]
pub struct GetPostQuery {
    pub locale: Option<String>,
}

pub async fn get_post_api(
    State(pool): State<DbPool>,
    Path(slug): Path<String>,
    Query(query): Query<GetPostQuery>,
) -> impl IntoResponse {
    let locale = query.locale.as_deref().unwrap_or("en");

    let post: Option<DbPost> = sqlx::query_as("SELECT * FROM posts WHERE slug = ?")
        .bind(&slug)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten();

    match post {
        Some(post) => {
            let content = post.content(locale);
            let response = PostApiResponse {
                slug: post.slug.clone(),
                title: post.title(locale).to_string(),
                excerpt: post.excerpt(locale).to_string(),
                content: content.to_string(),
                html_content: render_markdown(content),
                date: post.date.clone(),
                status: post.status.clone(),
                tags: post.tags_vec(),
                image: post.image.clone(),
            };
            (StatusCode::OK, Json(Some(response))).into_response()
        }
        None => (StatusCode::NOT_FOUND, Json::<Option<PostApiResponse>>(None)).into_response(),
    }
}
