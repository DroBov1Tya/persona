#!/usr/bin/env python3
"""Pin Trunk's single generated module bootstrap to its CSP SHA-256 hash."""

import base64
import html
import hashlib
import json
import re
import sys
from pathlib import Path


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: finalize_csp.py dist/index.html")

    path = Path(sys.argv[1])
    document = path.read_text(encoding="utf-8")
    project_root = Path(__file__).resolve().parent.parent
    content = json.loads((project_root / "content.json").read_text(encoding="utf-8"))
    title_pattern = re.compile(r"(<title>).*?(</title>)", flags=re.IGNORECASE | re.DOTALL)
    document, title_count = title_pattern.subn(
        lambda match: f"{match.group(1)}{html.escape(content['en']['seo']['title'])}{match.group(2)}",
        document,
    )
    if title_count != 1:
        raise SystemExit(f"expected one title element, found {title_count}")

    description_pattern = re.compile(
        r'(<meta\b[^>]*\bname="description"[^>]*\bcontent=")([^"]*)(")',
        flags=re.IGNORECASE,
    )
    document, description_count = description_pattern.subn(
        lambda match: f"{match.group(1)}{html.escape(content['en']['seo']['description'], quote=True)}{match.group(3)}",
        document,
    )
    if description_count != 1:
        raise SystemExit(f"expected one meta description, found {description_count}")

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
    print(f"Added Trunk bootstrap hash to {path}")


if __name__ == "__main__":
    main()
