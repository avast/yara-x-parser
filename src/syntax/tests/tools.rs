// Lot of helping functions used mainly for generating code,
// modifying it and formating it

use std::{
    fs,
    path::{Path, PathBuf},
};
use xshell::{cmd, pushenv};

pub fn add_preamble(generator: &'static str, mut text: String) -> String {
    let preamble = format!("//! Generated by `{}`, do not edit by hand.\n\n", generator);
    text.insert_str(0, &preamble);
    text
}

fn ensure_rustfmt() {
    let version = cmd!("rustfmt --version").read().unwrap_or_default();
    if !version.contains("stable") {
        panic!(
            "Failed to run rustfmt from toolchain 'stable'. \
                 Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }
}

pub fn reformat(text: String) -> String {
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt();
    let rustfmt_toml = project_root().join("rustfmt.toml");
    let mut stdout = cmd!("rustfmt --config-path {rustfmt_toml}").stdin(text).read().unwrap();
    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }
    stdout
}

pub fn ensure_file_contents(file: &Path, contents: &str) {
    if let Ok(old_contents) = fs::read_to_string(file) {
        if normalize_newlines(&old_contents) == normalize_newlines(contents) {
            // File is already up to date.
            return;
        }
    }

    let display_path = file.strip_prefix(&project_root()).unwrap_or(file);
    eprintln!(
        "\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n",
        display_path.display()
    );
    if std::env::var("CI").is_ok() {
        eprintln!("    NOTE: run `cargo test` locally and commit the updated files\n");
    }
    if let Some(parent) = file.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(file, contents).unwrap();
    panic!("some file was not up to date and has been updated, simply re-run the tests")
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}

pub fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir).to_owned()
}

pub fn to_lower_snake_case(s: &str) -> String {
    to_snake_case(s, char::to_ascii_lowercase)
}
pub fn to_upper_snake_case(s: &str) -> String {
    to_snake_case(s, char::to_ascii_uppercase)
}
fn to_snake_case<F: Fn(&char) -> char>(s: &str, change_case: F) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = false;
    for c in s.chars() {
        // `&& prev` is required to not insert `_` before the first symbol.
        if c.is_ascii_uppercase() && prev {
            // This check is required to not translate `Weird_Case` into `weird__case`.
            if !buf.ends_with('_') {
                buf.push('_')
            }
        }
        prev = true;

        buf.push(change_case(&c));
    }
    buf
}

pub fn to_pascal_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev_is_underscore = true;
    for c in s.chars() {
        if c == '_' {
            prev_is_underscore = true;
        } else if prev_is_underscore {
            buf.push(c.to_ascii_uppercase());
            prev_is_underscore = false;
        } else {
            buf.push(c.to_ascii_lowercase());
        }
    }
    buf
}
