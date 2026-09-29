use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use chrono::{NaiveDate, Utc};
use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};
use serde::Deserialize;
use serde_yaml::Value;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct Page {
    #[serde(default)]
    title: String,
    slug: String,
    layout: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    date: String,
    #[serde(default)]
    date_display: String,
    #[serde(default)]
    nav_active: String,
    #[serde(default)]
    font: String,
    #[serde(default)]
    format: String,
    #[serde(default)]
    posts: Vec<PostCard>,
    #[serde(default)]
    photos: Vec<Photo>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct Site {
    name: String,
    tagline: String,
    description: String,
    url: String,
    #[serde(default)]
    theme: Option<String>,
}

impl Site {
    fn title(&self) -> String {
        format!("{} - {}", self.name, self.tagline)
    }
}

#[derive(Debug, Deserialize)]
struct PostCard {
    slug: String,
    date: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct Photo {
    location: String,
    date: String,
    image: String,
    alt: String,
    headline: String,
    #[serde(default)]
    full_width: bool,
}

struct Markdown {
    syntax: SyntaxSet,
    themes: ThemeSet,
}

impl Markdown {
    fn new() -> Self {
        Self {
            syntax: SyntaxSet::load_defaults_newlines(),
            themes: ThemeSet::load_defaults(),
        }
    }

    fn code(&self, source: &str, language: &str) -> String {
        let syntax = self
            .syntax
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax.find_syntax_plain_text());
        let theme = self.themes.themes.get("InspiredGitHub").unwrap();
        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut out = String::new();
        for line in syntect::util::LinesWithEndings::from(source) {
            match highlighter
                .highlight_line(line, &self.syntax)
                .ok()
                .and_then(|ranges| {
                    styled_line_to_highlighted_html(&ranges, IncludeBackground::No).ok()
                }) {
                Some(markup) => out.push_str(&markup),
                None => out.push_str(&escape(line)),
            }
        }
        out
    }

    fn render(&self, source: &str) -> String {
        let options =
            Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
        let mut in_code = false;
        let mut language = String::new();
        let mut code = String::new();
        let mut events = Vec::new();
        for event in Parser::new_ext(source, options) {
            match event {
                Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(ref lang))) => {
                    in_code = true;
                    language = lang.to_string();
                    code.clear();
                }
                Event::End(TagEnd::CodeBlock) if in_code => {
                    in_code = false;
                    events.push(Event::Html(
                        format!(
                            "<pre class=\"b\"><code class=\"language-{}\">{}</code></pre>",
                            escape(&language),
                            self.code(&code, &language)
                        )
                        .into(),
                    ));
                }
                Event::Text(text) if in_code => code.push_str(&text),
                Event::Start(Tag::BlockQuote(_)) => {
                    events.push(Event::Html("<blockquote class=\"bl bgg\">".into()))
                }
                Event::End(TagEnd::BlockQuote(_)) => {
                    events.push(Event::Html("</blockquote>".into()))
                }
                other => events.push(other),
            }
        }
        let mut out = String::new();
        html::push_html(&mut out, events.into_iter());
        out
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn read_page(path: &Path) -> (Page, String) {
    let source = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let source = source.replace("\r\n", "\n");
    let rest = source
        .strip_prefix("---\n")
        .unwrap_or_else(|| panic!("{}: missing YAML front matter", path.display()));
    let (front, body) = rest
        .split_once("\n---\n")
        .unwrap_or_else(|| panic!("{}: unterminated YAML front matter", path.display()));
    let page: Page = serde_yaml::from_str(front)
        .unwrap_or_else(|e| panic!("{}: invalid front matter: {e}", path.display()));
    assert!(
        !page.title.is_empty() || page.slug == "index",
        "{}: title is required",
        path.display()
    );
    assert!(safe_name(&page.slug), "{}: invalid slug", path.display());
    assert!(
        safe_name(&page.layout),
        "{}: invalid layout",
        path.display()
    );
    (page, body.to_string())
}

fn safe_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

fn fill(template: &str, values: &HashMap<String, String>) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let tail = &rest[start + 2..];
        if let Some(end) = tail.find("}}") {
            let key = &tail[..end];
            if let Some(value) = values.get(key) {
                out.push_str(value);
            } else {
                out.push_str(&rest[start..start + end + 4]);
            }
            rest = &tail[end + 2..];
        } else {
            rest = &rest[start..];
            break;
        }
    }
    out.push_str(rest);
    out
}

fn template(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("template {}: {e}", path.display()))
}

