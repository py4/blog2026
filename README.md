# Py4_ blog

The blog, editable content, theme, and compiler live in this repository:

| Directory | Purpose |
| --- | --- |
| `src/` | Static assets copied into the generated site |
| `content/` | Editable Shredder Markdown source for every page and post |
| `site.yaml` | Site identity (name, tagline, description, url) and the active `theme:` pointer |
| `themes/_shared/` | Default `layouts/` and `blocks/` inherited by every theme |
| `themes/` | One directory per theme: `style.css` plus any `layouts/`/`blocks/` it overrides |
| `shredder/` | Rust static site compiler |
| `dist/` | Generated website to preview or deploy |

Build from the repository root with:

```sh
bash build.sh
```

Preview with `python3 preview.py` and open `http://127.0.0.1:8000/`. The preview server resolves extensionless links such as `/about` to `about.html`.

The active theme is selected by the `theme:` field in the root `site.yaml`; it currently points at `themes/classic`. Set it to `themes/yellow` to switch back to the original yellow theme. Passing `--theme DIR` on the compiler command overrides this for a one-off build.

A theme only needs to provide `style.css`; layouts and blocks come from `themes/_shared/` unless the theme supplies its own copy of that file. So `themes/classic/` is just a stylesheet, while `themes/yellow/` overrides every layout (including its own `photography.html`).

The classic theme keeps the yellow photography layout for now; the longest post currently exceeds the 14 KB HTML target.

The existing Cloudflare Pages build command, `npm ci && npm run build`, works during the migration: `package.json` is now a small bridge to `build.sh`, with no Node build dependencies. When the Cloudflare dashboard is available, simplify the build command to `bash build.sh`; keep `dist` as the output directory and the repository root as the root directory. The script installs Rust if the build image does not provide it, then runs Shredder with the checked-in dependency lockfile. The generated `dist/` stays ignored by Git.

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

`slug` is the output filename without `.html`. It must be unique and contain only ASCII letters, digits, or hyphens. `title` and `layout` are required except that the home page inherits its title from the root `site.yaml`. The `layout` name resolves to `<theme-dir>/layouts/<layout>.html` if the theme has it, otherwise `themes/_shared/layouts/<layout>.html`. Existing layouts are `post`, `box`, `index`, `photography`, `places`, and `raw`. The `format` field defaults to `markdown`; `html` and `raw` pass the body through unchanged. The `places` page uses `html` for its SVG map, while `404` uses `raw` for plain text.

Optional common fields include `description`, `nav_active`, and `font` (`sans` for a post; otherwise serif). A post's optional `ai_use` field renders a note next to the date and defaults to `revised_draft` ("AI Use: Draft fully by me, AI revised it."); set `no_ai` for "No AI use." or `grammer` for "AI Use: Grammer". Posts use `date` in `YYYY-MM-DD` form and `date_display` for the visible date. The home page's `posts` list controls the visible order and lets older posts remain published without appearing there. The photography page's `photos` list holds entries with `location`, `date`, `image`, `alt`, `headline`, and optional `full_width`. Additional scalar YAML fields can be used as placeholders in layout templates.

The layout templates accept `{{title}}`, `{{page_title}}`, `{{description}}`, `{{slug}}`, `{{url_path}}`, `{{date}}`, `{{date_display}}`, `{{content}}`, `{{nav}}`, `{{posts}}`, `{{photos}}`, `{{font_family}}`, `{{site_name}}`, `{{site_tagline}}`, and `{{site_url}}`. Scalar front matter fields are also available as placeholders. Set the shared site name, tagline, description, and URL once in the root `site.yaml`; the home page and feed use them too. HTML and CSS live in the theme, so a redesign does not need a Rust change. `blocks/post-card.html` and `photo-band.html` (resolved from the theme, else `themes/_shared/`) control the repeated home and photography entries.

Custom Markdown blocks use a matching file in `blocks/` (theme first, then `themes/_shared/`). For example, `:::callout` renders through `callout.html`, replacing `{{body}}` with rendered Markdown:

```md
:::callout
This is **custom** content.
:::
```

Blocks cannot nest. Inline HTML remains available for one-off pieces such as the contact image.

The compiler also accepts `--content DIR`, `--output DIR`, `--assets DIR`, and `--theme DIR` to override its sibling directory defaults. It writes HTML, a sitemap, an RSS feed, and static files to the output directory. It does not delete the output directory, so remove obsolete generated files yourself if you delete a source page.
