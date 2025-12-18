use crate::models::Skill;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct SkillsProps {
    pub skills: Vec<Skill>,
}

#[component]
pub fn Skills<G: Html>(cx: Scope, props: SkillsProps) -> View<G> {
    let skills = create_signal(cx, props.skills);

    view! { cx,
        div(class="animate-fade-in animate-delay-1") {
            h2(class="font-mono text-sm text-accent-light mb-6 flex items-center gap-3") {
                span(class="w-8 h-px bg-accent") {}
                "skills"
            }
            div(class="grid grid-cols-2 md:grid-cols-4 gap-4") {
                Indexed(
                    iterable=skills,
                    view=|cx, skill| view! { cx,
                        div(class="bg-darker/50 border border-dark rounded-lg p-4 hover:border-accent/30 transition-colors") {
                            p(class="text-cream text-sm font-medium mb-1") { (skill.name) }
                            p(class="text-gray-500 text-xs font-mono") { (skill.technologies) }
                        }
                    }
                )
            }
        }
    }
}
