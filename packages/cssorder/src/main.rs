#![feature(fs_try_exists)]
#![feature(is_some_with)]
use std::{
    fs::{read, try_exists},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use raffia::{ast::Stylesheet, Syntax};
use raffia_codegen::{CodeGenerator, Emit, Writer};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long)]
    paths: Vec<String>,
}

fn exists(path: &PathBuf) -> bool {
    return try_exists(path).is_ok_and(|ok| *ok);
}

fn main() {
    // let args = Cli::parse();

    // if args.paths.len() == 0 {
    //     return;
    // }

    // let filename = PathBuf::from_str(args.paths[0].as_str()).unwrap();
    // if !exists(&filename) {
    //     return;
    // };

    let filename = PathBuf::from_str("../../examples/css/index.css").unwrap();

    let content = read(filename).unwrap();

    let content = String::from_utf8_lossy(content[..].into()).to_string();

    let mut parser = raffia::Parser::new(&content, Syntax::Css);

    let mut ast = parser.parse::<Stylesheet>().unwrap();

    cssorder::parser::css::run(&mut ast);

    let mut stdout = std::io::stdout();

    let mut writer = raffia_codegen::CssWriter::new(stdout);

    let mut code_gen = CodeGenerator::new(writer);

    code_gen.emit(&mut ast);
    // code_gen.emit_stylesheet(node)
    // code_gen.emit(&mut ast);
    // println!("{:#?}", ast);
}
