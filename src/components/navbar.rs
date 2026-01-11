use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Navbar<G: Html>(cx: Scope) -> View<G> {
    let current_locale = Reactor::<G>::from_cx(cx).get_translator().get_locale();
    #[allow(unused_variables)] // its used but only in client so it gives error
    let (locale_display, target_locale) = if current_locale == "pl" {
        ("🇬🇧", "en")
    } else {
        ("🇵🇱", "pl")
    };

    view! { cx,
        header {
            nav(class="fixed top-0 left-0 right-0 z-50 bg-darkest/80 backdrop-blur-md border-b border-dark/50") {
                div(class="max-w-5xl mx-auto px-6 py-4 flex justify-between items-center") {
                    a(href= link!(cx, "/"), class="font-mono text-cream font-medium tracking-tight text-lg") {
                        "Milczyński"
                        span(class="text-accent-light") { "." }
                    }
                    div(class="flex gap-8 items-center") {
                        a(href=link!(cx, "/"), class="nav-link relative text-sm text-gray-400 hover:text-cream transition-colors") { "home" }
                        a(href=link!(cx, "/projects"), class="nav-link relative text-sm text-gray-400 hover:text-cream transition-colors") { "projects" }
                        a(href=link!(cx, "/blog"), class="nav-link relative text-sm text-gray-400 hover:text-cream transition-colors") { "blog" }
                        span(
                            class="text-2xl cursor-pointer hover:scale-110 transition-transform duration-200 select-none",
                            style="cursor: pointer;",
                            on:click = move |_| {
                                #[cfg(client)]
                                Reactor::<G>::from_cx(cx).switch_locale(target_locale);
                            }
                        ) {
                            (locale_display)
                        }
                    }
                }
            }
        }
    }
}
