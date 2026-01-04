use crate::components::icons::*;
use crate::models::Project;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct ProjectCardProps {
    pub project: Project,
}

#[component]
pub fn ProjectCard<G: Html>(cx: Scope, props: ProjectCardProps) -> View<G> {
    let project = props.project.clone();
    let tags = create_signal(cx, project.tags.clone());
    let has_demo = !project.demo.is_empty();
    let github_url = project.github.clone();
    let demo_url = project.demo.clone();
    let project_title = project.title.clone();

    view! { cx,
        article(class="project-card group bg-darker/30 border border-dark rounded-xl overflow-hidden hover:border-accent/40 transition-all duration-300") {
            div(class="aspect-video overflow-hidden bg-dark") {
                img(
                    src=project.image,
                    alt=format!("{} screenshot", project_title),
                    class="project-image w-full h-full object-cover opacity-80 group-hover:opacity-100 transition-all duration-500",
                    loading="lazy"
                )
            }
            div(class="p-5") {
                div(class="flex items-center justify-between mb-2") {
                    h3(class="text-cream font-medium") { (project.title) }
                    div(class="flex gap-2") {
                        a(
                            href=github_url,
                            class="text-gray-500 hover:text-accent-light transition-colors",
                            target="_blank",
                            rel="noopener noreferrer"
                        ) {
                            IconGithubSmall {}
                        }
                        (if has_demo
                            {
                            let demo_url = demo_url.clone();
                            view! { cx,
                                a(
                                    href=demo_url.clone(),
                                    class="text-gray-500 hover:text-accent-light transition-colors",
                                    target="_blank",
                                    rel="noopener noreferrer"
                                ) {
                                    IconExternalLink {}
                                }
                            }
                        } else {
                            view! { cx, }
                        })
                    }
                }
                p(class="text-gray-500 text-sm mb-4 leading-relaxed") {
                    (project.description)
                }
                ul(class="flex flex-wrap gap-2") {
                    Indexed(
                        iterable=tags,
                        view=|cx, tag| view! { cx,
                            li(class="px-2 py-1 bg-dark/80 text-gray-400 text-xs font-mono rounded") {
                                (tag)
                            }
                        }
                    )
                }
            }
        }
    }
}

#[derive(Prop)]
pub struct ProjectsGridProps {
    pub projects: Option<Vec<Project>>,
}

#[component]
pub fn ProjectsGrid<G: Html>(cx: Scope, props: ProjectsGridProps) -> View<G> {
    view! { cx,
        section(id="projects", class="py-24") {
            h2(class="font-mono text-sm text-accent-light mb-8 flex items-center gap-3") {
                span(class="w-8 h-px bg-accent") {}
                "projects"
            }
            (match props.projects {
                Some(ref projects) if !projects.is_empty() => {
                    let projects_signal = create_signal(cx, projects.clone());
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
                }
                _ => {
                    view! { cx,
                        div(class="text-gray-500 text-center py-12 border border-dark/50 rounded-xl bg-darker/30") {
                            p { "No projects yet." }
                        }
                    }
                }
            })
        }
    }
}
