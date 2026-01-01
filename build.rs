use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Tell Cargo to rerun if git history or posts change
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
    println!("cargo:rerun-if-changed=content/posts");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_metadata.rs");

    let mut metadata = Vec::new();
    let mut sitemap_entries = Vec::new();

    // Iterate over .org files in content/posts/
    let posts_dir = Path::new("content/posts");
    if let Ok(entries) = fs::read_dir(posts_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("org") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if let Some(date) = get_last_modified_date(&path) {
                        metadata.push(format!("    (\"{}\", \"{}\"),", filename, date));

                        // Generate slug from filename (remove .org extension)
                        let slug = filename.trim_end_matches(".org");
                        sitemap_entries.push((slug.to_string(), date));
                    }
                }
            }
        }
    }

    let code = format!(
        "pub static POST_UPDATED_DATES: &[(&str, &str)] = &[\n{}\n];",
        metadata.join("\n")
    );

    fs::write(&dest_path, code).unwrap();

    // Generate sitemap.xml and robots.txt
    generate_sitemap(&sitemap_entries);
    generate_robots_txt();
}

fn get_last_modified_date(path: &Path) -> Option<String> {
    // Try to get the last commit date from git
    let output = Command::new("git")
        .args(["log", "-1", "--format=%cI", "--"])
        .arg(path)
        .output()
        .ok()?;

    if !output.status.success() || output.stdout.is_empty() {
        return None;
    }

    let timestamp = String::from_utf8(output.stdout).ok()?;
    let timestamp = timestamp.trim();

    // Parse ISO 8601 and extract YYYY-MM-DD
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        Some(dt.format("%Y-%m-%d").to_string())
    } else {
        None
    }
}

fn generate_sitemap(posts: &[(String, String)]) {
    let domain = match env::var("SITE_DOMAIN") {
        Ok(d) => d,
        Err(_) => {
            println!("cargo:warning=SITE_DOMAIN not set, skipping sitemap.xml generation");
            return;
        }
    };

    let mut xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>{}/</loc>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
"#,
        domain
    );

    // Add all blog posts
    for (slug, lastmod) in posts {
        xml.push_str(&format!(
            r#"  <url>
    <loc>{}/post/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
"#,
            domain, slug, lastmod
        ));
    }

    xml.push_str("</urlset>\n");

    // Write sitemap.xml to static/ folder so it gets embedded
    let sitemap_path = Path::new("static/sitemap.xml");
    fs::write(sitemap_path, xml).expect("Failed to write sitemap.xml");
    println!("Generated sitemap.xml with {} posts for domain: {}", posts.len(), domain);
}

fn generate_robots_txt() {
    let domain = match env::var("SITE_DOMAIN") {
        Ok(d) => d,
        Err(_) => {
            println!("cargo:warning=SITE_DOMAIN not set, skipping robots.txt generation");
            return;
        }
    };

    let robots = format!(
        "User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n",
        domain
    );

    let robots_path = Path::new("static/robots.txt");
    fs::write(robots_path, robots).expect("Failed to write robots.txt");
    println!("Generated robots.txt for domain: {}", domain);
}
