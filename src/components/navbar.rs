use sycamore::prelude::*;

#[component]
pub fn Navbar<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        header {
            nav(class="fixed top-0 left-0 right-0 z-50 bg-darkest/80 backdrop-blur-md border-b border-dark/50") {
                div(class="max-w-5xl mx-auto px-6 py-4 flex justify-between items-center") {
                    a(href="/", class="font-mono text-cream font-medium tracking-tight text-lg") {
                        "Milczyński"
                        span(class="text-accent-light") { "." }
                    }
                    div(class="flex gap-8") {
                        a(href="/", class="nav-link relative text-sm text-gray-400 hover:text-cream transition-colors") { "home" }
                        a(href="/projects", class="nav-link relative text-sm text-gray-400 hover:text-cream transition-colors") { "projects" }
                        a(href="/blog", class="nav-link relative text-sm text-gray-400 hover:text-cream transition-colors") { "blog" }
                    }
                }
            }
        }
    }
}
