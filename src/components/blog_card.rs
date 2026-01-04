use crate::components::icons::IconArrowRight;
use crate::models::Post;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct BlogCardProps {
    pub post: Post,
}

#[component]
pub fn BlogCard<G: Html>(cx: Scope, props: BlogCardProps) -> View<G> {
    let post = props.post;
    let href = format!("/blog/posts/{}", post.frontmatter.slug);
    let date = create_ref(cx, post.frontmatter.date.clone());
    let title = create_ref(cx, post.frontmatter.title.clone());
    let excerpt = create_ref(cx, post.frontmatter.excerpt.clone());

    view! { cx,
        article {
            a(
                href=href,
                class="blog-card block bg-darker/30 border border-dark rounded-xl p-5 hover:bg-darker/50 hover:border-accent/40 transition-all duration-300 group"
            ) {
                time(
                    datetime=date,
                    class="font-mono text-xs text-gray-600 mb-3 block"
                ) {
                    (*date)
                }
                h3(class="text-cream font-medium mb-2 group-hover:text-accent-light transition-colors") {
                    (*title)
                }
                p(class="text-gray-500 text-sm leading-relaxed mb-4") {
                    (*excerpt)
                }
                span(class="flex items-center text-accent-light text-sm font-mono") {
                    "Read more"
                    span(class="ml-1 group-hover:translate-x-1 transition-transform") {
                        IconArrowRight {}
                    }
                }
            }
        }
    }
}

#[derive(Prop)]
pub struct BlogSectionProps {
    pub posts: Option<Vec<Post>>,
}

#[component]
pub fn BlogSection<G: Html>(cx: Scope, props: BlogSectionProps) -> View<G> {
    view! { cx,
        section(id="blog", class="py-24") {
            (match props.posts {
                Some(ref posts) if !posts.is_empty() => {
                    let display_posts: Vec<Post> = posts.iter().take(2).cloned().collect();
                    let posts_signal = create_signal(cx, display_posts);

                    view! { cx,
                        div(class="bg-darker/30 border border-dark rounded-xl p-8") {
                            div(class="flex items-center justify-between mb-8") {
                                h2(class="text-2xl font-semibold text-cream") {
                                    "Recent posts"
                                }
                                a(
                                    href="/blog",
                                    class="inline-flex items-center gap-2 px-4 py-2 bg-accent text-darkest font-mono text-sm rounded-lg hover:bg-accent-light transition-all duration-300 group"
                                ) {
                                    "See all"
                                    span(class="group-hover:translate-x-1 transition-transform") {
                                        IconArrowRight {}
                                    }
                                }
                            }

                            div(class="grid md:grid-cols-2 gap-6") {
                                Indexed(
                                    iterable=posts_signal,
                                    view=|cx, post| view! { cx,
                                        BlogCard(post=post)
                                    }
                                )
                            }
                        }
                    }
                }
                _ => {
                    view! { cx,
                        div(class="bg-darker/30 border border-dark rounded-xl p-8") {
                            div(class="text-gray-500 text-center py-12") {
                                p(class="text-lg") { "No posts yet." }
                                p(class="text-sm mt-2") { "Check back soon for new content!" }
                            }
                        }
                    }
                }
            })
        }
    }
}
