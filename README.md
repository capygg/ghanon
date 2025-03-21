# ghanon (GitHub Anonymizer)

A CLI tool to anonymize your GitHub identity in Git repositories.

## Features

- Rewrite Git commit history to replace author name and email
- Uses git-filter-repo under the hood for efficient history rewriting
- Simple CLI interface
- Can process multiple repositories at once

## Installation

```
cargo install --path .
```

## Dependencies

This tool relies on `git-filter-repo`, which needs to be installed separately:

```
pip install git-filter-repo
```

## Usage

```
ghanon --old-name "Real Name" --old-email "real@email.com" --new-name "Alias" --new-email "alias@example.com" [--path /path/to/repository] [--recursive]
```

Options:

- `--old-name`: Your current name in Git commits
- `--old-email`: Your current email in Git commits
- `--new-name`: The anonymized name to use
- `--new-email`: The anonymized email to use
- `--path`: Directory containing repositories to process (defaults to current directory)
- `--recursive, -r`: Recursively process Git repositories in subdirectories

### Examples

Process the current repository:

```
ghanon --old-name "John Doe" --old-email "john.doe@example.com" --new-name "Anonymous" --new-email "anon@example.com"
```

Process a specific repository:

```
ghanon --old-name "John Doe" --old-email "john.doe@example.com" --new-name "Anonymous" --new-email "anon@example.com" --path /path/to/repo
```

Process all repositories in a directory:

```
ghanon --old-name "John Doe" --old-email "john.doe@example.com" --new-name "Anonymous" --new-email "anon@example.com" --path /path/to/projects --recursive
```

## Warning

This tool rewrites Git history, which is a destructive operation. Always make backups before using it.
After rewriting history, you will need to force push to any remote repositories.

## License

MIT
