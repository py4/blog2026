# Py4_ Blog

Ultra-optimized personal blog built with static HTML and aggressive minification.

## Build System

### Local Development

```bash
# Install dependencies
npm install

# Build (minify HTML/CSS)
npm run build

# Output goes to dist/
```

### File Structure

```
├── src/           # Source HTML files (human-readable)
├── dist/          # Built files (minified, deployed to Cloudflare)
├── build.mjs      # Build script
└── package.json   # Dependencies
```

### Cloudflare Pages Settings

- **Build command**: `npm ci && npm run build`
- **Output directory**: `dist`

### Optimization Stack

- **HTML**: `@minify-html/node` (Rust-based, aggressive)
- **CSS**: `lightningcss` (minifies inline styles)
- **Compression**: Cloudflare Brotli (automatic)

### File Sizes

After build + Brotli compression:

- `index.html`: ~3.5KB → ~1.5KB transferred
- `post.html`: ~10KB → ~3KB transferred
- `about.html`: ~6KB → ~2KB transferred

Total page weight: **<5KB** for most pages.

### Design

Yellow/black brutalist aesthetic with:
- Impact font for headings
- Verdana for body text
- System fonts only (no web fonts)
- Inline CSS (no external stylesheets)
- Blinking terminal cursor

## Development

Edit files in `src/`, then run `npm run build` to generate optimized output in `dist/`.

Cloudflare Pages will automatically deploy the `dist/` folder.
