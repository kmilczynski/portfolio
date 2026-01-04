mod components;
mod models;
mod templates;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::blog_index::get_template())
        .template(crate::templates::blog_post::get_template())
        .error_views(crate::templates::error_views::get_error_views())
        .index_view(|cx| {
            sycamore::view! {cx,
                html(class="dark", lang="en") {
                    head() {
                        meta(name="viewport", content="width=device-width, initial-scale=1, minimum-scale=1")
                        meta(charset="UTF-8")
                        meta(name="author", content="Kacper")
                        link(href="/.perseus/static/css/styles.css", rel="stylesheet", defer=true)
                        link(rel="icon", type="image/x-icon", sizes="16x16", href="/.perseus/static/favicon.ico")
                        script(src="https://cdn.tailwindcss.com")
                    }
                    body() {
                        PerseusRoot()
                    }
                }
            }
        })
}
