#[cfg(client)]
use js_sys::Date;
use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    #[cfg(client)]
    let current_year = Date::new_0().get_full_year();
    #[cfg(not(client))]
    let current_year = 2026;

    view! { cx,
        footer(class="border-t border-dark/50 mt-10") {
            div(class="max-w-5xl mx-auto px-6 py-8") {
                div(class="flex flex-col md:flex-row justify-between items-center gap-4") {
                    p(class="text-gray-600 text-sm") {
                        "Built with "
                        span(class="text-accent-light") { "Rust" }
                        ", "
                        span(class="text-accent-light") { "Perseus" }
                        " & "
                        span(class="text-accent-light") { "Sycamore" }
                    }
                    p(class="font-mono text-gray-600 text-xs") {
                        "© " (current_year) " KM"
                    }
                }
            }
        }
    }
}
