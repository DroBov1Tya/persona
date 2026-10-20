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
    pub switch_to_russian: String,
    pub switch_to_english: String,
    pub toggle_navigation: String,
    pub language_button: String,
    pub open_project: String,
    pub showing_projects: String,
    pub more_work: String,
    pub back_to_top: String,
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
    pub mark: String,
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
    pub art_alt: String,
    pub eyebrow: String,
    pub title_first: String,
    pub title_second: String,
    pub description: String,
    pub primary_action: String,
    pub linkedin_action: String,
    pub focus_label: String,
    pub focus_value: String,
    pub building_label: String,
    pub building_value: String,
    pub caption_index: String,
    pub caption: String,
    pub scroll_label: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Work {
    pub eyebrow: String,
    pub title_first: String,
    pub title_second: String,
    pub description: String,
    pub filter_label: String,
    pub details_toggle: String,
    pub filters: Vec<Filter>,
    pub all_projects_action: String,
    pub projects: Vec<Project>,
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
    pub mark: String,
    pub stack: String,
    pub url: String,
    pub index: String,
    pub description: String,
    pub details: String,
    pub cover: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct About {
    pub eyebrow: String,
    pub title_first: String,
    pub title_second: String,
    pub description: String,
    pub principles: Vec<Principle>,
    pub ticker: String,
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
    pub eyebrow: String,
    pub title_first: String,
    pub title_second: String,
    pub description: String,
    pub action: String,
    pub linkedin_label: String,
    pub github_label: String,
    pub telegram_label: String,
    pub telegram_channel_label: String,
    pub social_links_label: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Footer {
    pub tagline: String,
    pub copyright: String,
}
