use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use html_escape::encode_text;

use crate::admin::auth::{create_session, delete_session, verify_password};
use crate::admin::db::DbPool;
use crate::admin::models::{CreatePostForm, DbPost, LoginForm, UpdatePostForm};

fn admin_layout(title: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en" class="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} | Admin</title>
    <link href="/.perseus/static/css/main.css" rel="stylesheet">
    <style>
        .admin-btn {{
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.625rem 1.25rem;
            background: #34d399;
            color: #0f0f0f;
            font-weight: 600;
            border-radius: 0.5rem;
            text-decoration: none;
            transition: all 0.2s;
            border: none;
            cursor: pointer;
        }}
        .admin-btn:hover {{
            background: #10b981;
            transform: translateY(-1px);
        }}
        .admin-btn-outline {{
            background: transparent;
            border: 1px solid #323232;
            color: #d1d5db;
        }}
        .admin-btn-outline:hover {{
            border-color: #6b7280;
            background: rgba(50, 50, 50, 0.3);
        }}
        .admin-btn-danger {{
            background: transparent;
            color: #f87171;
        }}
        .admin-btn-danger:hover {{
            background: rgba(248, 113, 113, 0.1);
        }}
        .admin-input {{
            width: 100%;
            padding: 0.75rem 1rem;
            background: #1a1a1a;
            border: 1px solid #323232;
            border-radius: 0.5rem;
            color: #f4e5c2;
            font-size: 0.95rem;
            transition: border-color 0.2s;
        }}
        .admin-input:focus {{
            outline: none;
            border-color: #34d399;
        }}
        .admin-input::placeholder {{
            color: #6b7280;
        }}
        .admin-label {{
            display: block;
            font-size: 0.875rem;
            font-weight: 500;
            color: #9ca3af;
            margin-bottom: 0.5rem;
        }}
        .admin-card {{
            background: #1a1a1a;
            border: 1px solid #323232;
            border-radius: 0.75rem;
            overflow: hidden;
        }}
        .admin-table {{
            width: 100%;
            border-collapse: collapse;
        }}
        .admin-table th {{
            padding: 1rem;
            text-align: left;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: #9ca3af;
            background: rgba(50, 50, 50, 0.5);
            border-bottom: 1px solid #323232;
        }}
        .admin-table td {{
            padding: 1rem;
            border-bottom: 1px solid #323232;
            color: #d1d5db;
        }}
        .admin-table tr:hover td {{
            background: rgba(50, 50, 50, 0.3);
        }}
        .admin-table tr:last-child td {{
            border-bottom: none;
        }}
        .status-badge {{
            display: inline-block;
            padding: 0.25rem 0.75rem;
            font-size: 0.75rem;
            font-weight: 500;
            border-radius: 9999px;
        }}
        .status-published {{
            background: rgba(34, 197, 94, 0.15);
            color: #4ade80;
        }}
        .status-draft {{
            background: rgba(234, 179, 8, 0.15);
            color: #facc15;
        }}
        .admin-nav {{
            background: #1a1a1a;
            border-bottom: 1px solid #323232;
            padding: 1rem 1.5rem;
        }}
        .admin-nav-inner {{
            max-width: 72rem;
            margin: 0 auto;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }}
        .admin-nav-logo {{
            color: #34d399;
            font-family: "JetBrains Mono", monospace;
            font-size: 1.125rem;
            font-weight: 600;
            text-decoration: none;
        }}
        .admin-nav-links {{
            display: flex;
            align-items: center;
            gap: 1.5rem;
        }}
        .admin-nav-link {{
            color: #9ca3af;
            text-decoration: none;
            transition: color 0.2s;
        }}
        .admin-nav-link:hover {{
            color: #f4e5c2;
        }}
        .section-card {{
            background: #1a1a1a;
            border: 1px solid #323232;
            border-radius: 0.75rem;
            padding: 1.5rem;
            margin-bottom: 1.5rem;
        }}
        .section-title {{
            font-size: 1.125rem;
            font-weight: 600;
            color: #f4e5c2;
            margin-bottom: 1.25rem;
            padding-bottom: 0.75rem;
            border-bottom: 1px solid #323232;
        }}
        .form-grid {{
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 1rem;
        }}
        .form-grid-full {{
            grid-column: span 2;
        }}
        .empty-state {{
            text-align: center;
            padding: 4rem 2rem;
            color: #6b7280;
        }}
        .empty-state svg {{
            width: 4rem;
            height: 4rem;
            margin: 0 auto 1rem;
            opacity: 0.5;
        }}
        .page-header {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 2rem;
        }}
        .page-title {{
            font-size: 1.75rem;
            font-weight: 600;
            color: #f4e5c2;
        }}
        textarea.admin-input {{
            min-height: 8rem;
            resize: vertical;
            font-family: "JetBrains Mono", monospace;
            font-size: 0.875rem;
            line-height: 1.6;
        }}
        .action-link {{
            color: #34d399;
            text-decoration: none;
            font-size: 0.875rem;
        }}
        .action-link:hover {{
            text-decoration: underline;
        }}
        .action-link-danger {{
            color: #f87171;
        }}
    </style>
