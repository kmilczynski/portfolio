use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::back_to_top::BackToTop;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;

use crate::models::Post;

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec![],
        extra: ().into(),
    }
}

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> Post {
    use crate::admin::db::get_pool;
    use crate::admin::models::DbPost;
    use crate::models::post::loader::render_markdown;
    use crate::models::PostFrontmatter;

    let slug = info.path.clone();
    let locale = info.locale.to_string();

    let pool = match get_pool() {
        Some(pool) => pool.clone(),
        None => {
            return Post {
                frontmatter: PostFrontmatter {
                    title: "Database Error".to_string(),
                    slug: slug.clone(),
                    date: String::new(),
                    status: "draft".to_string(),
                    excerpt: "Could not connect to database.".to_string(),
                    tags: vec![],
                    image: None,
                    seo: None,
                },
                content: "Database error.".to_string(),
                html_content: "<p>Could not connect to database.</p>".to_string(),
            };
        }
    };

    let slug_clone = slug.clone();
    let db_post: Option<DbPost> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            sqlx::query_as::<_, DbPost>(
                "SELECT * FROM posts WHERE slug = ?"
            )
            .bind(&slug_clone)
            .fetch_optional(&pool)
            .await
            .ok()
            .flatten()
        })
    });

    match db_post {
        Some(db_post) => {
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
        }
        None => {
            Post {
                frontmatter: PostFrontmatter {
                    title: "Post not found".to_string(),
                    slug: slug.clone(),
                    date: String::new(),
                    status: "draft".to_string(),
                    excerpt: "The requested post could not be found.".to_string(),
                    tags: vec![],
                    image: None,
                    seo: None,
                },
                content: "Post not found.".to_string(),
                html_content: "<p>The requested post could not be found.</p>".to_string(),
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, state: Post) -> View<SsrNode> {
    let base_url = "https://kmilczynski.byst.re";
    let post_url = create_ref(cx, format!("{}/blog/{}", base_url, state.frontmatter.slug));
    let title = create_ref(cx, format!("{} | Kacper", state.frontmatter.title));
    let description = create_ref(cx, state.frontmatter.excerpt.clone());
    let default_og_image = format!("{}/og-default.png", base_url);
    let og_image = create_ref(cx, state.frontmatter.image
        .as_ref()
        .map(|img| format!("{}/{}", base_url, img))
        .unwrap_or_else(|| default_og_image.clone()));
    let pub_date = create_ref(cx, state.frontmatter.date.clone());

    // JSON-LD structured data
    let author_json = r#"{
        "@type": "Person",
        "name": "Kacper Milczyński",
        "url": "https://kmilczynski.byst.re",
        "sameAs": [
            "https://github.com/kmilczynski",
            "https://linkedin.com/in/kacpermilczynski"
        ]
    }"#;

    let image_json = state.frontmatter.image
        .as_ref()
        .map(|img| format!(r#","image": {{"@type": "ImageObject", "url": "{}/{}"}}"#, base_url, img))
        .unwrap_or_default();

    let json_ld = create_ref(cx, format!(
        r#"{{
            "@context": "https://schema.org",
            "@type": "BlogPosting",
            "headline": "{}",
            "description": "{}",
            "datePublished": "{}",
            "dateModified": "{}",
            "author": {},
            "publisher": {{
                "@type": "Person",
                "name": "Kacper Milczyński",
                "url": "{}"
            }},
            "url": "{}",
            "mainEntityOfPage": {{
                "@type": "WebPage",
                "@id": "{}"
            }}{}
        }}"#,
        state.frontmatter.title.replace('"', "\\\""),
        description.replace('"', "\\\""),
        state.frontmatter.date,
        state.frontmatter.date,
        author_json,
        base_url,
        post_url,
        post_url,
        image_json
    ));

    view! { cx,
        title { (*title) }
        meta(name="description", content=description)

        // Open Graph
        meta(property="og:type", content="article")
        meta(property="og:title", content=title)
        meta(property="og:description", content=description)
        meta(property="og:url", content=post_url)
        meta(property="og:image", content=og_image)
        meta(property="article:published_time", content=pub_date)
        meta(property="article:author", content="Kacper Milczyński")

        // Twitter Card
        meta(name="twitter:card", content="summary_large_image")
        meta(name="twitter:title", content=title)
        meta(name="twitter:description", content=description)
        meta(name="twitter:image", content=og_image)

        // Canonical
        link(rel="canonical", href=post_url)

        // JSON-LD
        script(type="application/ld+json", dangerously_set_inner_html=json_ld)
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
                href=link!(cx, "/blog"),
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
        .incremental_generation()
        .revalidate_after("5s")
        .head_with_state(head)
        .view_with_unreactive_state(BlogPostPage)
        .build()
}
