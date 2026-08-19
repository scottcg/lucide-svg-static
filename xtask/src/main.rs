//! Repository maintenance commands.

#![forbid(unsafe_code)]

mod generate;
mod parse;

use std::{env, path::PathBuf};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut arguments = env::args().skip(1);
    let Some(command) = arguments.next() else {
        return Err(usage());
    };

    if command != "generate" {
        return Err(format!("unknown command `{command}`\n\n{}", usage()));
    }

    let mut input = None;
    let mut archive = None;
    let mut output = PathBuf::from("crates/lucide-static-svg/src/generated");
    let mut tag = None;

    while let Some(argument) = arguments.next() {
        let mut value = || {
            arguments
                .next()
                .ok_or_else(|| format!("missing value for `{argument}`"))
        };
        match argument.as_str() {
            "--input" => input = Some(PathBuf::from(value()?)),
            "--archive" => archive = Some(PathBuf::from(value()?)),
            "--output" => output = PathBuf::from(value()?),
            "--tag" => tag = Some(value()?),
            "-h" | "--help" => {
                println!("{}", usage());
                return Ok(());
            }
            _ => return Err(format!("unknown argument `{argument}`\n\n{}", usage())),
        }
    }

    if input.is_some() == archive.is_some() {
        return Err(
            "provide exactly one of `--input <directory>` or `--archive <zip>`".to_string(),
        );
    }
    let icons = match (input, archive) {
        (Some(input), None) => parse::load_icons(&input)?,
        (None, Some(archive)) => parse::load_archive(&archive)?,
        _ => unreachable!(),
    };
    generate::write_generated(&output, tag.as_deref(), &icons)
}

fn usage() -> String {
    "Usage: cargo xtask generate (--input <directory> | --archive <zip>) [--output <directory>] [--tag <version>]"
        .to_string()
}
