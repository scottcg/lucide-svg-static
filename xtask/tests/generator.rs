use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn generates_the_fixture_catalog() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let input = std::env::temp_dir().join(format!("lucide-static-svg-fixtures-{unique}"));
    let output = std::env::temp_dir().join(format!("lucide-static-svg-generator-{unique}"));
    fs::create_dir_all(&input).unwrap();
    fs::write(
        input.join("chevron-right.svg"),
        r#"<svg viewBox="0 0 24 24"><path d="m9 18 6-6-6-6"/></svg>"#,
    )
    .unwrap();
    fs::write(
        input.join("file-text.svg"),
        r#"<svg viewBox="0 0 24 24"><path d="M15 2H6"/><path d="M14 2v6h6"/></svg>"#,
    )
    .unwrap();
    fs::write(
        input.join("chevron-up.svg"),
        r#"<svg viewBox="0 0 24 24"><path d="m18 15-6-6-6 6"/></svg>"#,
    )
    .unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["generate", "--input"])
        .arg(&input)
        .args(["--output"])
        .arg(&output)
        .status()
        .unwrap();
    assert!(status.success());
    assert!(fs::read_to_string(output.join("icons.rs"))
        .unwrap()
        .contains("ChevronRight"));
    assert!(fs::read_to_string(output.join("paths.rs"))
        .unwrap()
        .contains("m9 18 6-6-6-6"));
    fs::remove_dir_all(output).unwrap();
    fs::remove_dir_all(input).unwrap();
}
