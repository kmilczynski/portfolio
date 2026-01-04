use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::components::blog_card::BlogCard;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::models::Post;

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> BlogIndexState {
    use crate::models::post::loader;
    use std::env;

    let posts_dir = env::current_dir().unwrap().join("posts");
    let posts = loader::load_all_posts(&posts_dir).unwrap_or_default();

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
        .build_state_fn(get_build_state)
        .head(head)
        .view_with_state(blog_index_page)
        .build()
}
