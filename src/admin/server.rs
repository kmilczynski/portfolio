use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

use crate::admin::api::{get_post_api, list_posts_api, rss_feed, sitemap_xml, robots_txt};
use crate::admin::db::{create_sqlite_pool, init_global_pool, DbPool};
use crate::admin::handlers::{
    create_post_handler, dashboard_handler, delete_post_handler, edit_post_page_handler,
    login_handler, login_page_handler, logout_handler, new_post_page_handler, posts_list_handler,
    update_post_handler, upload_image_handler,
};
use crate::admin::middleware::auth_middleware;

fn admin_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(dashboard_handler))
        .route("/login", get(login_page_handler).post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/posts", get(posts_list_handler))
        .route("/posts/new", get(new_post_page_handler).post(create_post_handler))
        .route("/posts/:id/edit", get(edit_post_page_handler).post(update_post_handler))
        .route("/posts/:id/delete", post(delete_post_handler))
        .route("/upload", post(upload_image_handler))
        .layer(middleware::from_fn_with_state(pool.clone(), auth_middleware))
        .with_state(pool)
}

fn api_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/posts", get(list_posts_api))
        .route("/posts/:slug", get(get_post_api))
        .with_state(pool)
}

pub async fn get_server<M: perseus::stores::MutableStore + 'static, T: perseus::i18n::TranslationsManager + 'static>(
    turbine: &'static perseus::turbine::Turbine<M, T>,
    opts: perseus::server::ServerOptions,
    (host, port): (String, u16),
) {
    // Load .env file
    dotenv::dotenv().ok();

    let addr: SocketAddr = if host.contains(':') {
        // IPv6 address needs brackets
        format!("[{}]:{}", host, port)
    } else {
        format!("{}:{}", host, port)
    }
    .parse()
    .expect("Invalid address provided");

    // Initialize database pool
    let pool = create_sqlite_pool()
        .await
        .expect("Failed to create database pool");

    // Store pool globally for template access
    init_global_pool(pool.clone());

    // Create the Perseus router
    let perseus_router = perseus_axum::get_router(turbine, opts).await;

    // Build the complete app
    let seo_routes = Router::new()
        .route("/feed.xml", get(rss_feed))
        .route("/sitemap.xml", get(sitemap_xml))
        .with_state(pool.clone());

    let app = Router::new()
        .nest("/admin", admin_routes(pool.clone()))
        .nest("/api", api_routes(pool.clone()))
        .route("/robots.txt", get(robots_txt))
        .merge(seo_routes)
        .merge(perseus_router);

    tracing::info!("Server running on http://{}", addr);
    tracing::info!("Admin panel: http://{}/admin", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
