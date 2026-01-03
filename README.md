# /dev/blog

A minimal blog engine built with Rust that serves org-mode files. All content and static assets are embedded into a single binary.

## Tech Stack

- **Axum** - Web framework
- **Askama** - HTML templating
- **orgize** - Org-mode parser
- **syntect** - Syntax highlighting
- **rust-embed** - Static file embedding

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Development tools
cargo install cargo-watch
brew install just

# Testing tools
cargo install cargo-nextest
cargo install cargo-llvm-cov

# Deployment tools (macOS → Linux cross-compilation)
cargo install cargo-zigbuild
brew install zig
```

**Note:** Git is required for builds. The build script extracts last modified timestamps from git history for blog posts.

## Configuration

The blog engine can be configured via `site.toml` in the project root:

```toml
[site]
name = "/dev/blog"                                      # Blog name (appears in header and titles)
domain = "https://wall.ninja"                            # Domain for sitemap and RSS generation (optional)
redirect_domains = ["example.com", "www.example.com"]   # Domains to redirect to main domain (optional)
description = "Your blog description"                    # Meta description for SEO
default_theme = "dark"                                   # Default theme: "dark", "light", or "system" (optional, defaults to "system")

[server]
bind_address = "127.0.0.1:3000"       # Server bind address
```

**Environment Variable Overrides:**

Environment variables take precedence over `site.toml`:

- `SITE_NAME` - Override blog name
- `SITE_DOMAIN` - Override domain
- `SITE_DESCRIPTION` - Override meta description
- `BIND_ADDRESS` - Override server bind address

**Example:**
```bash
cp site.toml.example site.toml
# Edit site.toml with your settings
```

If `site.toml` is not found, the engine uses sensible defaults.

## Quick Start

**Run development server** (auto-reloads on file changes):
```bash
just run
```

## Testing

**Run tests:**
```bash
cargo nextest run
```

**Run tests with coverage:**
```bash
cargo llvm-cov nextest
```

**Generate HTML coverage report:**
```bash
cargo llvm-cov nextest --open
```

## Writing Posts

Create `.org` files in `content/posts/` with frontmatter:

```org
#+TITLE: Your Post Title
#+DATE: 2025-01-20
#+DESCRIPTION: A brief description for SEO (optional)

* Your content here
```

The `DESCRIPTION` field provides a unique meta description for search engines. If omitted, the default site description is used.

See `AGENTS.md` for formatting guidelines.

## Deployment

**Initial Server Setup:**

```bash
# Using defaults from site.toml
just setup <server-ip>

# Or specify custom values
just setup <server-ip> example.com /opt/myblog myblog-engine
```

This copies `server-setup.sh` to the server and runs it. The script installs Caddy (reverse proxy with automatic HTTPS) and configures the systemd services.

Parameters:
- `server-ip` - The IP address of your server (required)
- `domain` - Domain name for Caddy configuration (default: from site.toml or "wall.ninja")
- `install-dir` - Installation directory (default: "/opt/blog")
- `binary-name` - Name of the binary (default: "blog-engine")

**Deploy Updates:**

```bash
# Using defaults
just deploy <server-ip>

# Or specify custom paths
just deploy <server-ip> /opt/myblog myblog-engine
```

This cross-compiles for Linux, copies the binary to the install directory, and restarts the systemd service.

**Deploy and Purge Cache:**

```bash
just deploy-purge <server-ip>
```

Deploys updates and purges Cloudflare cache in one command. Requires `CLOUDFLARE_ZONE_ID` and `CLOUDFLARE_API_TOKEN` environment variables.

**Purge Cloudflare Cache Only:**

```bash
just purge-cache
```

Purges all cached content from Cloudflare. Requires `CLOUDFLARE_ZONE_ID` and `CLOUDFLARE_API_TOKEN` environment variables.

**Why cargo-zigbuild?**

Handles C dependencies (like `onig_sys` from syntect) without needing a Linux GCC toolchain on macOS.

## Design

**Theme:** ef-maris (dark/light) by [Protesilaos Stavrou](https://protesilaos.com/emacs/ef-themes)
- Marine-inspired color palette
- Configurable default theme (dark/light/system) via `default_theme` in site.toml
- Theme toggle button for manual switching
- User preference persisted in localStorage

**Typography:**
- Body: [Inter](https://fonts.google.com/specimen/Inter)
- Headings: [Farro](https://fonts.google.com/specimen/Farro)
- Code: [JetBrains Mono](https://www.jetbrains.com/lp/mono/)
- Logo: [Victor Mono](https://rubjo.github.io/victor-mono/) (retro terminal aesthetic)

**Features:**
- Rainbow-colored org-mode headings
- Syntax highlighting via syntect
- Search with context excerpts
- Automatic last updated timestamps from git history
- Wind rose compass footer
- SVG favicon
- Collapsible table of contents for long posts (mobile modal with backdrop on small screens)
- Custom 404 page
- RSS feed at /rss.xml

**SEO:**
- Per-post meta descriptions via `#+DESCRIPTION` frontmatter
- Automatic sitemap.xml generation (requires domain in site.toml)
- RSS feed with full content (/rss.xml)
- robots.txt with sitemap reference
- Long-term caching headers for static assets
- Domain redirect support for canonicalization

**Security:**
- **Rate limiting** - Per-IP limits (10 req/sec, burst 20) via tower_governor. Uses X-Forwarded-For headers to identify real client IPs behind Cloudflare/reverse proxies.
- **Search protection** - Query length limits (200 chars) and 5-second timeout
- **HTML sanitization** - Proper entity encoding/decoding with html-escape
