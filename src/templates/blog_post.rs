use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::back_to_top::BackToTop;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;

use crate::models::Post;

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> Post {
    use crate::models::post::loader;
    use std::env;

    let posts_dir = env::current_dir().unwrap().join("posts");

    match loader::load_post_by_slug_and_locale(&posts_dir, &info.path, &info.locale) {
        Ok(post) => post,
        Err(e) => {
            eprintln!("Warning: Could not load post '{}' for locale '{}': {}. This post will be skipped.", info.path, info.locale, e);
            // Return a placeholder post that indicates the content is not available
            Post {
                frontmatter: crate::models::post::PostFrontmatter {
                    title: format!("Content not available in {}", info.locale),
                    slug: info.path.clone(),
                    date: String::new(),
                    status: "draft".to_string(),
                    excerpt: format!("This post is not available in the {} locale.", info.locale),
                    tags: vec![],
                    image: None,
                    seo: None,
                },
                content: format!("This content is not available in the {} locale.", info.locale),
                html_content: format!("<p>This content is not available in the {} locale.</p>", info.locale),
            }
        }
    }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    use crate::models::post::loader;
    use std::env;

    let posts_dir = env::current_dir().unwrap().join("posts");
    let locales = vec!["pl", "en"];
    let mut locale_slugs: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for locale in &locales {
        let posts = loader::load_all_posts_for_locale(&posts_dir, Some(locale)).unwrap_or_default();

        eprintln!("Found {} posts for locale {}", posts.len(), locale);
        let slugs: Vec<String> = posts.iter().map(|p| {
            eprintln!("  - {} (slug: {})", p.frontmatter.title, p.frontmatter.slug);
            p.frontmatter.slug.clone()
        }).collect();

        locale_slugs.insert(locale.to_string(), slugs);
    }

    // Only build paths that exist for each locale
    let all_paths: Vec<String> = locale_slugs.values()
        .flatten()
        .map(|s| s.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    BuildPaths {
        paths: all_paths,
        extra: ().into(),
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Blog Post | Kacper" }
    }
}

#[component]
fn BlogPostPage<G: Html>(cx: Scope, post: Post) -> View<G> {
    let tags = create_signal(cx, post.frontmatter.tags.clone());
    let reading_time = post.reading_time();
    let html_content = create_ref(cx, post.html_content.clone());
    let date = create_ref(cx, post.frontmatter.date.clone());
    let title = create_ref(cx, post.frontmatter.title.clone());

    view! { cx,
        Navbar {}

        main(class="max-w-3xl mx-auto px-6 pt-32 pb-20") {
            // Back link
            a(
                href="/blog",
                class="inline-flex items-center gap-2 text-gray-500 hover:text-accent-light transition-colors text-sm font-mono mb-10"
            ) {
                "← Back to blog"
            }

            // Article header
            header(class="mb-12") {
                div(class="flex items-center gap-4 mb-4") {
                    time(
                        datetime=date,
                        class="font-mono text-xs text-gray-600"
                    ) {
                        (*date)
                    }
                    span(class="text-gray-700") { "·" }
                    span(class="font-mono text-xs text-gray-600") {
                        (format!("{} min read", reading_time))
                    }
                }

                h1(class="text-3xl md:text-4xl font-semibold text-cream mb-6 tracking-tight leading-tight") {
                    (*title)
                }

                div(class="flex flex-wrap gap-2") {
                    Indexed(
                        iterable=tags,
                        view=|cx, tag| view! { cx,
                            span(class="px-2 py-1 bg-darker border border-dark text-gray-400 text-xs font-mono rounded") {
                                (tag)
                            }
                        }
                    )
                }
            }

            // Article content
            article(
                class="prose-custom",
                dangerously_set_inner_html=html_content
            )

            // Author section
            div(class="mt-16 pt-8 border-t border-dark") {
                div(class="flex items-center gap-4") {
                    div(class="w-12 h-12 rounded-full bg-darker border border-dark flex items-center justify-center") {
                        span(class="text-accent-light font-mono text-lg") { "K" }
                    }
                    div {
                        p(class="text-cream font-medium") { "Kacper" }
                        p(class="text-gray-600 text-sm") { "Software Developer" }
                    }
                }
            }
        }

        Footer {}
        BackToTop {}
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("blog/posts")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .head(head)
        .view_with_unreactive_state(BlogPostPage)
        .build()
}
