//! Executable option parsing and management.
//!
//! Use the `*Options::parse()` functions to get a program's configuration,
//! as parsed from the commandline.
//!
//! # Examples
//!
//! ```no_run
//! # use pir_8_emu::options::AssemblerOptions;
//! let opts = AssemblerOptions::parse();
//! println!("{:#?}", opts);
//! ```


use clap::{AppSettings, App, Arg};
use std::path::PathBuf;
use std::fs;


/// Representation of the assembler's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AssemblerOptions {
    /// The input assembly files, with `None` being stdin
    ///
    /// Guaranteed to be non-empty
    ///
    /// Defaults to `[None]` (stdin)
    pub input: Vec<Option<(String, PathBuf)>>,
    /// File to write the binary to, or `None` for stdout
    ///
    /// Parent directory must exist
    ///
    /// Default: `"a.p8b"`
    pub output: Option<(String, PathBuf)>,
}


impl AssemblerOptions {
    /// Parse `env`-wide command-line arguments into an `AssemblerOptions` instance
    pub fn parse() -> AssemblerOptions {
        let matches = App::new("pir-8-as")
            .version(crate_version!())
            .author(crate_authors!())
            .about("Assembler for the pir-8")
            .setting(AppSettings::ColoredHelp)
            .args(&[Arg::from_usage("-o [BINFILE] 'Name of the the binary-file output'").default_value("a.p8b").validator(output_file_validator),
                    Arg::from_usage("[ASMFILE]... 'Packages to update'")
                        .empty_values(false)
                        .validator(|s| if s == "-" {
                            Ok(())
                        } else {
                            filesystem_validator("Assembly file", false, &s)
                        })])
            .get_matches();

        AssemblerOptions {
            input: matches.values_of("ASMFILE")
                .map(|ff| {
                    ff.map(|f| match f {
                            "-" => None,
                            _ => Some((f.to_string(), fs::canonicalize(f).unwrap())),
                        })
                        .collect()
                })
                .unwrap_or_else(|| vec![None]),
            output: match matches.value_of("o").unwrap_or("a.p8b") {
                "-" => None,
                f => Some(output_file_process(f)),
            },
        }
    }
}


fn filesystem_validator(label: &str, directory: bool, s: &str) -> Result<(), String> {
    fs::canonicalize(&s).map_err(|_| format!("{} \"{}\" not found", label, s)).and_then(|f| if f.is_dir() == directory {
        Ok(())
    } else {
        Err(format!("{} \"{}\" not a {}", label, s, if directory { "directory" } else { "file" }))
    })
}

fn output_file_validator(s: String) -> Result<(), String> {
    if s == "-" {
        return Ok(());
    }

    let mut buf = PathBuf::from(s);
    if buf.exists() && buf.is_dir() {
        Err(format!("Output file \"{}\" is a directory", buf.display()))
    } else {
        buf.pop();

        // Handle pathless filename
        if buf.as_os_str().is_empty() {
            Ok(())
        } else {
            buf.canonicalize().map(|_| ()).map_err(|e| format!("Output file: {}", e))
        }
    }
}


fn output_file_process(file: &str) -> (String, PathBuf) {
    let mut file = PathBuf::from(file);
    let file_name = file.file_name().unwrap().to_os_string();

    file.pop();
    // Handle pathless filename
    if file.as_os_str().is_empty() {
        file.push(".");
    }

    (file_name.to_str().unwrap().to_string(),
     file.canonicalize()
         .map(|mut p| {
             p.push(file_name);
             p
         })
         .unwrap())
}
