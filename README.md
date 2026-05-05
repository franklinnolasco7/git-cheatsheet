# git-cheat

Blazing fast terminal git cheatsheet. Keyboard-driven vim like controls. Easy peak of git commands with full description and examples.

## Install

**Binaries**

Download the latest standalone executable for Linux or macOS from the [Releases](https://github.com/franklinnolasco7/git-cheatsheet/releases) page.

**Cargo**

If you have Rust installed:
```bash
cargo install git-cheatsheet
```


## Keybindings

| Key | Action |
|-----|--------|
| `j` / `Down` | Move down |
| `k` / `Up` | Move up |
| `l` / `Right` | Next category |
| `h` / `Left` | Previous category |
| `g` | First item |
| `G` | Last item |
| `Tab` | Toggle focus |
| `/` | Search |
| `Esc` | Exit search |
| `y` / `Enter` | Copy command |
| `?` | Help |
| `q` / `Ctrl-C` | Quit |

## Clipboard

Auto-detects backend: `xclip` → `xsel` → `wl-copy` → `pbcopy`

If `y` yank doesn't work on Linux, install one of them using your package manager.

## Screenshots

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/d1d2dbd6-825b-48f2-9521-4c5301675efa" />
<br></br>
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/52c56b33-5fd5-449f-9be5-da774f152c08" />
<br></br>
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/275aecb6-5d14-4087-a7fb-c3136b7d3f12" />

## Development & Contributing

Want to add commands or build from source? Read [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT License — see [LICENSE](LICENSE) for details.
