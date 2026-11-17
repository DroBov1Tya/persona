use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteContent {
    pub en: Locale,
    pub ru: Locale,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Locale {
    pub seo: Seo,
    pub common: Common,
    pub links: Links,
    pub header: Header,
    pub hero: Hero,
    pub work: Work,
    pub about: About,
    pub contact: Contact,
    pub footer: Footer,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Seo {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Common {
    pub skip_to_content: String,
    pub home: String,
    pub main_navigation: String,
    pub switch_language: String,
    pub language_button: String,
    pub showing_projects: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Links {
    pub linkedin: String,
    pub github: String,
    pub telegram: String,
    pub telegram_channel: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    pub brand: String,
    pub navigation: Vec<NavItem>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NavItem {
    pub id: String,
    pub label: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hero {
    pub name: String,
    pub role: String,
    pub description: String,
    pub primary_action: String,
    pub github_action: String,
    pub linkedin_action: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Work {
    pub title: String,
    pub description: String,
    pub featured: Featured,
    pub filter_label: String,
    pub details_toggle: String,
    pub filters: Vec<Filter>,
    pub projects: Vec<Project>,
    pub more_work: String,
    pub all_projects_action: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Featured {
    pub label: String,
    pub name: String,
    pub stack: String,
    pub description: String,
    pub image: String,
    pub image_alt: String,
    pub links: Vec<ExternalLink>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalLink {
    pub label: String,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    pub id: String,
    pub label: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub name: String,
    pub area: String,
    pub area_label: String,
    pub stack: String,
    pub url: String,
    pub description: String,
    pub details: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct About {
    pub title: String,
    pub description: String,
    pub principles: Vec<Principle>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Principle {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contact {
    pub title: String,
    pub description: String,
    pub action: String,
    pub github_label: String,
    pub telegram_label: String,
    pub telegram_channel_label: String,
    pub social_links_label: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Footer {
    pub tagline: String,
    pub back_to_top: String,
}
