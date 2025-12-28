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

## Routes

- `GET /` - List all posts (sorted by date, newest first)
- `GET /post/:slug` - View individual post
- `GET /static/*` - Serve static files

## Design & Credits

### Color Scheme
The blog uses the **ef-maris-dark** color theme by [Protesilaos Stavrou](https://protesilaos.com/).
- Theme repository: [ef-themes](https://github.com/protesilaos/ef-themes)
- License: GNU General Public License v3.0
- The ef-maris-dark theme features marine-inspired colors optimized for legibility in dark environments

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
