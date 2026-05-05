# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-05

### Added
- `Getting Help` category (`git help`, `git <cmd> -h`)
- `git stash branch` — create branch directly from stash
- `git log --follow` — track file history through renames
- `git config --global credential.helper` — credential caching
- `git clone --depth 1` — shallow clone support
- Total commands: 115 → 123 across 13 categories

### Changed
- Replaced GitHub-specific example URLs with generic `example.com` domains
- Minor note formatting
- Bumped Rust edition from `2021` to `2024`

---

## [0.1.0] - Initial Release

### Added
- 115 commands across 12 categories
- Vim-style navigation (j/k/h/l)
- Fuzzy search with `/`
- Yank any command to clipboard with `y`
- Syntax-highlighted commands
- Detail pane with notes and examples
- Danger warnings for destructive commands
