use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::navbar::Navbar;
use crate::components::project_card::ProjectsGrid;
use crate::components::skills::Skills;
use crate::components::social_links::SocialLinks;
use crate::components::timeline::Timeline;
use crate::models::{
    AboutSection, Experience, Project, ProjectsConfig, SiteConfig, Skill,
    SocialLinks as SocialLinksData,
};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// Combined state for the index page
#[derive(Debug, Clone, Serialize, Deserialize, ReactiveState)]
#[rx(alias = "IndexStateRx")]
pub struct IndexState {
    pub about: AboutSection,
    pub social: SocialLinksData,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub projects: Option<Vec<Project>>,
    pub site_title: String,
    pub site_description: String,
}
#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &'a IndexStateRx) -> View<G> {
    let about = (*state.about.get()).clone();
    let social = (*state.social.get()).clone();
    let skills = (*state.skills.get()).clone();
    let experience = (*state.experience.get()).clone();
    let projects = (*state.projects.get()).clone();

    view! { cx,
        Navbar {}

        main(id="main-content", class="max-w-5xl mx-auto px-6") {
            section(id="about", class="min-h-screen pt-32 pb-20") {
                Hero(about=about)
                SocialLinks(links=social)
                Skills(skills=skills)
                Timeline(experience=experience)
            }

            ProjectsGrid(projects=projects)

            section(id="blog", class="py-20") {
                h2(class="font-mono text-sm text-accent-light mb-8 flex items-center gap-3") {
                    span(class="w-8 h-px bg-accent") {}
                    "blog"
                }
                div(class="text-gray-500 text-center py-12") {
                    p { "Blog coming soon..." }
                }
            }
        }

        Footer {}
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Personal blog" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .head(head)
        .view_with_state(index_page)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexState {
    let site_config = SiteConfig::load().expect("Failed to load site config");
    let projects_config = ProjectsConfig::load().expect("Failed to load projects config");

    IndexState {
        about: site_config.about,
        social: site_config.social,
        skills: site_config.skills,
        experience: site_config.experience,
        projects: projects_config.projects,
        site_title: site_config.site.title,
        site_description: site_config.site.description,
    }
}
