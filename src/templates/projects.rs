use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::components::back_to_top::BackToTop;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::components::project_card::ProjectCard;
use crate::models::Project;

#[derive(Clone, Debug, Serialize, Deserialize, ReactiveState)]
#[rx(alias = "ProjectsStateRx")]
pub struct ProjectsState {
    pub projects: Option<Vec<Project>>,
}

#[auto_scope]
fn projects_page<G: Html>(cx: Scope, state: &'a ProjectsStateRx) -> View<G> {
    let projects = (*state.projects.get()).clone();
    let projects_signal = create_signal(cx, projects.clone().unwrap_or_default());
    let project_count = projects.as_ref().map(|p| p.len()).unwrap_or(0);

    view! { cx,
        Navbar {}

        main(class="max-w-5xl mx-auto px-6 pt-32 pb-20") {
            // Header
            header(class="mb-12") {
                h1(class="text-4xl md:text-5xl font-semibold text-cream mb-4 tracking-tight") {
                    "Projects"
                }
                p(class="text-gray-500 text-lg") {
                    (format!("{} project{}", project_count, if project_count != 1 { "s" } else { "" }))
                }
            }

            // Projects grid
            (if !projects_signal.get().is_empty() {
                view! { cx,
                    div(class="grid md:grid-cols-2 gap-6") {
                        Indexed(
                            iterable=projects_signal,
                            view=|cx, project| view! { cx,
                                ProjectCard(project=project)
                            }
                        )
                    }
                }
            } else {
                view! { cx,
                    div(class="text-gray-500 text-center py-20 border border-dark/50 rounded-xl bg-darker/30") {
                        p(class="text-lg") { "No projects yet." }
                        p(class="text-sm mt-2") { "Check back soon for new projects!" }
                    }
                }
            })
        }

        Footer {}
        BackToTop {}
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> ProjectsState {
    use crate::models::ProjectsConfig;

    let projects_config = ProjectsConfig::load().expect("Failed to load projects config");

    ProjectsState {
        projects: projects_config.projects,
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Projects | Kacper" }
        meta(name="description", content="Explore my portfolio of web development projects, built with modern technologies like Rust, TypeScript, and more.")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("projects")
        .build_state_fn(get_build_state)
        .head(head)
        .view_with_state(projects_page)
        .build()
}
