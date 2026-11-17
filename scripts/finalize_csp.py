#!/usr/bin/env python3
"""Finalize dist/index.html after `trunk build`.

Writes the English SEO metadata and the static first-paint fallback from content.json,
then pins Trunk's single generated module bootstrap to its CSP SHA-256 hash.
"""

import base64
import html
import hashlib
import json
import re
import sys
from pathlib import Path

# Meta tags that carry copy: (attribute, attribute value, key under "seo").
SEO_META = (
    ("name", "description", "description"),
    ("property", "og:title", "title"),
    ("property", "og:description", "description"),
)


def lookup(content: dict, path: str) -> str:
    """Resolve a dotted path such as `header.navigation.0.label` to a string."""
    node = content
    for part in path.split("."):
        try:
            node = node[int(part)] if isinstance(node, list) else node[part]
        except (KeyError, IndexError, ValueError):
            raise SystemExit(f"content.json has no value at en.{path}") from None
    if not isinstance(node, str):
        raise SystemExit(f"en.{path} in content.json is not a string")
    return node


def set_meta(document: str, attribute: str, name: str, value: str) -> str:
    pattern = re.compile(
        rf'(<meta\b[^>]*\b{attribute}="{re.escape(name)}"[^>]*\bcontent=")([^"]*)(")',
        flags=re.IGNORECASE,
    )
    document, count = pattern.subn(
        lambda match: f"{match.group(1)}{html.escape(value, quote=True)}{match.group(3)}",
        document,
    )
    if count != 1:
        raise SystemExit(f'expected one meta {attribute}="{name}", found {count}')
    return document


def fill_fallback(document: str, content: dict) -> str:
    """Rewrite each data-content text and data-href link; every marker must be filled."""
    text_pattern = re.compile(
        r'(<(?P<tag>[a-z0-9]+)\b[^<>]*\bdata-content="(?P<path>[\w.]+)"[^<>]*>)[^<]*(</(?P=tag)>)'
    )
    document, text_count = text_pattern.subn(
        lambda match: f"{match.group(1)}{html.escape(lookup(content, match.group('path')))}{match.group(4)}",
        document,
    )
    href_pattern = re.compile(r'(<a\b[^<>]*\bhref=")[^"]*("[^<>]*\bdata-href="(?P<path>[\w.]+)")')
    document, href_count = href_pattern.subn(
        lambda match: f"{match.group(1)}{html.escape(checked_url(lookup(content, match.group('path'))), quote=True)}{match.group(2)}",
        document,
    )
    # The explanatory comment in index.html mentions both marker names without `=`.
    expected_text = document.count("data-content=")
    expected_href = document.count("data-href=")
    if text_count == 0 or text_count != expected_text:
        raise SystemExit(f"filled {text_count} of {expected_text} data-content fallback elements")
    if href_count != expected_href:
        raise SystemExit(f"filled {href_count} of {expected_href} data-href fallback links")
    return document


def checked_url(url: str) -> str:
    if not url.startswith("https://"):
        raise SystemExit(f"fallback link must be https: {url}")
    return url


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: finalize_csp.py dist/index.html")

    path = Path(sys.argv[1])
    document = path.read_text(encoding="utf-8")
    project_root = Path(__file__).resolve().parent.parent
    content = json.loads((project_root / "content.json").read_text(encoding="utf-8"))["en"]
    title_pattern = re.compile(r"(<title>).*?(</title>)", flags=re.IGNORECASE | re.DOTALL)
    document, title_count = title_pattern.subn(
        lambda match: f"{match.group(1)}{html.escape(content['seo']['title'])}{match.group(2)}",
        document,
    )
    if title_count != 1:
        raise SystemExit(f"expected one title element, found {title_count}")

    for attribute, name, key in SEO_META:
        document = set_meta(document, attribute, name, content["seo"][key])
    document = fill_fallback(document, content)

    scripts = re.findall(
        r"<script\b(?=[^>]*\btype=[\"']module[\"'])(?![^>]*\bsrc=)[^>]*>(.*?)</script\s*>",
        document,
        flags=re.IGNORECASE | re.DOTALL,
    )
    if len(scripts) != 1:
        raise SystemExit(f"expected exactly one inline Trunk module bootstrap, found {len(scripts)}")

    bootstrap = scripts[0]
    expected_bootstrap = re.compile(
        r"\nimport init, \* as bindings from '\./"
        r"(?P<stem>neil-ermak-portfolio-[0-9a-f]{16})\.js';\n"
        r"const wasm = await init\(\{ module_or_path: '\./(?P=stem)_bg\.wasm' \}\);\n\n\n"
        r"window\.wasmBindings = bindings;\n\n\n"
        r'dispatchEvent\(new CustomEvent\("TrunkApplicationStarted", \{detail: \{wasm\}\}\)\);\n\n'
    )
    if expected_bootstrap.fullmatch(bootstrap) is None:
        raise SystemExit("the inline module does not match the pinned Trunk bootstrap template")

    digest = base64.b64encode(hashlib.sha256(bootstrap.encode("utf-8")).digest()).decode("ascii")
    hash_source = f"'sha256-{digest}'"
    csp_meta = re.compile(
        r'(<meta\b[^>]*\bhttp-equiv="Content-Security-Policy"[^>]*\bcontent=")([^"]*)(")',
        flags=re.IGNORECASE,
    )
    matches = list(csp_meta.finditer(document))
    if len(matches) != 1:
        raise SystemExit("Content-Security-Policy meta tag with script-src was not found")

    match = matches[0]
    policy = match.group(2)
    directives = [
        directive.strip()
        for directive in policy.split(";")
        if directive.strip().lower().startswith("script-src")
    ]
    expected_directive = "script-src 'self' 'wasm-unsafe-eval'"
    if directives != [expected_directive]:
        raise SystemExit("Content-Security-Policy script-src must match the strict expected policy")
    updated_policy = policy.replace(expected_directive, f"{expected_directive} {hash_source}", 1)
    updated = (
        document[: match.start(2)]
        + updated_policy
        + document[match.end(2) :]
    )
    path.write_text(updated, encoding="utf-8")
    print(f"Filled the static fallback and added the Trunk bootstrap hash to {path}")


if __name__ == "__main__":
    main()
