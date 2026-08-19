use std::{fs, path::Path, process::Command};

use crate::parse::IconSource;

pub fn write_generated(
    output: &Path,
    tag: Option<&str>,
    icons: &[IconSource],
) -> Result<(), String> {
    fs::create_dir_all(output)
        .map_err(|error| format!("could not create {}: {error}", output.display()))?;
    write(output, "mod.rs", module_source(tag))?;
    write(output, "icons.rs", icons_source(icons))?;
    write(output, "paths.rs", paths_source(icons))?;
    write(output, "unicode.rs", unicode_source())?;
    write(output, "version.rs", version_source(tag, icons.len()))?;
    format_generated(output)
}

fn write(output: &Path, name: &str, contents: String) -> Result<(), String> {
    fs::write(output.join(name), contents)
        .map_err(|error| format!("could not write {}/{}: {error}", output.display(), name))
}

fn format_generated(output: &Path) -> Result<(), String> {
    for name in ["mod.rs", "icons.rs", "paths.rs", "unicode.rs", "version.rs"] {
        let status = Command::new("rustfmt")
            .arg(output.join(name))
            .status()
            .map_err(|error| format!("could not run rustfmt for {name}: {error}"))?;
        if !status.success() {
            return Err(format!("rustfmt failed for {name}"));
        }
    }
    Ok(())
}

fn module_source(tag: Option<&str>) -> String {
    let version = tag.unwrap_or("local");
    format!(
        "//! Generated Lucide catalog. Source version: {version}\n\npub mod icons;\npub mod paths;\npub mod unicode;\npub mod version;\n"
    )
}

fn version_source(tag: Option<&str>, icon_count: usize) -> String {
    let version = tag.unwrap_or("unknown");
    format!(
        "pub const LUCIDE_VERSION: &str = {version:?};\npub const LUCIDE_RELEASE_TAG: &str = {version:?};\npub const LUCIDE_ICON_COUNT: usize = {icon_count};\npub const LUCIDE_SOURCE_URL: &str = \"https://github.com/lucide-icons/lucide/releases/tag/{version}\";\n"
    )
}

fn icons_source(icons: &[IconSource]) -> String {
    let variants = icons
        .iter()
        .map(|icon| format!("    {},\n", icon.variant))
        .collect::<String>();
    let all = icons
        .iter()
        .map(|icon| format!("    Icon::{},\n", icon.variant))
        .collect::<String>();
    let names = icons
        .iter()
        .map(|icon| format!("            Self::{} => {:?},\n", icon.variant, icon.name))
        .collect::<String>();
    let assets = icons
        .iter()
        .map(|icon| {
            format!(
                "            Self::{} => {:?},\n",
                icon.variant,
                format!("assets/lucide/{}.svg", icon.name)
            )
        })
        .collect::<String>();
    let view_boxes = icons
        .iter()
        .map(|icon| {
            format!(
                "            Self::{} => {:?},\n",
                icon.variant, icon.view_box
            )
        })
        .collect::<String>();
    let svgs = icons
        .iter()
        .map(|icon| format!("            Self::{} => {:?},\n", icon.variant, icon.svg))
        .collect::<String>();

    format!(
        "#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]\npub enum Icon {{\n{variants}}}\n\nconst ALL_ICONS: &[Icon] = &[\n{all}];\n\nimpl Icon {{\n    pub const fn all() -> &'static [Self] {{ ALL_ICONS }}\n\n    pub const fn name(self) -> &'static str {{\n        match self {{\n{names}        }}\n    }}\n\n    pub const fn asset_path(self) -> &'static str {{\n        match self {{\n{assets}        }}\n    }}\n\n    pub const fn view_box(self) -> &'static str {{\n        match self {{\n{view_boxes}        }}\n    }}\n\n    pub fn from_asset_path(path: &str) -> Option<Self> {{\n        let path = path.trim_start_matches('/');\n        ALL_ICONS.iter().copied().find(|icon| icon.asset_path() == path)\n    }}\n\n    #[cfg(feature = \"svg\")]\n    pub const fn svg_str(self) -> &'static str {{\n        match self {{\n{svgs}        }}\n    }}\n\n    #[cfg(feature = \"svg\")]\n    pub const fn svg_bytes(self) -> &'static [u8] {{\n        self.svg_str().as_bytes()\n    }}\n\n    #[cfg(feature = \"path-data\")]\n    pub const fn path_data(self) -> &'static str {{\n        super::paths::path_data(self)\n    }}\n}}\n\nimpl std::fmt::Display for Icon {{\n    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n        formatter.write_str(self.name())\n    }}\n}}\n"
    )
}

fn paths_source(icons: &[IconSource]) -> String {
    let entries = icons
        .iter()
        .map(|icon| {
            format!(
                "        Icon::{} => {:?},\n",
                icon.variant,
                icon.paths.join(";")
            )
        })
        .collect::<String>();
    format!(
        "#[cfg(feature = \"path-data\")]\nuse super::icons::Icon;\n\n#[cfg(feature = \"path-data\")]\npub const fn path_data(icon: Icon) -> &'static str {{\n    match icon {{\n{entries}    }}\n}}\n"
    )
}

fn unicode_source() -> String {
    "#[cfg(feature = \"font\")]\nuse super::icons::Icon;\n\n#[cfg(feature = \"font\")]\npub const fn unicode(_icon: Icon) -> Option<char> {\n    None\n}\n"
        .to_string()
}
