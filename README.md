# Lucide Static SVG (`lucide-static-svg`)

An automated, versioned Rust crate providing official [Lucide](https://lucide.dev) icons as static SVG vector data and metadata, complete with modular Cargo features (`svg`, `path-data`, `sprite`, `font`) and an optional GPUI rendering adapter (`lucide-gpui`).

Unlike icon-font crates that bundle `.ttf`/`.woff` font glyphs, `lucide-static-svg` delivers real vector path geometry, allowing UI frameworks like [GPUI](https://gpui.rs) to perform smooth frame-driven 2D transformations (`Transformation::rotate(...)`) for disclosure triggers, micro-animations, and dynamic motion.

---

## Key Features & Value

- ⚡ **100% Offline & Deterministic**: Pre-generated Rust code compiled into your binary. Zero build-time network calls or unpkg CDN dependencies.
- 🎨 **True 2D Vector Geometry**: Enables smooth frame-driven rotation ($0 \to 90^\circ$ chevron rolls, $180^\circ$ dropdown flips), scaling, and stroke styling.
- 📦 **Modular Cargo Feature Flags**: Enable only what you use to keep compiled binary sizes minimal.
- 🚀 **Built-in GPUI Adapter (`lucide-gpui`)**: Plug-and-play GPUI `AssetSource` integration and fixed-bounds vector rotation helpers.
- 🔄 **Automated Upstream Sync**: `xtask` generator continuously synchronized with official Lucide releases.

---

## Cargo Feature Flags

To prevent binary bloat, asset formats are feature-gated:

```toml
[dependencies]
lucide-static-svg = { version = "0.1", features = ["svg"] }
```

| Feature | Description | Output Types |
| :--- | :--- | :--- |
| **`svg`** *(default)* | Embedded raw `<svg ...>` strings & byte slices | `&'static str` / `&'static [u8]` |
| **`path-data`** | Vector path geometry strings (`d="..."`) | `&'static str` |
| **`sprite`** | Single combined SVG `<symbol>` sprite sheet | `&'static str` |
| **`font`** | Bundles font file bytes (`.ttf`/`.woff2`) + Unicode glyph mappings | `char` / `&'static [u8]` |

---

## Usage

### 1. Basic Usage (Core Crate)

```rust
use lucide_static_svg::Icon;

fn main() {
    let icon = Icon::ChevronRight;

    println!("Icon Name: {}", icon.name());
    println!("ViewBox: {}", icon.view_box());

    #[cfg(feature = "svg")]
    println!("SVG Data: {}", icon.svg_str());

    #[cfg(feature = "path-data")]
    println!("Path Geometry: {}", icon.path_data());
}
```

### 2. GPUI Usage (`lucide-gpui`)

```rust
use gpui::{svg, px, radians, Transformation, Element, Hsla};
use lucide_static_svg::Icon;

fn render_rotating_chevron(progress: f32, color: Hsla, size: f32) -> impl Element {
    let angle_rad = progress.clamp(0.0, 1.0) * std::f32::consts::FRAC_PI_2;

    svg()
        .path(Icon::ChevronRight.asset_path())
        .size(px(size))
        .text_color(color)
        .with_transformation(Transformation::rotate(radians(angle_rad)))
}
```

---

## Repository Architecture

```
lucide-static-svg/             # Repository Root
├── Cargo.toml                 # Workspace manifest
├── crates/
│   ├── lucide-static-svg/     # Core zero-dependency SVG asset crate
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── generated/     # Output produced by xtask
│   │   │       ├── mod.rs
│   │   │       ├── icons.rs
│   │   │       ├── paths.rs
│   │   │       └── unicode.rs
│   │   └── Cargo.toml
│   └── lucide-gpui/           # GPUI adapter & asset integration
│       ├── src/
│       │   ├── lib.rs
│       │   ├── asset_source.rs
│       │   └── animated_icon.rs
│       └── Cargo.toml
└── xtask/                     # Generator CLI for upstream releases
    ├── src/
    │   ├── main.rs
    │   ├── fetch.rs
    │   ├── parse.rs
    │   └── generate.rs
    └── Cargo.toml
```

---

## Technical Implementation Specification

### 1. `xtask` Generator Engine

The `xtask` crate is executed via `cargo xtask generate --tag <lucide-version-tag>` or during CI.

#### Upstream Input Source
- Downloads `lucide-static` npm package release tarball (`https://registry.npmjs.org/lucide-static/-/lucide-static-<version>.tgz`) or GitHub release ZIP.
- Unpacks static `.svg` vector files from `package/icons/*.svg` and `package/font/lucide.json` (for unicode glyph mappings).

#### SVG Parsing & Name Normalization (`parse.rs`)
- Uses `roxmltree` to parse SVG attributes (`viewBox="0 0 24 24"`) and extract all inner path element `d="..."` values.
- Converts file basename to Rust `PascalCase` enum variant names:
  - `chevron-right.svg` $\to$ `ChevronRight`
  - `file-text.svg` $\to$ `FileText`
  - `24-hours.svg` $\to$ `Hours24` (prefixes leading numbers with words or converts correctly to valid Rust identifiers).

#### Code Generation Output (`generate.rs`)
Generates static, formatted Rust files in `crates/lucide-static-svg/src/generated/`:
1. `icons.rs`:
   - Enums: `pub enum Icon { ChevronRight, ... }`
   - Implementations for `name()`, `asset_path()`, `view_box()`, `svg_str()`, `svg_bytes()`.
2. `paths.rs`:
   - Const lookup table mapping each `Icon` variant to its raw vector `d` path strings.
3. `unicode.rs`:
   - Const lookup table mapping each `Icon` variant to its font unicode glyph (`'\u{e900}'`).

### 2. `lucide-gpui` Adapter Implementation

#### `AssetSource` Implementation (`asset_source.rs`)
```rust
use gpui::{AssetSource, SharedString};
use anyhow::Result;
use lucide_static_svg::Icon;

pub struct LucideAssetSource;

impl AssetSource for LucideAssetSource {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        let clean_path = path.trim_start_matches('/');
        if let Some(icon) = Icon::from_asset_path(clean_path) {
            Ok(Some(std::borrow::Cow::Borrowed(icon.svg_bytes())))
        } else {
            Ok(None)
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        if path == "assets/lucide" {
            Ok(Icon::all().iter().map(|i| SharedString::from(i.asset_path())).collect())
        } else {
            Ok(Vec::new())
        }
    }
}
```

#### Animated Icon Primitive (`animated_icon.rs`)
```rust
use gpui::{svg, px, radians, Transformation, AnyElement, Hsla, Element};
use lucide_static_svg::Icon;

pub fn render_animated_icon(
    icon: Icon,
    progress: f32,
    angle_start_rad: f32,
    angle_end_rad: f32,
    color: Hsla,
    size: f32,
) -> AnyElement {
    let current_angle = angle_start_rad + (angle_end_rad - angle_start_rad) * progress.clamp(0.0, 1.0);

    svg()
        .path(icon.asset_path())
        .size(px(size))
        .text_color(color)
        .with_transformation(Transformation::rotate(radians(current_angle)))
        .into_any_element()
}
```

### 3. CI/CD GitHub Actions Workflows

#### `.github/workflows/sync.yml` (Upstream Sync Check)
- **Schedule**: `cron: '0 0 * * 0'` (weekly).
- **Steps**:
  1. Checks latest Lucide release on npm/GitHub.
  2. If version > recorded version, runs `cargo xtask generate --tag <version>`.
  3. Runs `cargo test`.
  4. Opens PR: `chore(deps): update lucide-static to v<version>`.

#### `.github/workflows/publish.yml` (Crates.io Trusted Publishing)
- **Trigger**: `push: tags: ['v*']`
- **Permissions**: `id-token: write`
- **Steps**:
  1. `cargo test --all-features`
  2. `cargo publish -p lucide-static-svg`
  3. `cargo publish -p lucide-gpui`

---

## Comparison with Other Lucide Rust Crates

| Dimension | `lucide-icons-rs` / `lucide-icons` | `KrisNK/lucide-svg` | `lucide-static-svg` (This Crate) |
| :--- | :--- | :--- | :--- |
| **Asset Format** | Icon font files (`.ttf` / Unicode glyphs) | SVG strings | Vector SVGs, path data, sprite sheets, font fallbacks |
| **Build Dependency** | Bundled font file | ❌ **Live network download from CDN during `cargo build`** | ✅ **100% Offline & Deterministic** |
| **API Shape** | Enum + Unicode char | Separate `struct` per icon (requires `Box<dyn Trait>`) | Ergonomic `Icon` Enum (`Copy`/`Clone`/`Display`) |
| **2D Vector Motion** | ❌ Cannot rotate font glyphs in GPUI | ⚠️ Limited | ✅ Native `Transformation::rotate` support |
| **GUI Adapters** | Iced font widgets | None | Built-in GPUI adapter (`lucide-gpui`) |

---

## License

Dual-licensed under **MIT OR ISC**:
- Upstream Lucide icons are copyright (c) 2026 Lucide Icons and Contributors, distributed under the **ISC License**.
- Rust crate code, generator tooling, and integration wrappers are distributed under the **MIT License**.
