# Blog Formatting Guide

Posts are written in org-mode.

## Writing Style

- Keep sentences short and concise with simple words
- Use a friendly, casual tone
- Find and add hyperlinks for all subjects mentioned

## Review Checklist

When reviewing posts, check for:

**Grammar and Spelling:**
- Correct spelling throughout
- Proper punctuation and sentence structure
- Subject-verb agreement
- Consistent verb tenses

**Clarity and Ambiguity:**
- Clear pronoun references (avoid ambiguous "it", "this", "that")
- Specific subjects in sentences (prefer "Claude Code did X" over "it did X")
- No vague or unclear statements
- Smooth transitions between sections
- Introductions for new sections where needed

## Hyperlinks

Use org-mode format: =[[URL][link text]]=

**Always add hyperlinks for:**
- Technologies and programming languages: [[https://www.rust-lang.org/][Rust]], [[https://github.com/tokio-rs/axum][Axum]]
- Libraries and frameworks: [[https://github.com/PoiScript/orgize][orgize]], [[https://github.com/djc/askama][Askama]]
- Tools and services: [[https://caddyserver.com/][Caddy]], [[https://www.cloudflare.com/][Cloudflare]]
- Software and platforms: [[https://www.gnu.org/software/emacs/][Emacs]], [[https://www.proxmox.com/][Proxmox]]
- Fonts and typography: [[https://fonts.google.com/specimen/Farro][Farro]], [[https://www.jetbrains.com/lp/mono/][JetBrains Mono]]
- Web APIs and standards: [[https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage][localStorage]]
- People's projects: [[https://protesilaos.com/emacs/ef-themes][Protesilaos Stavrou's ef-themes]]

**Where to link:**
- Official websites for products and services
- GitHub repositories for open source projects
- Documentation sites (MDN for web APIs, docs.rs for Rust crates)
- Project homepages for fonts, themes, and tools

## org-mode Code Formatting

### Source Blocks

When creating shell script source blocks, use =sh= as the language specifier, not =shell=. The syntect syntax highlighter doesn't recognize =shell=.

#+begin_src sh
just run
#+end_src

### Wrap in =equals=

**Code elements:**
- File extensions: =.org=, =.rs=, =.toml=
- Frontmatter fields: =TITLE=, =DATE=
- HTML elements: =H1=, =H2=, =div=, =span=
- CSS/JavaScript APIs: =LocalStorage=, =querySelector=
- CSS selectors and media queries: =(prefers-color-scheme)=

**Variables and identifiers:**
- CSS variables: =--fg-primary=, =--bg-secondary=, =--color-link=
- Color variable names: =green-cooler=, =blue-warmer=, =cyan=
- Function/variable names: =fibonacci()=, =main()=

**Files, commands, and paths:**
- Filenames: =setup-blog.sh=, =justfile=, =README=, =Cargo.toml=
- Commands: =just deploy=, =cargo build=, =git commit=
- URL paths/endpoints: =/search?q=query=, =/api/users=
- Domain names: =wall.ninja=

**Measurements:**
- Technical measurements: =~200 chars=, =80x24=

### Do NOT wrap in =equals=

**Technology/Language names:**
- Programming languages: Rust, Python, JavaScript, TypeScript
- Frameworks/libraries: Axum, Askama, React, Vue
- File formats: SVG, PNG, JSON, YAML

**Protocols and standards:**
- Network protocols: HTTP, HTTPS, DNS, TCP, UDP
- Web standards: HTML, CSS, ARIA

**Tools and platforms:**
- Compression: Gzip, Brotli
- Operating systems: Mac, Linux, Windows
- Editors/modes: Emacs, org-mode, vim-mode

**Frameworks and services:**
- Theme names: ef-maris, dracula, solarized
- Service names: Cloudflare, GitHub, Docker

## Dependencies

When adding or updating dependencies:
- Always search for the latest stable version before using a dependency
- Check compatibility with existing dependencies (e.g., axum-test version must match axum version)
- Verify the dependency is actively maintained
- Use official documentation or crates.io to confirm version numbers

## Git Commits

When committing changes to a git repository:
- Use multiple commits if changes address different concerns or logical units
- Each commit should represent a single, coherent change
- Don't bundle unrelated changes into a single commit
- Commit messages should clearly describe what changed and why
