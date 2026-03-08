use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::TimeZone;
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

pub async fn rss_feed(State(pool): State<DbPool>) -> Response<String> {
    let posts: Vec<DbPost> = sqlx::query_as(
        "SELECT * FROM posts WHERE status = 'published' ORDER BY date DESC LIMIT 20"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let base_url = "https://kmilczynski.byst.re";
    let now = chrono::Utc::now().to_rfc2822();

    let mut items = String::new();
    for post in posts {
        let title = post.title("en");
        let description = post.excerpt("en");
        let link = format!("{}/blog/{}", base_url, post.slug);
        let pub_date = chrono::NaiveDate::parse_from_str(&post.date, "%Y-%m-%d")
            .ok()
            .and_then(|d| d.and_hms_opt(12, 0, 0))
            .map(|dt| chrono::Utc.from_utc_datetime(&dt).to_rfc2822())
            .unwrap_or_else(|| now.clone());

        items.push_str(&format!(
            r#"    <item>
      <title>{}</title>
      <link>{}</link>
      <guid>{}</guid>
      <pubDate>{}</pubDate>
      <description>{}</description>
    </item>
"#,
            escape_xml(title),
            link,
            link,
            pub_date,
            escape_xml(description)
        ));
    }

    let rss = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>Kacper Milczyński | Software Developer</title>
    <link>{}/blog</link>
    <description>Software developer specializing in backend systems, real-time applications, and exploring the Rust ecosystem.</description>
    <language>en</language>
    <lastBuildDate>{}</lastBuildDate>
    <atom:link href="{}/feed.xml" rel="self" type="application/rss+xml"/>
{}  </channel>
</rss>"#,
        base_url, now, base_url, items
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")
        .body(rss)
        .unwrap()
}

pub async fn sitemap_xml(State(pool): State<DbPool>) -> Response<String> {
    let posts: Vec<DbPost> = sqlx::query_as(
        "SELECT * FROM posts WHERE status = 'published' ORDER BY date DESC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let base_url = "https://kmilczynski.byst.re";

    let mut urls = String::new();

    // Static pages
    urls.push_str(&format!(
        r#"  <url>
    <loc>{}/</loc>
    <changefreq>monthly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>{}/en</loc>
    <changefreq>monthly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>{}/projects</loc>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>{}/en/projects</loc>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>{}/blog</loc>
    <changefreq>daily</changefreq>
    <priority>0.9</priority>
  </url>
  <url>
    <loc>{}/en/blog</loc>
    <changefreq>daily</changefreq>
    <priority>0.9</priority>
  </url>
"#,
        base_url, base_url, base_url, base_url, base_url, base_url
    ));

    // Blog posts (Polish and English versions)
    for post in posts {
        let lastmod = post.updated_at.split('T').next().unwrap_or(&post.date);

        urls.push_str(&format!(
            r#"  <url>
    <loc>{}/blog/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.7</priority>
  </url>
  <url>
    <loc>{}/en/blog/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.7</priority>
  </url>
"#,
            base_url, post.slug, lastmod, base_url, post.slug, lastmod
        ));
    }

    let sitemap = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>"#,
        urls
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(sitemap)
        .unwrap()
}

pub async fn robots_txt() -> Response<String> {
    let robots = r#"User-agent: *
Allow: /

Sitemap: https://kmilczynski.byst.re/sitemap.xml
"#;

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(robots.to_string())
        .unwrap()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
