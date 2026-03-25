import fs from "fs/promises"
import minifyHtml from "@minify-html/node"
import { transform } from "lightningcss"
import { glob } from "glob"
import { createHighlighter } from "shiki"

const { minify } = minifyHtml

async function build() {
  // Initialize Shiki highlighter
  const highlighter = await createHighlighter({
    themes: ["github-light"],
    langs: ["python"]
  })

  // Find all HTML files
  const htmlFiles = await glob("src/**/*.html")

  // Create dist directory
  await fs.mkdir("dist", { recursive: true })

  // Process each HTML file
  for (const file of htmlFiles) {
    console.log(`Processing ${file}...`)

    let html = await fs.readFile(file, "utf8")

    // Highlight code blocks: <pre><code class="language-X">...</code></pre>
    html = html.replace(
      /<pre([^>]*)><code class="language-(\w+)">([\s\S]*?)<\/code><\/pre>/g,
      (match, preAttrs, lang, code) => {
        // Decode HTML entities before highlighting
        const decoded = code
          .replace(/&lt;/g, "<").replace(/&gt;/g, ">")
          .replace(/&amp;/g, "&").replace(/&quot;/g, '"')
          .replace(/&#39;/g, "'")
        const highlighted = highlighter.codeToHtml(decoded.trim(), {
          lang,
          theme: "github-light"
        })
        // Shiki outputs <pre ...><code>...</code></pre>, extract inner and keep our pre attrs
        const inner = highlighted.match(/<pre[^>]*><code>([\s\S]*?)<\/code><\/pre>/)?.[1] || decoded
        return `<pre${preAttrs}><code>${inner}</code></pre>`
      }
    )

    // Extract and minify inline CSS with maximum compression
    html = html.replace(/<style>([\s\S]*?)<\/style>/g, (match, css) => {
      const result = transform({
        filename: "inline.css",
        code: Buffer.from(css),
        minify: true,
        targets: {
          // Support only modern browsers for smaller output
          chrome: 90,
          firefox: 88,
          safari: 14
        }
      })
      return `<style>${result.code.toString()}</style>`
    })

    // Minify HTML with MAXIMUM aggression
    const minified = minify(Buffer.from(html), {
      do_not_minify_doctype: false,
      ensure_spec_compliant_unquoted_attribute_values: false,
      keep_closing_tags: false,
      keep_html_and_head_opening_tags: false,
      keep_spaces_between_attributes: false,
      keep_comments: false,
      minify_css: true,
      minify_js: true,
      remove_bangs: true,
      remove_processing_instructions: true,
      remove_spaces_between_attr_and_value: true
    })

    // Write output
    const outFile = file.replace("src/", "dist/")
    await fs.writeFile(outFile, minified)

    console.log(`  → ${outFile} (${minified.length} bytes)`)
  }

  // Copy other files (robots.txt, sitemap.xml, _headers, feed.xml)
  const otherFiles = ["robots.txt", "sitemap.xml", "_headers", "feed.xml"]
  for (const file of otherFiles) {
    try {
      await fs.copyFile(file, `dist/${file}`)
      console.log(`Copied ${file}`)
    } catch (err) {
      // File might not exist, skip
    }
  }

  // Copy image files from src/
  const imageFiles = await glob("src/**/*.{png,jpg,jpeg,gif,svg,webp,ico,txt,pdf}")
  for (const file of imageFiles) {
    const outFile = file.replace("src/", "dist/")
    // Create directory if it doesn't exist
    const outDir = outFile.substring(0, outFile.lastIndexOf("/"))
    await fs.mkdir(outDir, { recursive: true })
    await fs.copyFile(file, outFile)
    console.log(`Copied ${file} → ${outFile}`)
  }

  console.log("\n✅ Build complete!")
}

build().catch(console.error)
