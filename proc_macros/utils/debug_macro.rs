use crate::prelude::*;
use std::env;

pub fn debug_macro(name: &str, ts: TokenStream2) {
    if env::var("DEBUG_MACRO").unwrap_or_default() != "1" {
        return;
    }

    #[cfg(feature = "debug_macro_cli")]
    {
        use colored::{Colorize, control::SHOULD_COLORIZE};
        use syn::{File, parse2};

        SHOULD_COLORIZE.set_override(true);
        println!("==============================================================================");
        println!("{}", name.bold());
        println!();

        let code = match parse2::<File>(ts.clone()) {
            Ok(v) => prettyplease::unparse(&v),
            _ => str!(ts),
        };
        println!("{}", code.bright_black());
    }

    #[cfg(feature = "debug_macro_file")]
    {
        use std::fs::{File, create_dir_all};
        use std::io::Write;
        use std::path::PathBuf;
        use std::process::Command;

        let content = str!(ts);

        let path = PathBuf::from(strf!("target/grand-line/{}.rs", name));
        let _ = create_dir_all(
            path.parent()
                .unwrap_or_else(|| panic_with_location!("path.parent: None")),
        )
        .unwrap_or_else(|e| panic_with_location!("create_dir_all: {}", e));

        let mut file =
            File::create(&path).unwrap_or_else(|e| panic_with_location!("File::create: {}", e));
        let _ = writeln!(file, "{}", content)
            .unwrap_or_else(|e| panic_with_location!("writeln!: {}", e));

        let _ = Command::new("rustfmt")
            .arg("--edition")
            .arg("2024")
            .arg(&path)
            .status()
            .unwrap_or_else(|e| panic_with_location!("rustfmt: {}", e));
    }
}
