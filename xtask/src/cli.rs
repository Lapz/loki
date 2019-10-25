use std::path::PathBuf;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "loki-tools")]
pub enum Cli {
    Ast {
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
    },
    Hir,
}
// pub struct Cli {
//     /// Regenerate the Ast
//     #[structopt(short, long)]
//     pub ast: bool,
//     /// Regenerate the HIR,
//     #[structopt(short, long)]
//     pub hir:bool
// }

pub enum Commands {
    GenerateSyntax,
    GenerateHir,
}
