use std::{fs, path::Path, process::Command};

#[derive(Debug, Eq, PartialEq)]
pub struct IconSource {
    pub variant: String,
    pub name: String,
    pub view_box: String,
    pub svg: String,
    pub paths: Vec<String>,
}

pub fn load_icons(input: &Path) -> Result<Vec<IconSource>, String> {
    let mut paths = fs::read_dir(input)
        .map_err(|error| format!("could not read {}: {error}", input.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("could not enumerate {}: {error}", input.display()))?;
    paths.sort();

    let mut icons = Vec::new();
    for path in paths {
        if path.extension().and_then(|extension| extension.to_str()) != Some("svg") {
            continue;
        }
        icons.push(parse_icon(&path)?);
    }

    validate_icons(icons, input.display().to_string())
}

fn parse_icon(path: &Path) -> Result<IconSource, String> {
    let svg = fs::read_to_string(path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    parse_source(
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| format!("invalid UTF-8 filename: {}", path.display()))?,
        svg,
        path.display().to_string(),
    )
}

pub fn load_archive(archive_path: &Path) -> Result<Vec<IconSource>, String> {
    if !archive_path.is_file() {
        return Err(format!(
            "archive does not exist: {}",
            archive_path.display()
        ));
    }
    let listing = Command::new("unzip")
        .args(["-Z1"])
        .arg(archive_path)
        .output()
        .map_err(|error| format!("could not run unzip: {error}"))?;
    if !listing.status.success() {
        return Err(format!("could not list {}", archive_path.display()));
    }
    let mut entries = String::from_utf8(listing.stdout)
        .map_err(|error| format!("archive listing is not UTF-8: {error}"))?
        .lines()
        .filter(|name| name.starts_with("icons/") && name.ends_with(".svg"))
        .map(str::to_string)
        .collect::<Vec<_>>();
    entries.sort();

    let mut icons = Vec::with_capacity(entries.len());
    for name in entries {
        let stem = Path::new(&name)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| format!("invalid icon filename in archive: {name}"))?;
        let contents = Command::new("unzip")
            .args(["-p"])
            .arg(archive_path)
            .arg(&name)
            .output()
            .map_err(|error| format!("could not run unzip for {name}: {error}"))?;
        if !contents.status.success() {
            return Err(format!("could not read {name} from archive"));
        }
        let svg = String::from_utf8(contents.stdout)
            .map_err(|error| format!("could not decode {name} as UTF-8: {error}"))?;
        icons.push(parse_source(
            stem,
            svg,
            format!("{}:{name}", archive_path.display()),
        )?);
    }

    validate_icons(icons, archive_path.display().to_string())
}

fn parse_source(stem: &str, svg: String, source_name: String) -> Result<IconSource, String> {
    let document = roxmltree::Document::parse(&svg)
        .map_err(|error| format!("could not parse {source_name}: {error}"))?;
    let root = document.root_element();
    let view_box = root
        .attribute("viewBox")
        .ok_or_else(|| format!("{source_name} is missing a viewBox"))?
        .to_string();
    let paths = document
        .descendants()
        .filter(|node| node.has_tag_name("path"))
        .filter_map(|node| node.attribute("d"))
        .map(str::to_string)
        .collect::<Vec<_>>();
    let variant = rust_variant_name(stem)?;
    Ok(IconSource {
        variant,
        name: stem.to_string(),
        view_box,
        svg,
        paths,
    })
}

fn validate_icons(icons: Vec<IconSource>, source_name: String) -> Result<Vec<IconSource>, String> {
    if icons.is_empty() {
        return Err(format!("no SVG files found in {source_name}"));
    }
    for (index, icon) in icons.iter().enumerate() {
        if icons[..index]
            .iter()
            .any(|other| other.variant == icon.variant)
        {
            return Err(format!(
                "duplicate Rust variant `{}` in {source_name}",
                icon.variant
            ));
        }
    }
    Ok(icons)
}

pub fn rust_variant_name(name: &str) -> Result<String, String> {
    let words = name
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    if words.is_empty() {
        return Err(format!("cannot derive a Rust name from `{name}`"));
    }

    let mut variant = String::new();
    for word in words {
        let mut characters = word.chars();
        if let Some(first) = characters.next() {
            variant.extend(first.to_uppercase());
            variant.extend(characters);
        }
    }
    if variant
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        variant = format!("Icon{variant}");
    }
    if is_rust_keyword(&variant) {
        variant.push_str("Icon");
    }
    Ok(variant)
}

fn is_rust_keyword(value: &str) -> bool {
    matches!(
        value,
        "As" | "Break"
            | "Const"
            | "Continue"
            | "Crate"
            | "Else"
            | "Enum"
            | "Extern"
            | "False"
            | "Fn"
            | "For"
            | "If"
            | "Impl"
            | "In"
            | "Let"
            | "Loop"
            | "Match"
            | "Mod"
            | "Move"
            | "Mut"
            | "Pub"
            | "Ref"
            | "Return"
            | "Self"
            | "SelfType"
            | "Static"
            | "Struct"
            | "Super"
            | "Trait"
            | "True"
            | "Type"
            | "Unsafe"
            | "Use"
            | "Where"
            | "While"
            | "Async"
            | "Await"
            | "Dyn"
    )
}

#[cfg(test)]
mod tests {
    use super::rust_variant_name;

    #[test]
    fn normalizes_icon_names() {
        assert_eq!(rust_variant_name("chevron-right").unwrap(), "ChevronRight");
        assert_eq!(rust_variant_name("24-hours").unwrap(), "Icon24Hours");
        assert_eq!(rust_variant_name("match").unwrap(), "MatchIcon");
    }
}
