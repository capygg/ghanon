use anyhow::{Context, Result};
use clap::Parser;
use env_logger::Env;
use log::{error, info};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::TempDir;

/// CLI tool to anonymize GitHub identity in Git repositories
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Original name to replace
    #[clap(long)]
    old_name: String,

    /// Original email to replace
    #[clap(long)]
    old_email: String,

    /// New name to use
    #[clap(long)]
    new_name: String,

    /// New email to use
    #[clap(long)]
    new_email: String,

    /// Path to the repository (defaults to current directory)
    #[clap(long)]
    path: Option<PathBuf>,

    /// Recursively search for Git repositories in subdirectories
    #[clap(long, short)]
    recursive: bool,
}

fn check_git_filter_repo() -> Result<()> {
    let output = Command::new("git")
        .args(&["filter-repo", "--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match output {
        Ok(status) if status.success() => Ok(()),
        _ => {
            error!("Error: git-filter-repo not found. Run: pip install git-filter-repo");
            anyhow::bail!("Missing dependency: git-filter-repo")
        }
    }
}

fn anonymize_repository(
    repo_path: &Path,
    old_name: &str,
    old_email: &str,
    new_name: &str,
    new_email: &str,
) -> Result<()> {
    let repo_path_str = repo_path.to_string_lossy();
    let git_dir = repo_path.join(".git");
    if !git_dir.exists() {
        anyhow::bail!("Not a git repository: {}", repo_path_str);
    }

    // Create a backup
    let backup_dir = TempDir::new()?;
    let backup_path = backup_dir.path();

    Command::new("git")
        .args(&[
            "clone",
            "--mirror",
            &repo_path_str,
            &backup_path.to_string_lossy(),
        ])
        .stdout(Stdio::null())
        .status()
        .context("Failed to create backup")?;

    // Create mailmap file
    let temp_dir = TempDir::new()?;
    let mailmap_path = temp_dir.path().join("mailmap");
    let mailmap_content = format!(
        "{} <{}> {} <{}>\n",
        new_name, new_email, old_name, old_email
    );
    fs::write(&mailmap_path, mailmap_content).context("Failed to create mailmap file")?;

    // Rewrite history
    info!("📝 Rewriting history for: {}", repo_path_str);
    let status = Command::new("git")
        .current_dir(repo_path)
        .args(&[
            "filter-repo",
            "--force",
            "--mailmap",
            &mailmap_path.to_string_lossy(),
            "--replace-refs",
            "update-or-add",
        ])
        .status()
        .context("Failed to run git filter-repo")?;

    if !status.success() {
        anyhow::bail!("git filter-repo failed");
    }

    info!("✅ History rewritten successfully");
    info!("⚠️  To update remote, run:");
    info!("   git push --force --all");
    info!("   git push --force --tags");

    Ok(())
}

fn anonymize_directory(
    dir_path: &Path,
    old_name: &str,
    old_email: &str,
    new_name: &str,
    new_email: &str,
    recursive: bool,
) -> Result<()> {
    let git_dir = dir_path.join(".git");
    if git_dir.exists() {
        anonymize_repository(dir_path, old_name, old_email, new_name, new_email)?;
        return Ok(());
    }

    if recursive {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir()
                && !path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .starts_with(".")
            {
                anonymize_directory(&path, old_name, old_email, new_name, new_email, recursive)?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let repo_path = args.path.unwrap_or_else(|| PathBuf::from("."));

    info!("🔄 Anonymizing Git history");
    info!(
        "   {} <{}> → {} <{}>",
        args.old_name, args.old_email, args.new_name, args.new_email
    );

    check_git_filter_repo()?;

    if repo_path.join(".git").exists() {
        anonymize_repository(
            &repo_path,
            &args.old_name,
            &args.old_email,
            &args.new_name,
            &args.new_email,
        )?;
    } else {
        if args.recursive {
            info!(
                "🔍 Searching repositories recursively in: {}",
                repo_path.display()
            );
        }

        anonymize_directory(
            &repo_path,
            &args.old_name,
            &args.old_email,
            &args.new_name,
            &args.new_email,
            args.recursive,
        )?;
    }

    Ok(())
}
