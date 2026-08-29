# Dustoff

Dustoff is a safe, interactive Linux cleanup tool built for developers. It scans
common caches, lets you choose what to clean, shows a final review, and only
then performs the selected actions.

## Current cleanup targets

- npm download cache
- npx execution cache
- pnpm's unreferenced packages (`pnpm store prune`)
- Yarn cache
- Cargo registry and Git caches
- Docker stopped containers, unused images, unused networks, unused volumes,
  and build cache (each one is selected separately)
- User trash
- User thumbnail cache

Nothing is selected by default. Dustoff does not run as a background service and
does not need root privileges for the current targets.

## Install

Install the latest release for the current Linux architecture:

```bash
curl -fsSL https://raw.githubusercontent.com/CassianoThurow/dustoff/main/install.sh | bash
```

The installer downloads the release, verifies its SHA-256 checksum, and places
the binary in `~/.local/bin/dustoff`. It does not require `sudo`.

Then open the interactive flow from any terminal:

```bash
dustoff
```

Install a specific version:

```bash
curl -fsSL https://raw.githubusercontent.com/CassianoThurow/dustoff/main/install.sh | DUSTOFF_VERSION=v0.1.0 bash
```

Use a different installation directory:

```bash
curl -fsSL https://raw.githubusercontent.com/CassianoThurow/dustoff/main/install.sh | DUSTOFF_INSTALL_DIR="$HOME/bin" bash
```

## Build from source

```bash
cargo run --release
```

Analyze without opening the selection flow:

```bash
cargo run --release -- analyze
```

Install the global command from source:

```bash
cargo install --path .
dustoff
```

## Uninstall

```bash
rm ~/.local/bin/dustoff
```

If `DUSTOFF_INSTALL_DIR` was used during installation, remove the binary from
that directory instead.

## Safety model

- Analysis never deletes files.
- Destructive actions require explicit selection and final confirmation.
- Every filesystem target must be inside the current user's home directory.
- Symlinks are not followed while calculating directory sizes.
- Docker resources are cleaned by Docker itself, not by deleting internal files.
- Volumes and images are separate, opt-in actions and are marked as higher risk.

## Roadmap

- Rich terminal UI with live scan progress
- Distro-aware package cache cleanup
- Configurable exclusions and retention periods
- Machine-readable reports
- Automated tests and release packages
