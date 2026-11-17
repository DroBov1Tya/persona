mod content_schema;

use content_schema::{Locale, Project, SiteContent};
use std::sync::OnceLock;
use yew::prelude::*;

const ALL_PROJECTS: &str = "All";

#[derive(Clone, Copy, PartialEq)]
enum Language {
    En,
    Ru,
}

impl Language {
    fn other(self) -> Self {
        match self {
            Language::En => Language::Ru,
            Language::Ru => Language::En,
        }
    }

    fn code(self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Ru => "ru",
        }
    }
}

static CONTENT: OnceLock<SiteContent> = OnceLock::new();

fn content() -> &'static SiteContent {
    CONTENT.get_or_init(|| {
        serde_json::from_str(include_str!("../content.json"))
            .expect("content.json must match the site content structure")
    })
}

fn locale(language: Language) -> &'static Locale {
    let content = content();
    match language {
        Language::En => &content.en,
        Language::Ru => &content.ru,
    }
}

/// Keeps `<html lang>`, the title and the meta description in step with the chosen language.
fn sync_document(language: Language) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let seo = &locale(language).seo;
    document.set_title(&seo.title);
    // A missing element or a refused attribute only leaves the static English metadata in place.
    if let Some(root) = document.document_element() {
        let _ = root.set_attribute("lang", language.code());
    }
    if let Ok(Some(description)) = document.query_selector("meta[name='description']") {
        let _ = description.set_attribute("content", &seo.description);
    }
}

fn external_link(class: &'static str, url: &str, label: &str) -> Html {
    html! {
        <a {class} href={url.to_owned()} target="_blank" rel="noopener noreferrer">{label}</a>
    }
}

fn project_card(project: &Project, details_toggle: &str) -> Html {
    html! {
        <article class="project-card">
            <p class="project-area">{&project.area_label}</p>
            <h3>{external_link("project-name", &project.url, &project.name)}</h3>
            <p class="project-description">{&project.description}</p>
            <details class="project-details">
                <summary>{details_toggle}</summary>
                <p>{&project.details}</p>
            </details>
            <p class="project-stack">{&project.stack}</p>
        </article>
    }
}

#[function_component(App)]
fn app() -> Html {
    let lang = use_state(|| Language::En);
    let filter = use_state(|| ALL_PROJECTS.to_owned());
    let data = locale(*lang);
    let visible: Vec<&Project> = data
        .work
        .projects
        .iter()
        .filter(|project| *filter == ALL_PROJECTS || project.area == *filter)
        .collect();

    use_effect_with(*lang, |language| sync_document(*language));

    let toggle_language = {
        let lang = lang.clone();
        Callback::from(move |_| lang.set(lang.other()))
    };
    let featured = &data.work.featured;

    // The header below is mirrored by the static fallback in index.html; keep the markup the same.
    html! {
        <>
            <a class="skip-link" href="#main">{&data.common.skip_to_content}</a>
            <header class="sky" id="top">
                <div class="wrap">
                    <nav class="topbar" aria-label={data.common.main_navigation.clone()}>
                        <a class="brand" href="#top" aria-label={data.common.home.clone()}>
                            <img src="./favicon.svg" alt="" width="30" height="30" />
                            <span>{&data.header.brand}</span>
                        </a>
                        {for data.header.navigation.iter().map(|item| html! {
                            <a class="nav-link" href={format!("#{}", item.id)}>{&item.label}</a>
                        })}
                        <button class="lang-toggle" type="button" onclick={toggle_language} aria-label={data.common.switch_language.clone()}>
                            {&data.common.language_button}
                        </button>
                    </nav>
                    <div class="hero">
                        <h1>{&data.hero.name}</h1>
                        <p class="role">{&data.hero.role}</p>
                        <p class="lead">{&data.hero.description}</p>
                        <div class="actions">
                            <a class="button" href="#work">{&data.hero.primary_action}</a>
                            {external_link("chip-link", &data.links.github, &data.hero.github_action)}
                            {external_link("chip-link", &data.links.linkedin, &data.hero.linkedin_action)}
                        </div>
                    </div>
                </div>
            </header>

            <main id="main">
                <section id="work" aria-labelledby="work-title">
                    <div class="wrap">
                        <h2 id="work-title">{&data.work.title}</h2>
                        <p class="sub">{&data.work.description}</p>

                        <article class="featured">
                            <div class="featured-body">
                                <p class="featured-label">{&featured.label}</p>
                                <h3>
                                    <img src="./assets/dusk-mark.webp" alt="" width="40" height="40" />
                                    {&featured.name}
                                </h3>
                                <p class="featured-description">{&featured.description}</p>
                                <p class="project-stack">{&featured.stack}</p>
                                <div class="actions">
                                    {for featured.links.iter().map(|link| external_link("chip-link", &link.url, &link.label))}
                                </div>
                            </div>
                            <img class="featured-shot" src={featured.image.clone()} alt={featured.image_alt.clone()} width="1600" height="1000" decoding="async" />
                        </article>

                        <div class="filter-row" role="group" aria-label={data.work.filter_label.clone()}>
                            {for data.work.filters.iter().map(|item| {
                                let id = item.id.clone();
                                let active = *filter == item.id;
                                let filter = filter.clone();
                                html! {
                                    <button class="filter-chip" type="button" aria-pressed={if active { "true" } else { "false" }} onclick={Callback::from(move |_| filter.set(id.clone()))}>{&item.label}</button>
                                }
                            })}
                        </div>
                        <p class="sr-only" aria-live="polite">{data.common.showing_projects.replace("{count}", &visible.len().to_string())}</p>
                        <div class="project-grid">
                            {for visible.iter().map(|project| project_card(project, &data.work.details_toggle))}
                            <div class="project-card project-more">
                                <p>{&data.work.more_work}</p>
                                {external_link("chip-link", &data.links.github, &data.work.all_projects_action)}
                            </div>
                        </div>
                    </div>
                </section>

                <section id="about" aria-labelledby="about-title">
                    <div class="wrap">
                        <h2 id="about-title">{&data.about.title}</h2>
                        <p class="sub">{&data.about.description}</p>
                        <div class="points">
                            {for data.about.principles.iter().map(|item| html! {
                                <div><h3>{&item.title}</h3><p>{&item.description}</p></div>
                            })}
                        </div>
                    </div>
                </section>

                <section id="contact" aria-labelledby="contact-title">
                    <div class="wrap">
                        <h2 id="contact-title">{&data.contact.title}</h2>
                        <p class="sub">{&data.contact.description}</p>
                        <nav class="actions" aria-label={data.contact.social_links_label.clone()}>
                            {external_link("button", &data.links.linkedin, &data.contact.action)}
                            {external_link("chip-link", &data.links.github, &data.contact.github_label)}
                            {external_link("chip-link", &data.links.telegram, &data.contact.telegram_label)}
                            {external_link("chip-link", &data.links.telegram_channel, &data.contact.telegram_channel_label)}
                        </nav>
                    </div>
                </section>
            </main>

            <div class="wrap">
                <footer>
                    <span>{&data.footer.tagline}</span>
                    <a href="#top">{&data.footer.back_to_top}</a>
                </footer>
            </div>
        </>
    }
}

fn main() {
    // Yew clears the root before mounting, so the static fallback in index.html is replaced.
    yew::Renderer::<App>::with_root(
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id("root"))
            .expect("root element is required"),
    )
    .render();
}