fn themed(theme: &Path, kind: &str, name: &str) -> String {
    let own = theme.join(kind).join(name);
    if own.exists() {
        template(&own)
    } else {
        let shared = theme
            .parent()
            .unwrap()
            .join("_shared")
            .join(kind)
            .join(name);
        template(&shared)
    }
}

fn custom_blocks(markdown: &Markdown, source: &str, theme: &Path) -> String {
    let mut lines = source.lines();
    let mut out = String::new();
    while let Some(line) = lines.next() {
        if let Some(name) = line.trim().strip_prefix(":::") {
            let name = name.trim();
            if !name.is_empty() {
                assert!(safe_name(name), "invalid custom block name: {name}");
                let mut body = String::new();
                let mut closed = false;
                for inner in lines.by_ref() {
                    if inner.trim() == ":::" {
                        closed = true;
                        break;
                    }
                    body.push_str(inner);
                    body.push('\n');
                }
                assert!(closed, "unclosed custom block: {name}");
                let block = themed(theme, "blocks", &format!("{name}.html"));
                out.push_str("\n\n");
                out.push_str(&block.replace("{{body}}", &markdown.render(&body)));
                out.push_str("\n\n");
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn nav(active: &str) -> String {
    let mut out = String::from("<nav>\n");
    for (slug, label) in [
        ("index", "Home"),
        ("about", "About"),
        ("contact", "Contact"),
        ("places", "Places"),
        ("now", "Now"),
        ("photography", "Photography"),
    ] {
        let class = if slug == active {
            "btn btn-active ss f9 tu bgb b3 tr"
        } else {
            "btn ss f9 tu bg b3 tr"
        };
        out.push_str(&format!(
            "<a href=\"{slug}\" class=\"{class}\">{label}</a>\n"
        ));
    }
    out.push_str("</nav>");
    out
}

fn cards(items: &[PostCard], theme: &Path) -> String {
    if items.is_empty() {
        return String::new();
    }
    let template = themed(theme, "blocks", "post-card.html");
    items
        .iter()
        .map(|post| {
            fill(
                &template,
                &HashMap::from([
                    ("date".into(), escape(&post.date)),
                    ("slug".into(), escape(&post.slug)),
                    ("title".into(), escape(&post.title)),
                ]),
            )
        })
        .collect()
}

fn photo_bands(items: &[Photo], theme: &Path) -> String {
    if items.is_empty() {
        return String::new();
    }
    let template = themed(theme, "blocks", "photo-band.html");
    items
        .iter()
        .map(|photo| {
            fill(
                &template,
                &HashMap::from([
                    ("location".into(), escape(&photo.location)),
                    ("date".into(), escape(&photo.date)),
                    ("image".into(), escape(&photo.image)),
                    ("alt".into(), escape(&photo.alt)),
                    ("headline".into(), photo.headline.clone()),
                    (
                        "full_width".into(),
                        if photo.full_width { " full-width" } else { "" }.into(),
                    ),
                ]),
            )
        })
        .collect()
}

fn build(
    page: &Page,
    body: &str,
    markdown: &Markdown,
    theme: &Path,
    site: &Site,
) -> (String, String) {
    let title = if page.slug == "index" && page.title.is_empty() {
        site.title()
    } else {
        page.title.clone()
    };
    let description = if page.slug == "index" && page.description.is_empty() {
        &site.description
    } else {
        &page.description
    };
    let content = match page.format.as_str() {
        "" | "markdown" => markdown.render(&custom_blocks(markdown, body, theme)),
        "html" | "raw" => body.to_string(),
        other => panic!("{}: unsupported format {other}", page.slug),
    };
    let content = match page.layout.as_str() {
        "post" => content
            .replace("<h2>", "<h2 class=\"im f9 tu bb4\">")
            .replace("<h3>", "<h3 class=\"im tu bb4\">"),
        "box" => content
            .replace("<h2>", "<h2 class=\"im f9 tu bg b6\">")
            .replace("<h3>", "<h3 class=\"im f9 tu bb4\">"),
        _ => content,
    };
    let mut values = HashMap::from([
        ("title".into(), escape(&title)),
        (
            "page_title".into(),
            escape(&if page.layout == "post" {
                format!("{} - {}", title, site.name)
            } else {
                title
            }),
        ),
        ("site_name".into(), escape(&site.name)),
        ("site_tagline".into(), escape(&site.tagline)),
        ("site_url".into(), escape(site.url.trim_end_matches('/'))),
        ("slug".into(), escape(&page.slug)),
        (
            "url_path".into(),
            if page.slug == "index" {
                "".into()
            } else {
                escape(&page.slug)
            },
        ),
        ("description".into(), escape(description)),
        ("date".into(), escape(&page.date)),
        ("date_display".into(), escape(&page.date_display)),
        (
            "font_family".into(),
            if page.font == "sans" {
                "-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif"
            } else {
                "Georgia,serif"
            }
            .into(),
        ),
        ("content".into(), content.clone()),
        (
            "nav".into(),
            nav(if page.nav_active.is_empty() {
                &page.slug
            } else {
                &page.nav_active
            }),
        ),
        ("posts".into(), cards(&page.posts, theme)),
        ("photos".into(), photo_bands(&page.photos, theme)),
    ]);
    for (key, value) in &page.extra {
        if let Some(s) = value.as_str() {
            values.insert(key.clone(), escape(s));
        }
    }
    values.insert(
        "ai_use".into(),
        match page.extra.get("ai_use").and_then(Value::as_str) {
            None | Some("revised_draft") => {
                "AI Use: Draft fully by me, AI revised it.".to_string()
            }
            Some("no_ai") => "No AI use.".to_string(),
            Some("grammer") => "AI Use: Grammer".to_string(),
            Some(other) => panic!("{}: unsupported ai_use {other}", page.slug),
        },
    );
    let layout = themed(theme, "layouts", &format!("{}.html", page.layout));
    let style = theme.join("style.css");
    values.insert(
        "stylesheet".into(),
        if style.exists() {
            template(&style)
        } else {
            String::new()
        },
    );
    (fill(&layout, &values), content)
}

fn minify(source: &str) -> Vec<u8> {
    let mut cfg = minify_html::Cfg::new();
    cfg.minify_css = true;
    cfg.minify_js = true;
    minify_html::minify(source.as_bytes(), &cfg)
}

fn rss(posts: &[(Page, String)], out: &Path, site: &Site) {
    let mut items = String::new();
    for (page, content) in posts {
        if page.date.is_empty() {
            continue;
        }
        let date = NaiveDate::parse_from_str(&page.date, "%Y-%m-%d")
            .unwrap_or_else(|e| panic!("{}: invalid date: {e}", page.slug));
        let date = date.and_hms_opt(0, 0, 0).unwrap().and_utc().to_rfc2822();
        let url = format!("{}/{}", site.url.trim_end_matches('/'), page.slug);
        items.push_str(&format!("<item><title>{}</title><link>{url}</link><guid>{url}</guid><pubDate>{date}</pubDate><description>{}</description><content:encoded><![CDATA[{}]]></content:encoded></item>\n",
            escape(&page.title), escape(&page.description), content.replace("]]>", "]]><![CDATA[>")));
    }
    let xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\"><channel><title>{}</title><link>{}/</link><description>{}</description><language>en-us</language><lastBuildDate>{}</lastBuildDate><atom:link href=\"{}/feed.xml\" rel=\"self\" type=\"application/rss+xml\"/>{items}</channel></rss>\n", escape(&site.title()), escape(site.url.trim_end_matches('/')), escape(&site.description), Utc::now().to_rfc2822(), escape(site.url.trim_end_matches('/')));
    fs::write(out.join("feed.xml"), xml).expect("cannot write RSS feed");
}

fn sitemap(slugs: &[String], out: &Path, site: &Site) {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">");
    for slug in slugs {
        let path = if slug == "index" { "" } else { slug };
        xml.push_str(&format!(
            "<url><loc>{}/{path}</loc></url>",
            escape(site.url.trim_end_matches('/'))
        ));
    }
    xml.push_str("</urlset>\n");
    fs::write(out.join("sitemap.xml"), xml).expect("cannot write sitemap");
}

fn main() {
    let start = Instant::now();
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut content = workspace.join("content");
    let mut output = workspace.join("dist");
    let mut assets = workspace.join("src");
    let mut theme = PathBuf::new();
    let mut pinned_theme = false;
    let mut args = std::env::args().skip(1);
    while let Some(option) = args.next() {
        if option == "--help" || option == "-h" {
            println!("Usage: shredder [--content DIR] [--output DIR] [--assets DIR] [--theme DIR]");
            return;
        }
        let value = PathBuf::from(
            args.next()
                .unwrap_or_else(|| panic!("{option} needs a directory")),
        );
        match option.as_str() {
            "--content" => content = value,
            "--output" => output = value,
            "--assets" => assets = value,
            "--theme" => {
                theme = value;
                pinned_theme = true;
            }
            _ => panic!("unknown option {option}"),
        }
    }
    for dir in [&content, &assets] {
        assert!(dir.is_dir(), "missing directory: {}", dir.display());
    }
    let site: Site = serde_yaml::from_str(&template(&workspace.join("site.yaml")))
        .expect("invalid site.yaml");
    if !pinned_theme {
        theme = workspace.join(
            site.theme
                .as_deref()
                .expect("site.yaml needs a theme: field"),
        );
    }
    assert!(theme.is_dir(), "missing theme directory: {}", theme.display());
    fs::create_dir_all(&output).expect("cannot create output directory");
    let markdown = Markdown::new();
    let mut slugs = HashSet::new();
    let mut posts = Vec::new();
    for group in ["posts", "pages"] {
        let mut paths: Vec<_> = WalkDir::new(content.join(group))
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
            .map(|e| e.into_path())
            .collect();
        paths.sort();
        for path in paths {
            let (page, body) = read_page(&path);
            assert!(
                slugs.insert(page.slug.clone()),
                "duplicate slug: {}",
                page.slug
            );
            let (html, article) = build(&page, &body, &markdown, &theme, &site);
            let bytes = if page.format == "raw" {
                html.into_bytes()
            } else {
                minify(&html)
            };
            fs::write(output.join(format!("{}.html", page.slug)), bytes)
                .unwrap_or_else(|e| panic!("{}: {e}", page.slug));
            println!("  {}", page.slug);
            if group == "posts" {
                posts.push((page, article));
            }
        }
    }
    posts.sort_by(|a, b| b.0.date.cmp(&a.0.date));
    let mut slugs: Vec<_> = slugs.into_iter().filter(|slug| slug != "404").collect();
    slugs.sort();
    rss(&posts, &output, &site);
    sitemap(&slugs, &output, &site);
    for file in ["robots.txt", "_headers"] {
        let source = assets.parent().unwrap().join(file);
        if source.exists() {
            fs::copy(source, output.join(file)).expect("cannot copy site file");
        }
    }
    for entry in WalkDir::new(&assets)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if !matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" | "pdf" | "txt")
        ) {
            continue;
        }
        let target = output.join(path.strip_prefix(&assets).unwrap());
        fs::create_dir_all(target.parent().unwrap()).expect("cannot create asset directory");
        fs::copy(path, target).expect("cannot copy asset");
    }
    println!("Built {} pages in {:.2?}", slugs.len() + 1, start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternate_theme_inlines_its_stylesheet() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let theme = root.join("themes/classic");
        let site: Site = serde_yaml::from_str(&template(&root.join("site.yaml"))).unwrap();
        let (page, body) = read_page(&root.join("content/pages/index.md"));
        let (html, _) = build(&page, &body, &Markdown::new(), &theme, &site);
        assert!(html.contains("--paper:#f9fff5"));
        assert!(!html.contains("{{stylesheet}}"));
    }

    #[test]
    fn homepage_uses_shared_site_identity() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let theme = root.join("themes/yellow");
        let site: Site = serde_yaml::from_str(&template(&root.join("site.yaml"))).unwrap();
        let (page, body) = read_page(&root.join("content/pages/index.md"));
        let (html, _) = build(&page, &body, &Markdown::new(), &theme, &site);
        assert!(html.contains(&format!("<title>{}</title>", escape(&site.title()))));
        assert!(html.contains(&format!("<p class=\"tg vd f5\">{}", escape(&site.tagline))));
        assert!(html.contains(&format!("content=\"{}\"", escape(&site.description))));
    }

    #[test]
    fn ai_use_flag_maps_values_and_defaults() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let theme = root.join("themes/classic");
        let site: Site = serde_yaml::from_str(&template(&root.join("site.yaml"))).unwrap();
        let markdown = Markdown::new();
        let build_with = |front: &str| {
            let page: Page = serde_yaml::from_str(front).unwrap();
            build(&page, "", &markdown, &theme, &site).0
        };
        assert!(build_with("title: T\nslug: t\nlayout: post")
            .contains("AI Use: Draft fully by me, AI revised it."));
        assert!(build_with("title: T\nslug: t\nlayout: post\nai_use: no_ai")
            .contains("No AI use."));
        assert!(build_with("title: T\nslug: t\nlayout: post\nai_use: grammer")
            .contains("AI Use: Grammer"));
    }

    #[test]
    fn custom_block_uses_theme_and_keeps_inserted_text_literal() {
        let theme = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("themes/yellow");
        let markdown = Markdown::new();
        let input = ":::callout\n**Hello** {{title}}\n:::\n";
        let html = markdown.render(&custom_blocks(&markdown, input, &theme));
        assert!(
            html.contains("<aside class=\"callout\">") && html.contains("<strong>Hello</strong>")
        );
        let filled = fill(
            "<main>{{content}}</main><title>{{title}}</title>",
            &HashMap::from([("content".into(), html), ("title".into(), "Page".into())]),
        );
        assert!(filled.contains("{{title}}"));
        assert!(filled.contains("<title>Page</title>"));
    }
}
