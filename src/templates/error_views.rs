use perseus::errors::ClientError;
use perseus::prelude::*;
use sycamore::prelude::*;

pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    ErrorViews::new(|cx, err, _err_info, _err_pos| {
        match err {
            // Errors from the server, like 404s; these are best displayed over the whole page
            ClientError::ServerError {
                status,
                message: _
            } => match status {
                // 404 - Page not found
                404 => (
                    view! { cx,
                        title { "404 - Page Not Found" }
                    },
                    view! { cx,
                        div(class="min-h-screen bg-darkest flex items-center justify-center px-6") {
                            div(class="max-w-3xl w-full text-center animate-fade-in") {
                                // Animated 404 number
                                div(class="mb-12") {
                                    p(class="font-mono text-sm text-accent-light mb-6") {
                                        "error_404"
                                    }
                                    div(class="relative inline-block") {
                                        h1(class="font-sans text-9xl font-semibold text-cream tracking-tight") {
                                            "404"
                                        }
                                        div(class="absolute -left-8 top-2 w-2 h-2 rounded-full bg-accent glow-dot") {}
                                    }
                                }

                                // Error message
                                h2(class="text-2xl md:text-3xl text-gray-500 font-light mb-6") {
                                    "Page Not Found"
                                }
                                p(class="text-gray-400 mb-12 leading-relaxed") {
                                    "The page you're looking for doesn't exist or has been moved."
                                }

                                // Action buttons
                                div(class="flex flex-col md:flex-row gap-4 justify-center items-center") {
                                    a(
                                        href="/",
                                        class="px-8 py-4 bg-accent hover:bg-accent-light text-cream font-medium rounded-lg transition-all duration-300"
                                    ) {
                                        "← Back to Home"
                                    }
                                }
                            }
                        }
                    }
                ),

                // 4xx - Client errors
                _ if (400..500).contains(&status) => (
                    view! { cx,
                        title { (format!("{} - Client Error", status)) }
                    },
                    view! { cx,
                        div(class="min-h-screen bg-darkest flex items-center justify-center px-6") {
                            div(class="max-w-3xl w-full text-center animate-fade-in") {
                                div(class="mb-12") {
                                    p(class="font-mono text-sm text-accent-light mb-6") {
                                        (format!("error_{}", status))
                                    }
                                    h1(class="font-sans text-5xl md:text-6xl font-semibold text-cream tracking-tight mb-6") {
                                        "Something Went Wrong"
                                    }
                                }
                                p(class="text-gray-400 mb-12 leading-relaxed") {
                                    "There was an issue with your request. Please try again."
                                }
                                a(
                                    href="/",
                                    class="inline-block px-8 py-4 bg-accent hover:bg-accent-light text-cream font-medium rounded-lg transition-all duration-300"
                                ) {
                                    "← Back to Home"
                                }
                            }
                        }
                    }
                ),

                // 5xx - Server errors
                _ => (
                    view! { cx,
                        title { (format!("{} - Server Error", status)) }
                    },
                    view! { cx,
                        div(class="min-h-screen bg-darkest flex items-center justify-center px-6") {
                            div(class="max-w-3xl w-full text-center animate-fade-in") {
                                div(class="mb-12") {
                                    p(class="font-mono text-sm text-accent-light mb-6") {
                                        (format!("error_{}", status))
                                    }
                                    h1(class="font-sans text-5xl md:text-6xl font-semibold text-cream tracking-tight mb-6") {
                                        "Server Error"
                                    }
                                }
                                p(class="text-gray-400 mb-12 leading-relaxed") {
                                    "The server encountered an error. Please try again in a moment."
                                }
                                button(
                                    onclick="window.location.reload()",
                                    class="px-8 py-4 bg-accent hover:bg-accent-light text-cream font-medium rounded-lg transition-all duration-300"
                                ) {
                                    "↻ Reload Page"
                                }
                            }
                        }
                    }
                )
            },

            // Critical panic error
            ClientError::Panic(_) => (
                view! { cx,
                    title { "Critical Error" }
                },
                view! { cx,
                    div(class="min-h-screen bg-darkest flex items-center justify-center px-6") {
                        div(class="max-w-3xl w-full text-center animate-fade-in") {
                            div(class="mb-12") {
                                p(class="font-mono text-sm text-accent-light mb-6") {
                                    "critical_error"
                                }
                                h1(class="font-sans text-5xl md:text-6xl font-semibold text-cream tracking-tight mb-6") {
                                    "Critical Error"
                                }
                            }
                            p(class="text-gray-400 mb-12 leading-relaxed") {
                                "A critical error has occurred. Please try reloading the page."
                            }
                            button(
                                onclick="window.location.reload()",
                                class="px-8 py-4 bg-accent hover:bg-accent-light text-cream font-medium rounded-lg transition-all duration-300"
                            ) {
                                "↻ Reload Page"
                            }
                        }
                    }
                }
            ),

            // Network errors
            ClientError::FetchError(_) => (
                view! { cx,
                    title { "Network Error" }
                },
                view! { cx,
                    div(class="min-h-screen bg-darkest flex items-center justify-center px-6") {
                        div(class="max-w-3xl w-full text-center animate-fade-in") {
                            div(class="mb-12") {
                                p(class="font-mono text-sm text-accent-light mb-6") {
                                    "network_error"
                                }
                                h1(class="font-sans text-5xl md:text-6xl font-semibold text-cream tracking-tight mb-6") {
                                    "Connection Lost"
                                }
                            }
                            p(class="text-gray-400 mb-12 leading-relaxed") {
                                "Unable to connect to the server. Please check your internet connection."
                            }
                            button(
                                onclick="window.location.reload()",
                                class="px-8 py-4 bg-accent hover:bg-accent-light text-cream font-medium rounded-lg transition-all duration-300"
                            ) {
                                "↻ Retry"
                            }
                        }
                    }
                }
            ),

            // All other internal errors
            ClientError::InvariantError(_) |
            ClientError::PluginError(_) |
            ClientError::ThawError(_) |
            ClientError::PlatformError(_) |
            ClientError::PreloadError(_) => (
                view! { cx,
                    title { "Internal Error" }
                },
                view! { cx,
                    div(class="min-h-screen bg-darkest flex items-center justify-center px-6") {
                        div(class="max-w-3xl w-full text-center animate-fade-in") {
                            div(class="mb-12") {
                                p(class="font-mono text-sm text-accent-light mb-6") {
                                    "internal_error"
                                }
                                h1(class="font-sans text-5xl md:text-6xl font-semibold text-cream tracking-tight mb-6") {
                                    "Something Went Wrong"
                                }
                            }
                            p(class="text-gray-400 mb-8 leading-relaxed") {
                                "An unexpected error occurred. Please try reloading the page."
                            }

                            details(class="mb-12 text-left max-w-xl mx-auto border border-dark/50 rounded-lg bg-darker/30 overflow-hidden") {
                                summary(class="cursor-pointer font-mono text-sm text-gray-500 hover:text-accent-light p-4 transition-colors") {
                                    "Technical details"
                                }
                                div(class="p-4 bg-dark/50 text-xs text-gray-400 font-mono overflow-auto border-t border-dark/50") {
                                    (format!("{}", err))
                                }
                            }

                            div(class="flex flex-col md:flex-row gap-4 justify-center items-center") {
                                button(
                                    onclick="window.location.reload()",
                                    class="px-8 py-4 bg-accent hover:bg-accent-light text-cream font-medium rounded-lg transition-all duration-300"
                                ) {
                                    "↻ Reload Page"
                                }
                                a(
                                    href="/",
                                    class="px-8 py-4 border border-dark hover:border-accent/40 text-gray-400 hover:text-cream font-medium rounded-lg transition-all duration-300"
                                ) {
                                    "← Back to Home"
                                }
                            }
                        }
                    }
                }
            )
        }
    })
}
