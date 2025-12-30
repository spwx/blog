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

    // Iterate over .org files in content/posts/
    let posts_dir = Path::new("content/posts");
    if let Ok(entries) = fs::read_dir(posts_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("org") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if let Some(date) = get_last_modified_date(&path) {
                        metadata.push(format!("    (\"{}\", \"{}\"),", filename, date));
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
