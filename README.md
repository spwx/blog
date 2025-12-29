# Rust Blog Engine

A minimal blog engine built with Rust that serves org-mode files.

## Tech Stack

- **Axum** - Web framework
- **Askama** - HTML templating
- **orgize** - Org-mode parser
- **syntect** - Syntax highlighting
- **rust-embed** - Embed static files in binary

## Project Structure

```
.
├── content/posts/      # Org-mode blog posts
├── templates/          # Askama templates
├── static/            # Static files (CSS, images, etc.)
└── src/
    └── main.rs        # Main application
```

## Writing Posts

Create `.org` files in `content/posts/` with frontmatter:

```org
#+TITLE: Your Post Title
#+DATE: 2025-01-20

* Your content here

Write your post using org-mode syntax...
```

## Running

```bash
cargo run
```

Visit http://127.0.0.1:3000

## Building

The blog compiles into a single binary with all content and static files embedded:

```bash
cargo build --release
```

## Deployment

### Prerequisites

For cross-compiling from macOS to Linux x86_64:

```bash
# Install cargo-zigbuild for cross-compilation
cargo install cargo-zigbuild

# Install zig (provides cross-compilation toolchain)
brew install zig
```

### Deploy to Server

The project includes a justfile for easy deployment:

```bash
# Build, copy to server, and restart service
just deploy <server-ip>
```

This command:
1. Cross-compiles the binary for Linux using `cargo zigbuild`
2. Copies the binary to `/opt/blog/` on the server via SCP
3. Restarts the `blog.service` systemd service

### Server Setup

For initial server setup (LXC container or VM), run `setup-blog.sh` on the server:

```bash
# On the server
bash setup-blog.sh
```

This script:
- Installs Caddy (web server with automatic HTTPS)
- Configures systemd service for the blog
- Sets up Caddy as reverse proxy to localhost:3000
- Enables automatic restarts on failure

**Caddy Configuration** (`/etc/caddy/Caddyfile`):
```
wall.ninja {
    reverse_proxy localhost:3000
    encode gzip
}

www.wall.ninja {
    redir https://wall.ninja{uri} permanent
}
```

**Systemd Service** (`/etc/systemd/system/blog.service`):
```ini
[Unit]
Description=Blog Engine
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/blog
ExecStart=/opt/blog/blog-engine
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

### Cross-Compilation Details

The project uses **cargo-zigbuild** instead of standard `cargo build` for cross-compilation because:
- Handles C dependencies (like `onig_sys` from syntect) without needing Linux GCC toolchain on macOS
- Zig provides a robust, self-contained cross-compilation environment
- No need to install platform-specific toolchains

## Routes

- `GET /` - List all posts (sorted by date, newest first)
- `GET /search?q=query` - Search posts by title and content
- `GET /post/:slug` - View individual post
- `GET /static/*` - Serve static files

## Design & Credits

### Color Scheme
The blog supports both **ef-maris-dark** and **ef-maris-light** themes by [Protesilaos Stavrou](https://protesilaos.com/).
- Theme repository: [ef-themes](https://github.com/protesilaos/ef-themes)
- License: GNU General Public License v3.0
- Marine-inspired color palettes optimized for legibility in both dark and light environments
- Theme toggle button in header (top-right)
- Defaults to system preference (`prefers-color-scheme`)
- User preference stored in localStorage
- Smooth transitions between themes

### Typography
- **Headings**: [Farro](https://fonts.google.com/specimen/Farro) - SIL Open Font License
- **Code**: [JetBrains Mono](https://www.jetbrains.com/lp/mono/) - SIL Open Font License
- **Body**: System font stack

### Org-mode Heading Colors
The blog implements the rainbow heading color scheme from org-mode using ef-maris-dark palette mappings:
- H1: green-cooler (`#30c489`)
- H2: blue-warmer (`#70a0ff`)
- H3: green-warmer (`#7fce5f`)
- H4: cyan (`#2fd0db`)
- H5: magenta-cooler (`#cf90ff`)
- H6: blue-cooler (`#12b4ff`)

### Acknowledgments
- Theme design inspiration: [Protesilaos Stavrou's ef-themes](https://github.com/protesilaos/ef-themes)
- Font resources: [Google Fonts](https://fonts.google.com/)
- Org-mode syntax: [Org mode for Emacs](https://orgmode.org/)
