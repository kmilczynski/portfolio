use sycamore::prelude::*;

#[component]
pub fn BackToTop<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        a(
            href="#",
            class="back-to-top",
            aria-label="Back to top"
        ) {
            svg(
                fill="none",
                stroke="currentColor",
                viewBox="0 0 24 24"
            ) {
                path(
                    stroke-linecap="round",
                    stroke-linejoin="round",
                    stroke-width="2",
                    d="M5 10l7-7m0 0l7 7m-7-7v18"
                )
            }
        }

        // Add inline script for scroll behavior
        script {
            r#"
            (function() {
                var backToTop = document.querySelector('.back-to-top');
                if (!backToTop) return;

                function toggleVisibility() {
                    if (window.scrollY > 300) {
                        backToTop.classList.add('visible');
                    } else {
                        backToTop.classList.remove('visible');
                    }
                }

                window.addEventListener('scroll', toggleVisibility);
                toggleVisibility();

                backToTop.addEventListener('click', function(e) {
                    e.preventDefault();
                    window.scrollTo({ top: 0, behavior: 'smooth' });
                });
            })();
            "#
        }
    }
}
