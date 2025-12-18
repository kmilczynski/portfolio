use crate::models::AboutSection;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct HeroProps {
    pub about: AboutSection,
}

#[component]
pub fn Hero<G: Html>(cx: Scope, props: HeroProps) -> View<G> {
    view! { cx,
        div(class="animate-fade-in") {
            p(class="font-mono text-accent-light text-sm mb-4") {
                (props.about.greeting)
            }
            h1(class="font-sans text-5xl md:text-6xl font-semibold text-cream mb-4 tracking-tight") {
                (props.about.name)
            }
            p(class="text-2xl md:text-3xl text-gray-500 font-light mb-8") {
                (props.about.tagline)
            }
            p(class="text-gray-400 max-w-xl leading-relaxed mb-12") {
                (props.about.bio)
            }
        }
    }
}
