use crate::components::icons::*;
use crate::models::SocialLinks as SocialLinksData;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct SocialLinksProps {
    pub links: SocialLinksData,
}

#[component]
pub fn SocialLinks<G: Html>(cx: Scope, props: SocialLinksProps) -> View<G> {
    let github = props.links.github.clone();
    let linkedin = props.links.linkedin.clone();
    let email = format!("mailto:{}", props.links.email);

    view! { cx,
        nav(class="flex gap-5 mb-20") {
            a(
                href=github,
                class="text-gray-500 hover:text-accent-light transition-colors",
                target="_blank",
                rel="noopener noreferrer"
            ) {
                IconGithub {}
            }
            a(
                href=linkedin,
                class="text-gray-500 hover:text-accent-light transition-colors",
                target="_blank",
                rel="noopener noreferrer"
            ) {
                IconLinkedin {}
            }
            a(
                href=email,
                class="text-gray-500 hover:text-accent-light transition-colors"
            ) {
                IconEmail {}
            }
        }
    }
}
