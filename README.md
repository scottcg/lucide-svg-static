# lucide-static-svg

[![crates.io](https://img.shields.io/crates/v/lucide-static-svg.svg)](https://crates.io/crates/lucide-static-svg)
[![docs.rs](https://docs.rs/lucide-static-svg/badge.svg)](https://docs.rs/lucide-static-svg)
[![License](https://img.shields.io/badge/license-MIT%20AND%20ISC-blue.svg)](https://github.com/scottcg/lucide-svg-static/blob/main/LICENSE)

An offline Rust crate containing the Lucide icon catalog as generated, embedded SVG data and optional SVG path data.

The catalog is generated from official [Lucide](https://lucide.dev) release archives ([lucide-icons/lucide](https://github.com/lucide-icons/lucide)). Runtime use has no network dependency.

## Install

Add the dependency from [crates.io](https://crates.io/crates/lucide-static-svg):

```sh
cargo add lucide-static-svg
```

Or add it manually to `Cargo.toml`:

```toml
[dependencies]
lucide-static-svg = "1"
```

The `svg` feature is enabled by default. Enable `path-data` when raw path geometry is needed:

```toml
[dependencies]
lucide-static-svg = { version = "1", default-features = false, features = ["svg", "path-data"] }
```

Alternatively, you can depend on the repository directly via Git:

```toml
[dependencies]
lucide-static-svg = { git = "https://github.com/scottcg/lucide-svg-static" }
```

## Usage

```rust
use lucide_static_svg::Icon;

let icon = Icon::ChevronRight;

assert_eq!(icon.name(), "chevron-right");
assert_eq!(icon.asset_path(), "assets/lucide/chevron-right.svg");

#[cfg(feature = "svg")]
let svg = icon.svg_str();

#[cfg(feature = "path-data")]
let paths = icon.path_data();
```

`Icon` also provides `all()`, `from_asset_path()`, `view_box()`, and `Display` support. The crate exports the embedded Lucide version and source metadata as `LUCIDE_VERSION`, `LUCIDE_RELEASE_TAG`, `LUCIDE_ICON_COUNT`, and `LUCIDE_SOURCE_URL`.

## Features

| Feature | Default | Provides |
| --- | --- | --- |
| `svg` | Yes | Embedded SVG strings and byte slices |
| `path-data` | No | SVG path geometry |

## Development

The workspace contains the catalog crate and an `xtask` generator. To regenerate the catalog from a local Lucide release archive:

```sh
cargo run -p xtask -- generate \
  --archive /path/to/lucide-icons-<version>.zip \
  --output crates/lucide-static-svg/src/generated \
  --tag <version>
```

For an extracted icon directory, use `--input <directory>` instead of `--archive`.

Before submitting changes, run:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace --all-features
cargo clippy --workspace --all-features --all-targets -- -D warnings
```

## Release workflow

`.github/workflows/lucide-release.yml` runs daily and can also be started manually with an optional Lucide version. It:

1. Downloads and verifies official release archives from [lucide-icons/lucide](https://github.com/lucide-icons/lucide).
2. Regenerates the catalog when a release is needed.
3. Runs formatting, checks, tests, and Clippy.
4. Packages and publishes the crate to [crates.io](https://crates.io/crates/lucide-static-svg).
5. Commits generated changes and creates or refreshes a GitHub release containing the `.crate` artifact.

## License

The crate code and generator are licensed under **MIT**. The upstream [Lucide icons](https://github.com/lucide-icons/lucide) are licensed under **ISC**.
