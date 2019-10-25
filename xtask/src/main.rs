mod cli;

use crate::cli::{Cli, Commands};
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const GRAMMAR_ROOT: &'static str = "loki-syntax/src/";
fn main() -> std::io::Result<()> {
    let opt = Cli::from_args();

    match opt {
        Cli::Ast { mut output } => {
            if let Some(ref mut path) = output {
                generate(Some(path))?
            } else {
                generate(None)?
            }
        }
        _ => unimplemented!(),
    };

    Ok(())
}

pub fn generate(out: Option<&mut PathBuf>) -> io::Result<()> {
    println!("{:?}", project_root());

    let mut root = project_root();

    if let Some(path) = out {
        root.join(path);
    } else {
        root = root.join(GRAMMAR_ROOT);
    }

    println!("{:?}", &root.join("grammar.ron"));

    match teraron::generate(
        &root.clone().join("ast.rs.tera"),
        &root.join("grammar.ron"),
        teraron::Mode::Overwrite,
    ) {
        Ok(_) => (),
        Err(e) => println!("teraron error {}", e),
    };
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
