use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;

use crate::admin::auth::validate_session;
use crate::admin::db::DbPool;

pub async fn auth_middleware<B>(
    State(pool): State<DbPool>,
    jar: CookieJar,
    request: Request<B>,
    next: Next<B>,
) -> Response
where
    B: Send,
{
    let path = request.uri().path();

    // Allow login page without auth (path is relative to /admin nest)
    if path == "/login" || path == "/admin/login" {
        return next.run(request).await;
    }

    // Check for session cookie
    let session_token = jar
        .get("admin_session")
        .map(|cookie| cookie.value().to_string());

    match session_token {
        Some(token) => {
            match validate_session(&pool, &token).await {
                Ok(true) => next.run(request).await,
                Ok(false) => Redirect::to("/admin/login").into_response(),
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
                }
            }
        }
        None => Redirect::to("/admin/login").into_response(),
    }
}
