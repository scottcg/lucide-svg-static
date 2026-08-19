//! Official Lucide icons as static SVG data.
//!
//! The public API is backed by generated sources under `generated/`. Run
//! `just generate` when the upstream catalog or generator changes.

#![forbid(unsafe_code)]

mod generated;

pub use generated::icons::Icon;
pub use generated::version::{
    LUCIDE_ICON_COUNT, LUCIDE_RELEASE_TAG, LUCIDE_SOURCE_URL, LUCIDE_VERSION,
};

/// Resolves a GPUI-compatible asset path to embedded SVG bytes.
#[cfg(feature = "svg")]
pub fn asset_bytes(path: &str) -> Option<&'static [u8]> {
    Icon::from_asset_path(path).map(Icon::svg_bytes)
}

#[cfg(feature = "path-data")]
pub use generated::paths::path_data;

#[cfg(feature = "font")]
pub use generated::unicode::unicode;

#[cfg(test)]
mod tests {
    use super::{Icon, LUCIDE_ICON_COUNT, LUCIDE_RELEASE_TAG, LUCIDE_SOURCE_URL, LUCIDE_VERSION};

    #[test]
    fn catalog_contains_the_initial_icons() {
        assert_eq!(Icon::all().len(), LUCIDE_ICON_COUNT);
        assert_eq!(LUCIDE_VERSION, LUCIDE_RELEASE_TAG);
        assert!(LUCIDE_SOURCE_URL.ends_with(LUCIDE_RELEASE_TAG));
        assert_eq!(Icon::ChevronRight.to_string(), "chevron-right");
    }

    #[test]
    fn asset_paths_round_trip() {
        for icon in Icon::all() {
            assert_eq!(Icon::from_asset_path(icon.asset_path()), Some(*icon));
            assert_eq!(
                Icon::from_asset_path(&format!("/{}", icon.asset_path())),
                Some(*icon)
            );
        }
        assert_eq!(Icon::from_asset_path("assets/lucide/missing.svg"), None);
    }

    #[cfg(feature = "svg")]
    #[test]
    fn svg_assets_have_the_expected_view_box() {
        for icon in Icon::all() {
            assert!(icon.svg_str().contains("viewBox=\"0 0 24 24\""));
            assert_eq!(icon.svg_bytes(), icon.svg_str().as_bytes());
            assert_eq!(
                super::asset_bytes(icon.asset_path()),
                Some(icon.svg_bytes())
            );
        }
    }

    #[cfg(feature = "path-data")]
    #[test]
    fn path_data_is_present() {
        assert_eq!(Icon::ChevronRight.path_data(), "m9 18 6-6-6-6");
    }
}
