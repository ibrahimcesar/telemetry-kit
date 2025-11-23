# Documentation Setup Guide

This guide explains how to run and develop the telemetry-kit documentation site built with [Fumadocs](https://fumadocs.dev/).

## Overview

The documentation site is a modern Next.js application using:

- **Fumadocs** - Documentation framework with great DX
- **Next.js 14** - React framework with App Router
- **MDX** - Markdown with React components
- **Tailwind CSS** - Styling
- **TypeScript** - Type safety

## Directory Structure

```
docs/
├── app/                      # Next.js App Router
│   ├── docs/                # Documentation pages
│   │   ├── [[...slug]]/     # Dynamic routes for all docs
│   │   └── layout.tsx       # Docs layout with sidebar
│   ├── layout.tsx           # Root layout
│   ├── layout.config.tsx    # Navigation configuration
│   ├── page.tsx             # Homepage
│   ├── source.ts            # MDX source configuration
│   └── global.css           # Global styles
│
├── content/docs/            # Documentation content (MDX)
│   ├── index.mdx           # Getting Started
│   ├── auto-sync.mdx       # Auto-Sync guide
│   ├── cli.mdx             # CLI documentation
│   ├── api.mdx             # API reference
│   ├── self-hosting.mdx    # Self-hosting guide
│   ├── examples.mdx        # Code examples
│   └── meta.json           # Navigation structure
│
├── components/              # React components
│   └── ui/
│       └── icon.tsx        # Icon component
│
├── public/                  # Static assets
│
├── package.json            # Dependencies
├── tsconfig.json           # TypeScript config
├── tailwind.config.ts      # Tailwind config
├── postcss.config.mjs      # PostCSS config
└── next.config.mjs         # Next.js config
```

## Quick Start

### Prerequisites

- Node.js 18+ (or 20+ recommended)
- npm, pnpm, or yarn

### Installation

1. Navigate to the docs directory:

```bash
cd docs
```

2. Install dependencies:

<details>
<summary>Using npm</summary>

```bash
npm install
```
</details>

<details>
<summary>Using pnpm (recommended)</summary>

```bash
pnpm install
```
</details>

<details>
<summary>Using yarn</summary>

```bash
yarn install
```
</details>

### Development

Start the development server:

```bash
npm run dev
# or
pnpm dev
# or
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

The site will auto-reload when you make changes to:
- MDX content files
- React components
- Configuration files

### Building

Build for production:

```bash
npm run build
```

Start the production server:

```bash
npm run start
```

### Static Export

For static hosting (GitHub Pages, Netlify, etc.):

```bash
npm run build
```

The static site will be in the `out/` directory.

## Adding Documentation

### Creating a New Page

1. Create a new `.mdx` file in `content/docs/`:

```mdx
---
title: My New Page
description: A helpful description for SEO
icon: Rocket
---

## Introduction

Your content here...
```

2. Add it to `content/docs/meta.json`:

```json
{
  "title": "Documentation",
  "pages": [
    "index",
    "auto-sync",
    "cli",
    "my-new-page",  // Add here
    "---API Reference---",
    "api"
  ]
}
```

### Frontmatter Options

```mdx
---
title: Page Title              # Required
description: Page description  # Required (for SEO)
icon: IconName                 # Optional (Lucide icon name)
full: true                     # Optional (full-width content)
---
```

### Using Components

Fumadocs provides helpful components:

#### Callouts

```mdx
import { Callout } from 'fumadocs-ui/components/callout';

<Callout type="info">
This is an info callout.
</Callout>

<Callout type="warn">
This is a warning.
</Callout>

<Callout type="error">
This is an error message.
</Callout>
```

#### Tabs

```mdx
import { Tab, Tabs } from 'fumadocs-ui/components/tabs';

<Tabs items={['npm', 'pnpm', 'yarn']}>
  <Tab value="npm">
    ```bash
    npm install telemetry-kit
    ```
  </Tab>
  <Tab value="pnpm">
    ```bash
    pnpm add telemetry-kit
    ```
  </Tab>
  <Tab value="yarn">
    ```bash
    yarn add telemetry-kit
    ```
  </Tab>
</Tabs>
```

#### Steps

```mdx
import { Step, Steps } from 'fumadocs-ui/components/steps';

<Steps>
<Step>

### First Step

Content for step 1

</Step>

<Step>

### Second Step

Content for step 2

</Step>
</Steps>
```

### Code Blocks

Code blocks support syntax highlighting:

````mdx
```rust
use telemetry_kit::prelude::*;

let telemetry = TelemetryKit::builder()
    .service_name("my-app")?
    .build()?;
```
````

With filename:

````mdx
```rust title="main.rs"
fn main() {
    println!("Hello!");
}
```
````

With line highlighting:

````mdx
```rust {1,3-5}
use telemetry_kit::prelude::*;

let telemetry = TelemetryKit::builder()
    .service_name("my-app")?
    .build()?;
```
````

## Configuration

### Navigation

Edit `app/layout.config.tsx` to change top navigation:

```tsx
export const baseOptions: BaseLayoutProps = {
  nav: {
    title: '🔭 telemetry-kit',
  },
  links: [
    {
      text: 'Documentation',
      url: '/docs',
      active: 'nested-url',
    },
    // Add more links
  ],
  githubUrl: 'https://github.com/ibrahimcesar/telemetry-kit',
};
```

### Sidebar

Edit `content/docs/meta.json` to change sidebar structure:

```json
{
  "title": "Documentation",
  "pages": [
    "index",
    "auto-sync",
    "---Section Separator---",
    "api"
  ]
}
```

### Theme

Fumadocs includes built-in dark mode. Customize colors in `tailwind.config.ts`.

## Deployment

### Vercel (Recommended)

1. Push to GitHub
2. Import project in Vercel
3. Vercel auto-detects Next.js and deploys

### Netlify

1. Build command: `npm run build`
2. Publish directory: `out`
3. Set Node version: `20`

### GitHub Pages

1. Build: `npm run build`
2. Deploy the `out/` directory
3. Or use GitHub Actions:

```yaml
name: Deploy Docs

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Install dependencies
        run: cd docs && npm ci

      - name: Build
        run: cd docs && npm run build

      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs/out
```

### Custom Server

1. Build: `npm run build`
2. Serve `out/` directory with any static host:

```bash
# Using a simple HTTP server
npx serve out

# Or copy to your server
rsync -avz out/ user@server:/var/www/docs/
```

## Troubleshooting

### "Module not found" errors

```bash
# Clear cache and reinstall
rm -rf node_modules .next
npm install
```

### TypeScript errors

```bash
# Regenerate types
npm run build
```

### MDX parsing errors

- Check frontmatter syntax
- Ensure proper component imports
- Verify code block formatting

### Build fails

1. Check Node.js version (18+)
2. Clear `.next` directory
3. Check for syntax errors in MDX files
4. Verify all imports are correct

## Development Tips

### Hot Reload

Development server watches:
- MDX files in `content/`
- Components in `app/` and `components/`
- Configuration files

### Preview Production Build

```bash
npm run build
npm run start
```

### Search

Fumadocs includes built-in search. It automatically indexes:
- Page titles
- Headings
- Content

### Icons

Available icons from [Lucide](https://lucide.dev/icons/):

```mdx
---
icon: Rocket
---
```

Common icons:
- `Rocket` - Getting started
- `Code` - API docs
- `Terminal` - CLI
- `Server` - Self-hosting
- `RefreshCw` - Auto-sync

## Contributing

When adding documentation:

1. Follow the existing structure
2. Use proper MDX formatting
3. Include code examples
4. Test locally before committing
5. Update `meta.json` for navigation
6. Check for broken links
7. Verify mobile responsiveness

## Resources

- [Fumadocs Documentation](https://fumadocs.dev/)
- [Next.js Documentation](https://nextjs.org/docs)
- [MDX Documentation](https://mdxjs.com/)
- [Tailwind CSS](https://tailwindcss.com/)

## License

MIT OR Apache-2.0
