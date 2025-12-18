use crate::models::Experience;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct TimelineProps {
    pub experience: Vec<Experience>,
}

#[component]
pub fn Timeline<G: Html>(cx: Scope, props: TimelineProps) -> View<G> {
    let experience = create_signal(cx, props.experience);

    view! { cx,
        div(class="mt-20 animate-fade-in animate-delay-2") {
            h2(class="font-mono text-sm text-accent-light mb-8 flex items-center gap-3") {
                span(class="w-8 h-px bg-accent") {}
                "experience"
            }
            div(class="relative pl-8") {
                div(class="absolute left-[3px] top-2 bottom-2 w-px timeline-line") {}
                div(class="space-y-10") {
                    Indexed(
                        iterable=experience,
                        view=|cx, exp| {
                            let dot_class = if exp.current {
                                "absolute -left-8 top-1.5 w-2 h-2 rounded-full bg-accent-light glow-dot"
                            } else {
                                "absolute -left-8 top-1.5 w-2 h-2 rounded-full bg-accent/60"
                            };

                            view! { cx,
                                article(class="relative") {
                                    div(class=dot_class) {}
                                    div(class="flex flex-col md:flex-row md:items-center gap-1 md:gap-4 mb-2") {
                                        h3(class="text-cream font-medium") { (exp.title) }
                                        span(class="text-gray-600 text-sm") { "·" }
                                        span(class="text-accent-light text-sm") { (exp.company) }
                                    }
                                    p(class="font-mono text-xs text-gray-600 mb-2") {
                                        (exp.period)
                                    }
                                    p(class="text-gray-400 text-sm leading-relaxed") {
                                        (exp.description)
                                    }
                                }
                            }
                        }
                    )
                }
            }
        }
    }
}
