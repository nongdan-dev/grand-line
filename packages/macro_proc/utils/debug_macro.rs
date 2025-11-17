use crate::prelude::*;
use std::env;

pub fn debug_macro(name: &str, ts: Ts2) {
    if env::var("DEBUG_MACRO").unwrap_or_default() != "1" {
        return;
    }

    #[cfg(feature = "debug_macro_cli")]
    {
        use colored::{Colorize, control::SHOULD_COLORIZE};

        SHOULD_COLORIZE.set_override(true);
        println!("==============================================================================");
        println!("{}", name.bold());
        println!();

        let code = match parse2::<File>(ts.clone()) {
            Ok(v) => prettyplease::unparse(&v),
            _ => ts.to_string(),
        };
        println!("{}", code.bright_black());
    }

    #[cfg(feature = "debug_macro_file")]
    {
        use std::fs::{File, create_dir_all};
        use std::io::Write;
        use std::path::PathBuf;
        use std::process::Command;

        let code = ts.to_string();
        let path = format!("target/grand-line/{name}.rs");
        let path = PathBuf::from(path);

        let parent = path.parent().unwrap_or_else(|| {
            panic!("path.parent: None");
        });
        create_dir_all(parent).unwrap_or_else(|e| {
            panic!("create_dir_all: {e}");
        });

        let mut file = File::create(&path).unwrap_or_else(|e| {
            panic!("File::create: {e}");
        });
        writeln!(file, "{}", code).unwrap_or_else(|e| {
            panic!("writeln!: {e}");
        });

        Command::new("rustfmt")
            .arg("--edition")
            .arg("2024")
            .arg(&path)
            .status()
            .unwrap_or_else(|e| {
                panic!("rustfmt: {e}");
            });
    }
}
