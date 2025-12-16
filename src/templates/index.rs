use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // Don't worry, there are much better ways of styling in Perseus!
        div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh;") {
            h1 { "Welcome to my page!" }
            p {
                "This page is still under construction..."
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Personal blog" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