</head>
<body style="background: #0f0f0f; color: #d1d5db; min-height: 100vh;">
    <nav class="admin-nav">
        <div class="admin-nav-inner">
            <a href="/admin" class="admin-nav-logo">Admin Panel</a>
            <div class="admin-nav-links">
                <a href="/admin/posts" class="admin-nav-link">Posts</a>
                <a href="/" class="admin-nav-link" target="_blank">View Site</a>
                <form action="/admin/logout" method="post" style="display: inline;">
                    <button type="submit" class="admin-nav-link" style="background: none; border: none; cursor: pointer; font: inherit;">Logout</button>
                </form>
            </div>
        </div>
    </nav>
    <main style="max-width: 72rem; margin: 0 auto; padding: 2rem 1.5rem;">
        {content}
    </main>
</body>
</html>"#,
        title = encode_text(title),
        content = content
    )
}

fn login_page(error: Option<&str>) -> String {
    let error_html = error
        .map(|e| format!(r#"<div style="background: rgba(239, 68, 68, 0.1); border: 1px solid #dc2626; color: #fca5a5; padding: 0.75rem 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem;">{}</div>"#, encode_text(e)))
        .unwrap_or_default();

    format!(
        r#"<!DOCTYPE html>
<html lang="en" class="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Login | Admin</title>
    <link href="/.perseus/static/css/main.css" rel="stylesheet">
    <style>
        .login-container {{
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            background: #0f0f0f;
        }}
        .login-card {{
            width: 100%;
            max-width: 24rem;
            padding: 2.5rem;
            background: #1a1a1a;
            border: 1px solid #323232;
            border-radius: 1rem;
        }}
        .login-title {{
            font-size: 1.5rem;
            font-weight: 600;
            color: #f4e5c2;
            text-align: center;
            margin-bottom: 2rem;
        }}
        .login-input {{
            width: 100%;
            padding: 0.875rem 1rem;
            background: #0f0f0f;
            border: 1px solid #323232;
            border-radius: 0.5rem;
            color: #f4e5c2;
            font-size: 1rem;
            margin-bottom: 1.5rem;
            transition: border-color 0.2s;
        }}
        .login-input:focus {{
            outline: none;
            border-color: #34d399;
        }}
        .login-btn {{
            width: 100%;
            padding: 0.875rem;
            background: #34d399;
            color: #0f0f0f;
            font-weight: 600;
            font-size: 1rem;
            border: none;
            border-radius: 0.5rem;
            cursor: pointer;
            transition: background 0.2s;
        }}
        .login-btn:hover {{
            background: #10b981;
        }}
        .login-label {{
            display: block;
            font-size: 0.875rem;
            color: #9ca3af;
            margin-bottom: 0.5rem;
        }}
    </style>
</head>
<body>
    <div class="login-container">
        <div class="login-card">
            <h1 class="login-title">Admin Login</h1>
            {error_html}
            <form action="/admin/login" method="post">
                <label class="login-label" for="password">Password</label>
                <input
                    type="password"
                    id="password"
                    name="password"
                    required
                    class="login-input"
                    placeholder="Enter admin password"
                >
                <button type="submit" class="login-btn">Login</button>
            </form>
        </div>
    </div>
</body>
</html>"#,
        error_html = error_html
    )
}

pub async fn login_page_handler() -> Html<String> {
    Html(login_page(None))
}

pub async fn login_handler(
    State(pool): State<DbPool>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> Response {
    if verify_password(&form.password) {
        match create_session(&pool).await {
            Ok(session_id) => {
                let cookie = Cookie::build("admin_session", session_id)
                    .path("/")
                    .http_only(true)
                    .same_site(SameSite::Lax)
                    .max_age(time::Duration::hours(24))
                    .finish();

                (jar.add(cookie), Redirect::to("/admin/posts")).into_response()
            }
            Err(_) => Html(login_page(Some("Failed to create session"))).into_response(),
        }
    } else {
        Html(login_page(Some("Invalid password"))).into_response()
    }
}

pub async fn logout_handler(
    State(pool): State<DbPool>,
    jar: CookieJar,
) -> Response {
    if let Some(cookie) = jar.get("admin_session") {
        let _ = delete_session(&pool, cookie.value()).await;
    }

    let cookie = Cookie::build("admin_session", "")
        .path("/")
        .max_age(time::Duration::seconds(0))
        .finish();

    (jar.remove(cookie), Redirect::to("/admin/login")).into_response()
}

pub async fn dashboard_handler() -> Redirect {
    Redirect::to("/admin/posts")
}

pub async fn posts_list_handler(State(pool): State<DbPool>) -> Response {
    let posts: Vec<DbPost> = match sqlx::query_as(
        "SELECT * FROM posts ORDER BY date DESC"
    )
    .fetch_all(&pool)
    .await
    {
        Ok(posts) => posts,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
                .into_response();
        }
    };

    let content = if posts.is_empty() {
        r#"<div class="page-header">
            <h1 class="page-title">Posts</h1>
            <a href="/admin/posts/new" class="admin-btn">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
                New Post
            </a>
        </div>
        <div class="admin-card">
            <div class="empty-state">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="16" y1="13" x2="8" y2="13"></line><line x1="16" y1="17" x2="8" y2="17"></line><polyline points="10 9 9 9 8 9"></polyline></svg>
                <p style="font-size: 1.125rem; margin-bottom: 0.5rem;">No posts yet</p>
                <p style="font-size: 0.875rem;">Create your first post to get started!</p>
            </div>
        </div>"#.to_string()
    } else {
        let posts_html: String = posts
            .iter()
            .map(|post| {
                let status_class = if post.status == "published" {
                    "status-published"
                } else {
                    "status-draft"
                };

                format!(
                    r#"<tr>
                        <td>
                            <div style="font-weight: 500; color: #f4e5c2;">{}</div>
                            <div style="font-size: 0.75rem; color: #6b7280; font-family: monospace; margin-top: 0.25rem;">{}</div>
                        </td>
                        <td>{}</td>
                        <td><span class="status-badge {}">{}</span></td>
                        <td>
                            <div style="display: flex; gap: 1rem;">
                                <a href="/admin/posts/{}/edit" class="action-link">Edit</a>
                                <form action="/admin/posts/{}/delete" method="post" style="display: inline;" onsubmit="return confirm('Delete this post?')">
                                    <button type="submit" class="action-link action-link-danger" style="background: none; border: none; cursor: pointer; font: inherit;">Delete</button>
                                </form>
                            </div>
                        </td>
                    </tr>"#,
                    encode_text(&post.title_en),
                    encode_text(&post.slug),
                    encode_text(&post.date),
                    status_class,
                    encode_text(&post.status),
                    post.id,
                    post.id
                )
            })
            .collect();

        format!(
            r#"<div class="page-header">
                <h1 class="page-title">Posts</h1>
                <a href="/admin/posts/new" class="admin-btn">
                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
                    New Post
                </a>
            </div>
            <div class="admin-card">
                <table class="admin-table">
                    <thead>
                        <tr>
                            <th>Title</th>
                            <th>Date</th>
                            <th>Status</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        {posts_html}
                    </tbody>
                </table>
            </div>"#,
            posts_html = posts_html
        )
    };

    Html(admin_layout("Posts", &content)).into_response()
}

fn post_form(post: Option<&DbPost>, error: Option<&str>) -> String {
    let (title, action, button_text) = match post {
        Some(p) => ("Edit Post", format!("/admin/posts/{}/edit", p.id), "Update Post"),
        None => ("New Post", "/admin/posts/new".to_string(), "Create Post"),
    };

    let error_html = error
        .map(|e| format!(r#"<div style="background: rgba(239, 68, 68, 0.1); border: 1px solid #dc2626; color: #fca5a5; padding: 0.75rem 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem;">{}</div>"#, encode_text(e)))
        .unwrap_or_default();

    let slug = post.map(|p| &p.slug as &str).unwrap_or("");
    let title_pl = post.map(|p| &p.title_pl as &str).unwrap_or("");
    let excerpt_pl = post.map(|p| &p.excerpt_pl as &str).unwrap_or("");
    let content_pl = post.map(|p| &p.content_pl as &str).unwrap_or("");
    let title_en = post.map(|p| &p.title_en as &str).unwrap_or("");
    let excerpt_en = post.map(|p| &p.excerpt_en as &str).unwrap_or("");
    let content_en = post.map(|p| &p.content_en as &str).unwrap_or("");
    let date = post.map(|p| &p.date as &str).unwrap_or("");
    let status = post.map(|p| &p.status as &str).unwrap_or("draft");
    let tags = post.map(|p| &p.tags as &str).unwrap_or("[]");
    let image = post.and_then(|p| p.image.as_deref()).unwrap_or("");

    let draft_selected = if status == "draft" { "selected" } else { "" };
    let published_selected = if status == "published" { "selected" } else { "" };

    format!(
        r#"<div style="max-width: 56rem;">
            <div class="page-header">
                <h1 class="page-title">{title}</h1>
            </div>

            {error_html}

            <form action="{action}" method="post">
                <!-- Metadata -->
                <div class="section-card">
                    <h2 class="section-title">Post Settings</h2>
                    <div class="form-grid">
                        <div>
                            <label class="admin-label" for="slug">Slug</label>
                            <input type="text" id="slug" name="slug" value="{slug}" required class="admin-input" placeholder="my-post-slug">
                        </div>
                        <div>
                            <label class="admin-label" for="date">Date</label>
                            <input type="date" id="date" name="date" value="{date}" required class="admin-input">
                        </div>
                        <div>
                            <label class="admin-label" for="status">Status</label>
                            <select id="status" name="status" required class="admin-input">
                                <option value="draft" {draft_selected}>Draft</option>
                                <option value="published" {published_selected}>Published</option>
                            </select>
                        </div>
                        <div>
                            <label class="admin-label" for="image">Image URL (optional)</label>
                            <input type="text" id="image" name="image" value="{image}" class="admin-input" placeholder="/images/post.jpg">
                        </div>
                        <div class="form-grid-full">
                            <label class="admin-label" for="tags">Tags (JSON array)</label>
                            <input type="text" id="tags" name="tags" value='{tags}' class="admin-input" placeholder='["rust", "web"]'>
                        </div>
                    </div>
                </div>

                <!-- Polish content -->
                <div class="section-card">
                    <h2 class="section-title">Polish Content (PL)</h2>
                    <div style="display: flex; flex-direction: column; gap: 1rem;">
                        <div>
                            <label class="admin-label" for="title_pl">Title</label>
                            <input type="text" id="title_pl" name="title_pl" value="{title_pl}" required class="admin-input">
                        </div>
                        <div>
                            <label class="admin-label" for="excerpt_pl">Excerpt</label>
                            <textarea id="excerpt_pl" name="excerpt_pl" rows="2" required class="admin-input" style="min-height: 4rem;">{excerpt_pl}</textarea>
                        </div>
                        <div>
                            <label class="admin-label" for="content_pl">Content (Markdown)</label>
                            <textarea id="content_pl" name="content_pl" rows="12" required class="admin-input">{content_pl}</textarea>
                        </div>
                    </div>
                </div>

                <!-- English content -->
                <div class="section-card">
                    <h2 class="section-title">English Content (EN)</h2>
                    <div style="display: flex; flex-direction: column; gap: 1rem;">
                        <div>
                            <label class="admin-label" for="title_en">Title</label>
                            <input type="text" id="title_en" name="title_en" value="{title_en}" required class="admin-input">
                        </div>
                        <div>
                            <label class="admin-label" for="excerpt_en">Excerpt</label>
                            <textarea id="excerpt_en" name="excerpt_en" rows="2" required class="admin-input" style="min-height: 4rem;">{excerpt_en}</textarea>
                        </div>
                        <div>
                            <label class="admin-label" for="content_en">Content (Markdown)</label>
                            <textarea id="content_en" name="content_en" rows="12" required class="admin-input">{content_en}</textarea>
                        </div>
                    </div>
                </div>

                <div style="display: flex; gap: 1rem; margin-top: 1.5rem;">
                    <button type="submit" class="admin-btn">{button_text}</button>
                    <a href="/admin/posts" class="admin-btn admin-btn-outline">Cancel</a>
                </div>
            </form>
        </div>"#,
        title = title,
        action = action,
        error_html = error_html,
        slug = encode_text(slug),
        date = encode_text(date),
        draft_selected = draft_selected,
        published_selected = published_selected,
        image = encode_text(image),
        tags = encode_text(tags),
        title_pl = encode_text(title_pl),
        excerpt_pl = encode_text(excerpt_pl),
        content_pl = encode_text(content_pl),
        title_en = encode_text(title_en),
        excerpt_en = encode_text(excerpt_en),
        content_en = encode_text(content_en),
        button_text = button_text
    )
}

pub async fn new_post_page_handler() -> Html<String> {
    Html(admin_layout("New Post", &post_form(None, None)))
}

pub async fn create_post_handler(
    State(pool): State<DbPool>,
    Form(form): Form<CreatePostForm>,
) -> Response {
    let image = if form.image.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        None
    } else {
        form.image.clone()
    };

    let result = sqlx::query(
        r#"INSERT INTO posts (slug, title_pl, excerpt_pl, content_pl, title_en, excerpt_en, content_en, date, status, tags, image)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&form.slug)
    .bind(&form.title_pl)
    .bind(&form.excerpt_pl)
    .bind(&form.content_pl)
    .bind(&form.title_en)
    .bind(&form.excerpt_en)
    .bind(&form.content_en)
    .bind(&form.date)
    .bind(&form.status)
    .bind(&form.tags)
    .bind(&image)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/admin/posts").into_response(),
        Err(e) => {
            let error_msg = format!("Failed to create post: {}", e);
            Html(admin_layout("New Post", &post_form(None, Some(&error_msg)))).into_response()
        }
    }
}

pub async fn edit_post_page_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Response {
    let post: Option<DbPost> = sqlx::query_as("SELECT * FROM posts WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten();

    match post {
        Some(post) => Html(admin_layout("Edit Post", &post_form(Some(&post), None))).into_response(),
        None => (StatusCode::NOT_FOUND, "Post not found").into_response(),
    }
}

pub async fn update_post_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    Form(form): Form<UpdatePostForm>,
) -> Response {
    let image = if form.image.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        None
    } else {
        form.image.clone()
    };

    let result = sqlx::query(
        r#"UPDATE posts SET
           slug = ?, title_pl = ?, excerpt_pl = ?, content_pl = ?,
           title_en = ?, excerpt_en = ?, content_en = ?,
           date = ?, status = ?, tags = ?, image = ?,
           updated_at = datetime('now')
           WHERE id = ?"#
    )
    .bind(&form.slug)
    .bind(&form.title_pl)
    .bind(&form.excerpt_pl)
    .bind(&form.content_pl)
    .bind(&form.title_en)
    .bind(&form.excerpt_en)
    .bind(&form.content_en)
    .bind(&form.date)
    .bind(&form.status)
    .bind(&form.tags)
    .bind(&image)
    .bind(id)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/admin/posts").into_response(),
        Err(e) => {
            let post: Option<DbPost> = sqlx::query_as("SELECT * FROM posts WHERE id = ?")
                .bind(id)
                .fetch_optional(&pool)
                .await
                .ok()
                .flatten();

            let error_msg = format!("Failed to update post: {}", e);
            match post {
                Some(post) => Html(admin_layout("Edit Post", &post_form(Some(&post), Some(&error_msg)))).into_response(),
                None => (StatusCode::NOT_FOUND, "Post not found").into_response(),
            }
        }
    }
}

pub async fn delete_post_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Response {
    let _ = sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await;

    Redirect::to("/admin/posts").into_response()
}
