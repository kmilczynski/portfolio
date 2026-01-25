use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::components::back_to_top::BackToTop;
use crate::components::blog_card::BlogCard;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::models::Post;

#[engine_only_fn]
async fn get_request_state(
    info: StateGeneratorInfo<()>,
    _req: Request,
) -> BlogIndexState {
    use crate::admin::db::get_pool;
    use crate::admin::models::DbPost;
    use crate::models::post::loader::render_markdown;
    use crate::models::PostFrontmatter;

    let locale = info.locale.to_string();

    let pool = match get_pool() {
        Some(pool) => pool.clone(),
        None => {
            eprintln!("Database pool not initialized");
            return BlogIndexState { posts: vec![] };
        }
    };

    // Use block_in_place to run async code in sync context
    let db_posts: Vec<DbPost> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            sqlx::query_as::<_, DbPost>(
                "SELECT * FROM posts WHERE status = 'published' ORDER BY date DESC"
            )
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        })
    });

    let posts: Vec<Post> = db_posts
        .into_iter()
        .map(|db_post| {
            let content = db_post.content(&locale).to_string();
            let html_content = render_markdown(&content);

            Post {
                frontmatter: PostFrontmatter {
                    title: db_post.title(&locale).to_string(),
                    slug: db_post.slug.clone(),
                    date: db_post.date.clone(),
                    status: db_post.status.clone(),
                    excerpt: db_post.excerpt(&locale).to_string(),
                    tags: db_post.tags_vec(),
                    image: db_post.image.clone(),
                    seo: None,
                },
                content,
                html_content,
            }
        })
        .collect();

    BlogIndexState { posts }
}

#[derive(Clone, Debug, Serialize, Deserialize, ReactiveState)]
#[rx(alias = "BlogIndexStateRx")]
pub struct BlogIndexState {
    pub posts: Vec<Post>,
}

#[auto_scope]
fn blog_index_page<G: Html>(cx: Scope, state: &'a BlogIndexStateRx) -> View<G> {
    let posts = (*state.posts.get()).clone();
    let posts_signal = create_signal(cx, posts.clone());
    let post_count = posts.len();

    view! { cx,
        Navbar {}

        main(class="max-w-5xl mx-auto px-6 pt-32 pb-20") {
            // Header
            header(class="mb-12") {
                h1(class="text-4xl md:text-5xl font-semibold text-cream mb-4 tracking-tight") {
                    "Blog"
                }
                p(class="text-gray-500 text-lg") {
                    (format!("{} post{}", post_count, if post_count != 1 { "s" } else { "" }))
                }
            }

            // Posts grid
            (if !posts.is_empty() {
                view! { cx,
                    div(class="grid md:grid-cols-2 gap-6") {
                        Indexed(
                            iterable=posts_signal,
                            view=|cx, post| view! { cx,
                                BlogCard(post=post)
                            }
                        )
                    }
                }
            } else {
                view! { cx,
                    div(class="text-gray-500 text-center py-20 border border-dark/50 rounded-xl bg-darker/30") {
                        p(class="text-lg") { "No posts yet." }
                        p(class="text-sm mt-2") { "Check back soon for new content!" }
                    }
                }
            })
        }

        Footer {}
        BackToTop {}
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Blog | Kacper" }
        meta(name="description", content="Read my thoughts on software development, Rust, web technologies, and more.")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("blog")
        .request_state_fn(get_request_state)
        .head(head)
        .view_with_state(blog_index_page)
        .build()
}
