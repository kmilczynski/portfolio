use crate::components::back_to_top::BackToTop;
use crate::components::blog_card::BlogSection;
use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::navbar::Navbar;
use crate::components::skills::Skills;
use crate::components::social_links::SocialLinks;
use crate::components::timeline::Timeline;
use crate::models::{
    AboutSection, Experience, Post, SiteConfig, Skill,
    SocialLinks as SocialLinksData,
};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, ReactiveState)]
#[rx(alias = "IndexStateRx")]
pub struct IndexState {
    pub about: AboutSection,
    pub social: SocialLinksData,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub posts: Option<Vec<Post>>,
    pub site_title: String,
    pub site_description: String,
}
#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &'a IndexStateRx) -> View<G> {
    let about = (*state.about.get()).clone();
    let social = (*state.social.get()).clone();
    let skills = (*state.skills.get()).clone();
    let experience = (*state.experience.get()).clone();
    let posts = (*state.posts.get()).clone();

    view! { cx,
        Navbar {}

        main(id="main-content", class="max-w-5xl mx-auto px-6") {
            section(id="about", class="min-h-80vh pt-32 pb-20") {
                Hero(about=about)
                SocialLinks(links=social)
                Skills(skills=skills)
                Timeline(experience=experience)
            }

            BlogSection(posts=posts)
        }

        Footer {}
        BackToTop {}
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Kacper Milczyński | Software Developer" }
        meta(name="description", content="Software developer specializing in backend systems, real-time applications, and exploring the Rust ecosystem.")
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
    use crate::models::post::loader;
    use std::env;

    let site_config = SiteConfig::load().expect("Failed to load site config");

    let posts_dir = env::current_dir().unwrap().join("posts");
    let posts = loader::load_all_posts(&posts_dir).ok();

    IndexState {
        about: site_config.about,
        social: site_config.social,
        skills: site_config.skills,
        experience: site_config.experience,
        posts,
        site_title: site_config.site.title,
        site_description: site_config.site.description,
    }
}
