use crate::prelude::*;
use std::env;
use std::fs::File as FsFile;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn debug_macro(name: &str, ts: TokenStream2) {
    if env::var("DEBUG_MACRO").unwrap_or_default() != "1" {
        return;
    }

    let content = str!(ts);

    let path = PathBuf::from(strf!("target/grand-line/{}.rs", name));
    let _ = std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    let mut file = FsFile::create(&path).unwrap();
    let _ = writeln!(file, "{}", content).unwrap();

    let _ = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg(&path)
        .status()
        .unwrap();
}
