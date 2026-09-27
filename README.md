# Py4_ blog

The blog, editable content, theme, and compiler live in this repository:

| Directory | Purpose |
| --- | --- |
| `src/` | Original HTML pages kept as migration references, plus static assets |
| `content/` | Editable Shredder Markdown source for every page and post |
| `theme/` | Editable HTML layouts and custom block templates |
| `shredder/` | Rust static site compiler |
| `dist/` | Generated website to preview or deploy |

Build from the repository root with:

```sh
cargo run --release --manifest-path shredder/Cargo.toml --
```

Preview with `python3 preview.py` and open `http://127.0.0.1:8000/`. The preview server resolves extensionless links such as `/about` to `about.html`.

The existing `package.json` and `build.mjs` still build from the original HTML pages for the current deployment. To publish the Markdown version, switch the hosting build command to `cargo run --release --manifest-path shredder/Cargo.toml --` and keep `dist` as the output directory. That switch should happen after the new site has been reviewed and the host provides Rust.

## Shredder Markdown

Each `content/posts/*.md` or `content/pages/*.md` file starts with YAML front matter between `---` lines, followed by the body. A post looks like:

```md
---
title: An Example
slug: an-example
layout: post
date: 2026-09-26
date_display: September 26, 2026
description: A short summary for search results and the feed.
---

## A section

Regular **Markdown** goes here. Tables, task lists, fenced code blocks,
links, quotes, and inline HTML are supported.
```

`slug` is the output filename without `.html`. It must be unique and contain only ASCII letters, digits, or hyphens. `title` and `layout` are required. The `layout` name selects `theme/layouts/<layout>.html`, so adding a design only requires a new template file. Existing layouts are `post`, `box`, `index`, `photography`, `places`, and `raw`. The `format` field defaults to `markdown`; `html` and `raw` pass the body through unchanged. The `places` page uses `html` for its SVG map, while `404` uses `raw` for plain text.

Optional common fields include `description`, `nav_active`, and `font` (`sans` for a post; otherwise serif). Posts use `date` in `YYYY-MM-DD` form and `date_display` for the visible date. The home page's `posts` list controls the visible order and lets older posts remain published without appearing there. The photography page's `photos` list holds entries with `location`, `date`, `image`, `alt`, `headline`, and optional `full_width`. Additional scalar YAML fields can be used as placeholders in layout templates.

The layout templates accept `{{title}}`, `{{page_title}}`, `{{description}}`, `{{slug}}`, `{{url_path}}`, `{{date}}`, `{{date_display}}`, `{{content}}`, `{{nav}}`, `{{posts}}`, `{{photos}}`, and `{{font_family}}`. Scalar front matter fields are also available as placeholders. HTML and CSS live in the theme, so a redesign does not need a Rust change. `theme/blocks/post-card.html` and `photo-band.html` control the repeated home and photography entries.

Custom Markdown blocks use a matching file in `theme/blocks/`. For example, `:::callout` renders through `theme/blocks/callout.html`, replacing `{{body}}` with rendered Markdown:

```md
:::callout
This is **custom** content.
:::
```

Blocks cannot nest. Inline HTML remains available for one-off pieces such as the contact image. The original HTML files are retained as migration references and as the source of static assets.

The compiler also accepts `--content DIR`, `--output DIR`, `--assets DIR`, and `--theme DIR` to override its sibling directory defaults. It writes HTML, a sitemap, an RSS feed, and static files to the output directory. It does not delete the output directory, so remove obsolete generated files yourself if you delete a source page.
