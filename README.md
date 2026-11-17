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

- The page shares its visual language with the [duskcode site](https://drobov1tya.github.io/dusk_code/): the same dark palette, one violet accent, the system sans stack with `ui-monospace` for small labels, and the dusk sky with stars behind the header. The tokens are at the top of `style.css`.
- Edit `content.json` to update the page copy, navigation labels, links, filters, the featured project, project cards, principles, SEO metadata, and footer. The file has two top-level variants, `en` and `ru`; keep both structures intact. The Rust app embeds this file at build time, and Cargo validates the full schema (`src/content_schema.rs`, unknown fields are refused) before compiling.
- Social destinations are in each locale's `links` object; their visible labels are in `hero` and `contact`.
- `work.featured` is the wide card for Dusk Code: a label, name, stack line, one paragraph, a screenshot (`assets/covers/dusk-code.webp`, converted from the duskcode desktop screenshot) and a list of links.
- Project cards are typographic and have no cover image: an area, a name, a short `description`, an expandable `details` paragraph and a `stack` line. Project `area` values must match a filter `id`. When adding a project or principle, copy an existing object in both locale variants.
- `index.html` carries a static copy of the header and hero inside `#root`, so the first paint and crawlers get real content before the WebAssembly loads. Yew clears `#root` when it mounts, so the copy is replaced, not duplicated. Its markup must mirror the header in `src/main.rs`. Its text comes from `content.json`: the finalization step rewrites every element marked `data-content="path.in.en.locale"` and every link marked `data-href`, and fails if a marker cannot be filled. The hidden `aria-label` on the fallback navigation is the only hand-written copy.
- After changing content, run the build commands above and publish the updated `dist/` output. The finalization step also copies the English SEO title and description from `content.json` into the title, the meta description and the Open Graph tags.
- Open Graph and canonical URLs are absolute and point at `https://drobov1tya.github.io/persona/`; change them in `index.html` if the site moves. The preview image is `assets/og.jpg` (1200x630).
- Project descriptions are based on the public GitHub profile and repository READMEs.
- The language control switches between English and Russian. Below 560 px the section links in the header are hidden; the page is short enough to scroll.
- There is no ambient motion. Hover and focus transitions take 160 ms and are switched off under `prefers-reduced-motion`.
- The site uses a restrictive Content Security Policy meta tag and has no analytics, external fonts, or third-party runtime scripts. `style-src 'self'` forbids inline styles, so the stars are drawn with `box-shadow` in the stylesheet.
- GitHub Pages does not let a repository set all HTTP response headers. The CSP meta tag covers browser-enforceable directives; HSTS and X-Content-Type-Options require response headers from the hosting layer.
