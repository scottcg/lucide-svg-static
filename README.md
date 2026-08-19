# lucide-static-svg

An offline Rust crate containing the Lucide icon catalog as generated, embedded SVG data and optional SVG path data.

The catalog is generated from official [Lucide](https://lucide.dev) release archives. Runtime use has no network dependency.

## Install

```toml
[dependencies]
lucide-static-svg = "1.33"
```

The `svg` feature is enabled by default. Enable `path-data` when raw path geometry is needed:

```toml
lucide-static-svg = { version = "1.33", default-features = false, features = ["svg", "path-data"] }
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

`.github/workflows/lucide-release.yml` runs weekly and can also be started manually with an optional Lucide version. It:

1. Downloads and verifies the official Lucide release archive.
2. Regenerates the catalog when a release is needed.
3. Runs formatting, checks, tests, and Clippy.
4. Packages the crate, commits generated changes, and creates or refreshes a GitHub release containing the `.crate` artifact.

The workflow does not publish to crates.io.

## License

The crate code and generator are licensed under **MIT**. The upstream Lucide icons are licensed under **ISC**.
