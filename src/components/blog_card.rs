use crate::components::icons::IconArrowRight;
use crate::models::Post;
use perseus::prelude::*;
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
                href=link!(cx, &href),
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
