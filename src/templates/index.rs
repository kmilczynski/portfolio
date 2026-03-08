use crate::components::back_to_top::BackToTop;
use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::navbar::Navbar;
use crate::components::skills::Skills;
use crate::components::social_links::SocialLinks;
use crate::components::timeline::Timeline;
use crate::models::{AboutSection, Experience, SiteConfig, Skill, SocialLinks as SocialLinksData};
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
    pub site_title: String,
    pub site_description: String,
    pub locale: String,
}
#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &'a IndexStateRx) -> View<G> {
    let about = (*state.about.get()).clone();
    let social = (*state.social.get()).clone();
    let skills = (*state.skills.get()).clone();
    let experience = (*state.experience.get()).clone();

    view! { cx,
        Navbar {}

        main(id="main-content", class="max-w-5xl mx-auto px-6") {
            section(id="about", class="min-h-80vh pt-32 pb-20") {
                Hero(about=about)
                SocialLinks(links=social)
                Skills(skills=skills)
                Timeline(experience=experience)
            }
        }

        Footer {}
        BackToTop {}
    }
}

#[engine_only_fn]
fn head(cx: Scope, state: IndexState) -> View<SsrNode> {
    let base_url = "https://kmilczynski.byst.re";
    let title = create_ref(cx, state.site_title.clone());
    let description = create_ref(cx, state.site_description.clone());
    let canonical = create_ref(cx, if state.locale == "pl" {
        base_url.to_string()
    } else {
        format!("{}/{}/", base_url, state.locale)
    });
    let og_image = create_ref(cx, format!("{}/og-default.png", base_url));
    let feed_url = create_ref(cx, format!("{}/feed.xml", base_url));

    // JSON-LD for Person/Organization
    let json_ld = create_ref(cx, format!(
        r#"{{
            "@context": "https://schema.org",
            "@type": "Person",
            "name": "Kacper Milczyński",
            "jobTitle": "Software Developer",
            "url": "{}",
            "sameAs": [
                "{}",
                "{}"
            ],
            "email": "{}",
            "description": "{}",
            "knowsAbout": ["PHP", "TypeScript", "JavaScript", "Rust", "Symfony", "Nest.js", "PostgreSQL", "MySQL", "MongoDB", "Redis", "Docker", "AWS", "GCP"],
            "alumniOf": {{
                "@type": "Organization",
                "name": "Software Development"
            }}
        }}"#,
        base_url,
        state.social.github,
        state.social.linkedin,
        state.social.email,
        description.replace('"', "\\\"")
    ));

    view! { cx,
        title { (*title) }
        meta(name="description", content=description)

        // Open Graph
        meta(property="og:type", content="website")
        meta(property="og:title", content=title)
        meta(property="og:description", content=description)
        meta(property="og:url", content=base_url)
        meta(property="og:image", content=og_image)

        // Twitter Card
        meta(name="twitter:card", content="summary_large_image")
        meta(name="twitter:title", content=title)
        meta(name="twitter:description", content=description)
        meta(name="twitter:image", content=og_image)

        // Canonical
        link(rel="canonical", href=canonical)

        // RSS Feed
        link(rel="alternate", type="application/rss+xml", title="Kacper's Blog", href=feed_url)

        // JSON-LD
        script(type="application/ld+json", dangerously_set_inner_html=json_ld)
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .head_with_state(head)
        .view_with_state(index_page)
        .build()
}

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> IndexState {
    let site_config =
        SiteConfig::load_for_locale(&info.locale).expect("Failed to load site config");

    IndexState {
        about: site_config.about,
        social: site_config.social,
        skills: site_config.skills,
        experience: site_config.experience,
        site_title: site_config.site.title,
        site_description: site_config.site.description,
        locale: info.locale.to_string(),
    }
}
