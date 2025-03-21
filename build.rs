use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Check if git-filter-repo is installed
    let output = Command::new("git")
        .args(&["filter-repo", "--version"])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let version = String::from_utf8_lossy(&o.stdout);
            println!("cargo:warning=Found git-filter-repo: {}", version.trim());
        }
        _ => {
            println!("cargo:warning=git-filter-repo not found. Please install it with:");
            println!("cargo:warning=pip install git-filter-repo");
        }
    }
}
