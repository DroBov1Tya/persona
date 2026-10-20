#[allow(dead_code)]
#[path = "src/content_schema.rs"]
mod content_schema;

use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=content.json");
    println!("cargo:rerun-if-changed=src/content_schema.rs");

    let source = fs::read_to_string("content.json").expect("content.json must be readable");
    let _: content_schema::SiteContent =
        serde_json::from_str(&source).expect("content.json must match the site content structure");
}
