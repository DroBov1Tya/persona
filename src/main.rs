mod content_schema;

use content_schema::{Locale, Project, SiteContent};
use std::sync::OnceLock;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Language {
    En,
    Ru,
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

#[function_component(App)]
fn app() -> Html {
    let lang = use_state(|| Language::En);
    let filter = use_state(|| "All".to_owned());
    let menu_open = use_state(|| false);
    let data = locale(*lang);
    let ru = *lang == Language::Ru;
    let visible: Vec<&Project> = data
        .work
        .projects
        .iter()
        .filter(|project| *filter == "All" || project.area == *filter)
        .collect();

    use_effect_with(*lang, |language| {
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            if let Some(root) = document.document_element() {
                let _ = root.set_attribute(
                    "lang",
                    if *language == Language::Ru {
                        "ru"
                    } else {
                        "en"
                    },
                );
            }
            document.set_title(&locale(*language).seo.title);
            if let Ok(Some(description)) = document.query_selector("meta[name='description']") {
                let _ = description.set_attribute("content", &locale(*language).seo.description);
            }
        }
        || ()
    });

    let toggle_language = {
        let lang = lang.clone();
        Callback::from(move |_| {
            lang.set(if *lang == Language::En {
                Language::Ru
            } else {
                Language::En
            });
        })
    };
    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };
    let close_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(false))
    };

    html! {
        <>
            <a class="skip-link" href="#main">{&data.common.skip_to_content}</a>
            <header class="topbar">
                <a class="wordmark" href="#top" aria-label={data.common.home.as_str()}>
                    <span class="wordmark-symbol">{&data.header.mark}</span>
                    <span>{&data.header.brand}</span>
                </a>
                <nav class={classes!("nav-links", (*menu_open).then_some("is-open"))} aria-label={data.common.main_navigation.as_str()}>
                    {for data.header.navigation.iter().map(|item| html! {
                        <a href={format!("#{}", item.id)} onclick={close_menu.clone()}>{&item.label}</a>
                    })}
                </nav>
                <div class="top-actions">
                    <button class="lang-toggle" type="button" onclick={toggle_language} aria-label={if ru { data.common.switch_to_english.as_str() } else { data.common.switch_to_russian.as_str() }}>
                        {&data.common.language_button}
                    </button>
                    <button class="menu-toggle" type="button" onclick={toggle_menu} aria-expanded={if *menu_open { "true" } else { "false" }} aria-label={data.common.toggle_navigation.as_str()}>
                        <span></span><span></span>
                    </button>
                </div>
            </header>

            <main id="main">
                <section class="hero" id="top" aria-labelledby="hero-title">
                    <div class="hero-art" role="img" aria-label={data.hero.art_alt.as_str()}></div>
                    <div class="hero-shade"></div>
                    <div class="hero-content">
                        <div class="eyebrow"><span class="status-dot"></span>{&data.hero.eyebrow}</div>
                        <h1 id="hero-title">{&data.hero.title_first}<br/><em>{&data.hero.title_second}</em></h1>
                        <p class="hero-copy">{&data.hero.description}</p>
                        <div class="hero-cta">
                            <a class="button button-primary" href="#work">{&data.hero.primary_action}<span aria-hidden="true">{" ↘"}</span></a>
                            <a class="text-link" href={data.links.linkedin.clone()} target="_blank" rel="noopener noreferrer">{&data.hero.linkedin_action}</a>
                        </div>
                        <div class="hero-meta">
                            <div><span class="meta-label">{&data.hero.focus_label}</span><span>{&data.hero.focus_value}</span></div>
                            <div><span class="meta-label">{&data.hero.building_label}</span><span>{&data.hero.building_value}</span></div>
                        </div>
                    </div>
                    <div class="hero-caption"><span>{&data.hero.caption_index}</span>{&data.hero.caption}</div>
                    <a class="scroll-cue" href="#work" aria-label={data.hero.scroll_label.as_str()}>{"↓"}</a>
                </section>

                <section class="work-section section-wrap" id="work" aria-labelledby="work-title">
                    <div class="section-heading">
                        <div>
                            <p class="eyebrow eyebrow-dark">{&data.work.eyebrow}</p>
                            <h2 id="work-title">{&data.work.title_first}<br/><em>{&data.work.title_second}</em></h2>
                        </div>
                        <p class="section-intro">{&data.work.description}</p>
                    </div>
                    <div class="filter-row" role="group" aria-label={data.work.filter_label.as_str()}>
                        {for data.work.filters.iter().map(|item| {
                            let id = item.id.clone();
                            let active = *filter == item.id;
                            let filter = filter.clone();
                            html! {
                                <button class={classes!("filter-chip", active.then_some("active"))} type="button" aria-pressed={if active { "true" } else { "false" }} onclick={Callback::from(move |_| filter.set(id.clone()))}>{&item.label}</button>
                            }
                        })}
                    </div>
                    <p class="sr-only" aria-live="polite">{&data.common.showing_projects.replace("{count}", &visible.len().to_string())}</p>
                    <div class="project-grid">
                        {for visible.iter().map(|project| html! {
                            <article class="project-card">
                                <a class="project-cover-link" href={project.url.clone()} target="_blank" rel="noopener noreferrer" aria-label={project.name.as_str()}>
                                  <div class="project-visual">
                                    <img class="project-cover" src={project.cover.clone()} alt="" loading="lazy" decoding="async" />
                                    <span class="project-mark">{&project.mark}</span>
                                    <span class="project-index">{&project.index}</span>
                                    <span class="visual-label">{&project.area_label.to_uppercase()}</span>
                                  </div>
                                </a>
                                <div class="project-info">
                                    <div class="project-title-line"><h3><a href={project.url.clone()} target="_blank" rel="noopener noreferrer">{&project.name}</a></h3><a href={project.url.clone()} target="_blank" rel="noopener noreferrer" aria-label={project.name.as_str()}><span aria-hidden="true">{"↗"}</span></a></div>
                                    <p>{&project.description}</p>
                                    <details class="project-details">
                                        <summary>{&data.work.details_toggle}</summary>
                                        <div class="project-details-content"><p>{&project.details}</p></div>
                                    </details>
                                    <div class="project-foot"><span>{&project.stack}</span><span>{&data.common.open_project}</span></div>
                                </div>
                            </article>
                        })}
                    </div>
                    <div class="all-work">
                        <span>{&data.common.more_work}</span>
                        <a class="text-link dark-link" href={data.links.github.clone()} target="_blank" rel="noopener noreferrer">{&data.work.all_projects_action}</a>
                    </div>
                </section>

                <section class="approach-section" id="about" aria-labelledby="about-title">
                    <div class="section-wrap approach-inner">
                        <div class="approach-intro">
                            <p class="eyebrow">{&data.about.eyebrow}</p>
                            <h2 id="about-title">{&data.about.title_first}<br/><em>{&data.about.title_second}</em></h2>
                            <p>{&data.about.description}</p>
                        </div>
                        <div class="principles">
                            {for data.about.principles.iter().enumerate().map(|(index, item)| html! {
                                <article class="principle"><span class="principle-no">{format!("{:02}", index + 1)}</span><div><h3>{&item.title}</h3><p>{&item.description}</p></div><span class="principle-arrow" aria-hidden="true">{"↘"}</span></article>
                            })}
                        </div>
                    </div>
                    <div class="ticker" aria-hidden="true"><div class="ticker-track"><span>{&data.about.ticker}</span></div></div>
                </section>

                <section class="contact-section section-wrap" id="contact" aria-labelledby="contact-title">
                    <div class="contact-orb" aria-hidden="true"></div>
                    <p class="eyebrow eyebrow-dark">{&data.contact.eyebrow}</p>
                    <h2 id="contact-title">{&data.contact.title_first}<br/><em>{&data.contact.title_second}</em></h2>
                    <p class="contact-copy">{&data.contact.description}</p>
                    <div class="contact-links">
                        <a class="button button-dark" href={data.links.linkedin.clone()} target="_blank" rel="noopener noreferrer">{&data.contact.action}{" ↗"}</a>
                        <nav class="social-links" aria-label={data.contact.social_links_label.as_str()}>
                            <a class="social-link" href={data.links.github.clone()} target="_blank" rel="noopener noreferrer">{&data.contact.github_label}</a>
                            <a class="social-link" href={data.links.linkedin.clone()} target="_blank" rel="noopener noreferrer">{&data.contact.linkedin_label}</a>
                            <a class="social-link" href={data.links.telegram.clone()} target="_blank" rel="noopener noreferrer">{&data.contact.telegram_label}</a>
                            <a class="social-link" href={data.links.telegram_channel.clone()} target="_blank" rel="noopener noreferrer">{&data.contact.telegram_channel_label}</a>
                        </nav>
                    </div>
                </section>
            </main>

            <footer class="footer">
                <a class="wordmark footer-mark" href="#top"><span class="wordmark-symbol">{&data.header.mark}</span><span>{&data.header.brand}</span></a>
                <span>{&data.footer.tagline}</span>
                <a href="#top">{&data.common.back_to_top}</a>
                <span class="copyright">{&data.footer.copyright}</span>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::with_root(
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id("root"))
            .expect("root element is required"),
    )
    .render();
}
