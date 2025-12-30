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

# Deployment tools (macOS → Linux cross-compilation)
cargo install cargo-zigbuild
brew install zig
```

**Note:** Git is required for builds. The build script extracts last modified timestamps from git history for blog posts.

## Quick Start

**Run development server** (auto-reloads on file changes):
```bash
just run
```

## Writing Posts

Create `.org` files in `content/posts/` with frontmatter:

```org
#+TITLE: Your Post Title
#+DATE: 2025-01-20

* Your content here
```

See `AGENTS.md` for formatting guidelines.

## Deployment

**Initial Server Setup:**

```bash
just setup <server-ip>
```

This copies `setup-blog.sh` to the server and runs it. The script installs Caddy (reverse proxy with automatic HTTPS) and configures the systemd services.

**Deploy Updates:**

```bash
just deploy <server-ip>
```

This cross-compiles for Linux, copies the binary to `/opt/blog/`, and restarts the systemd service.

**Why cargo-zigbuild?**

Handles C dependencies (like `onig_sys` from syntect) without needing a Linux GCC toolchain on macOS.

## Design

**Theme:** ef-maris (dark/light) by [Protesilaos Stavrou](https://protesilaos.com/emacs/ef-themes)
- Marine-inspired color palette
- Theme toggle respects system preference
- Preference persisted in localStorage

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

**Security:**
- **Rate limiting** - Per-IP limits (10 req/sec, burst 20) via tower_governor. Uses X-Forwarded-For headers to identify real client IPs behind Cloudflare/reverse proxies.
- **Search protection** - Query length limits (200 chars) and 5-second timeout
- **HTML sanitization** - Proper entity encoding/decoding with html-escape
