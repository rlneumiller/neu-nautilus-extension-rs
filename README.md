# nautilus4-extension-rs

Borrows heavily from (https://github.com/talklittle/nautilus-extension-rs), which only supported gtk3 at the time)

A Rust workspace providing safe Nautilus extension bindings for GTK4.

This repository contains two workspace members:

* `nautilus4-extension` — the safe Rust wrapper crate for `libnautilus-extension`
* `nautilus4-extension-sys` — the low-level FFI bindings to Nautilus and related GNOME C APIs

The workspace is currently wired for local development, and the safe wrapper crate depends on the `nautilus4-extension-sys` crate by local path.

---

## Repository layout

```text
neu-nautilus-extension-rs/
  Cargo.toml
  Cargo.lock
  CHANGELOG.md
  COPYING.txt
  README.md
  nautilus4-extension/
    Cargo.toml
    src/
  nautilus4-extension-sys/
    Cargo.toml
    README.md
    src/
```

---

## Requirements

* Nautilus 43+ or compatible Nautilus 4 runtime
* GTK4 development libraries
* Rust 1.96+
* Cargo

---

## Building

From the repository root:

```bash
cargo build --workspace
```

Build a single crate only:

```bash
cargo build -p nautilus4-extension
cargo build -p nautilus4-extension-sys
```

---

## Usage

This repository does not currently include a standalone example application.

The `nautilus4-extension` crate is intended to be consumed by a Nautilus extension project such as `neu-nautilus-media-columns-rs`.

---

## Release notes

See `CHANGELOG.md` for change history.

---

## License

[GNU General Public License version 3](COPYING.txt)
