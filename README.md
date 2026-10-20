# Neil Ermak — Portfolio

A personal portfolio built with Rust, Yew and WebAssembly for static hosting on GitHub Pages.

## Run locally

Install the WebAssembly target and Trunk, then build and serve the finalized static output:

    rustup target add wasm32-unknown-unknown
    cargo install --locked trunk --version 0.21.14
    trunk build --locked --release
    python3 scripts/finalize_csp.py dist/index.html
    python3 -m http.server 8000 --directory dist

## Build

    trunk build --locked --release
    python3 scripts/finalize_csp.py dist/index.html

The static site is written to dist/. Trunk.toml uses a relative public URL so the generated site works at either a user site root or a repository subpath.

## Publish with GitHub Pages

1. Push this project to the main or master branch.
2. In the repository, open **Settings → Pages** and select **GitHub Actions** as the build and deployment source.
3. The workflow in .github/workflows/pages.yml builds the Rust/WASM app and deploys dist/.

The workflow pins each GitHub Action to a commit SHA. It installs the pinned Trunk release with its lockfile.

## Site details

- Edit `content.json` to update the page copy, navigation labels, links, filters, project cards, principles, SEO metadata, and footer. The file has two top-level variants, `en` and `ru`; keep both structures intact. The Rust app embeds this file at build time, and Cargo validates the full schema before compiling.
- Social destinations are in each locale's `links` object; their visible labels are in `contact`.
- Project cards have a short `description`, an expandable `details` paragraph, and a `cover` path. Project `area` values must match a filter `id`. When adding a project or principle, copy an existing object in both locale variants.
- After changing content, run the build commands above and publish the updated `dist/` output. The finalization step copies the English SEO metadata from `content.json` into the static HTML.
- Project descriptions are based on the public GitHub profile and repository READMEs.
- The language control switches between English and Russian.
- Every UI transition is capped at 260 ms; ambient artwork motion is decorative and respects prefers-reduced-motion.
- The site uses a restrictive Content Security Policy meta tag and has no analytics, external fonts, or third-party runtime scripts.
- GitHub Pages does not let a repository set all HTTP response headers. The CSP meta tag covers browser-enforceable directives; HSTS and X-Content-Type-Options require response headers from the hosting layer.
