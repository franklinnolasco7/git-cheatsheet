# Contributing

One thing per PR. Keep it focused.

## Build Requirements

- Rust 1.70+ (`rustup` recommended)
- To test clipboard yank on Linux: `xclip`, `xsel`, or `wl-copy`

## Running Locally

Clone the repo and run:

```bash
cargo run
```

To build a release binary:

```bash
cargo build --release
# Binary will be at target/release/git-cheat
```

## Adding a command

Everything is in `src/data.rs`. Each command looks like this:

```rust
Command {
    cmd: "git example --flag <arg>",
    description: "What it does, one line.",
    note: Some("# Extra context, warnings, file paths, etc."),
    example: Some("$ git example --flag value"),
    dangerous: false,
},
```

Set `dangerous: true` for anything destructive (force push, rebase, reset --hard, etc.), it shows a warning in the UI.

Add to an existing category or create a new one using the same pattern.

## Before opening a PR

```bash
cargo fmt
cargo clippy -- -D warnings
cargo build --release
```

Don't add dependencies unless there's a real reason. The binary should stay small.

## PR tips

- Say what changed and why
- If it's a bug fix, describe how to reproduce it
