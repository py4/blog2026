import fs from "fs/promises"
import minifyHtml from "@minify-html/node"
import { transform } from "lightningcss"
import { glob } from "glob"

const { minify } = minifyHtml

async function build() {
  // Find all HTML files
  const htmlFiles = await glob("src/**/*.html")

  // Create dist directory
  await fs.mkdir("dist", { recursive: true })

  // Process each HTML file
  for (const file of htmlFiles) {
    console.log(`Processing ${file}...`)

    let html = await fs.readFile(file, "utf8")

    // Extract and minify inline CSS
    html = html.replace(/<style>([\s\S]*?)<\/style>/g, (match, css) => {
      const result = transform({
        filename: "inline.css",
        code: Buffer.from(css),
        minify: true
      })
      return `<style>${result.code.toString()}</style>`
    })

    // Minify HTML
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
      remove_processing_instructions: true
    })

    // Write output
    const outFile = file.replace("src/", "dist/")
    await fs.writeFile(outFile, minified)

    console.log(`  → ${outFile} (${minified.length} bytes)`)
  }

  // Copy other files (robots.txt, sitemap.xml, _headers)
  const otherFiles = ["robots.txt", "sitemap.xml", "_headers"]
  for (const file of otherFiles) {
    try {
      await fs.copyFile(file, `dist/${file}`)
      console.log(`Copied ${file}`)
    } catch (err) {
      // File might not exist, skip
    }
  }

  console.log("\n✅ Build complete!")
}

build().catch(console.error)
