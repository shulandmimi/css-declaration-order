#![feature(fs_try_exists)]
#![feature(is_some_with)]
use std::{
    collections::HashMap,
    fs::{read, try_exists},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use cssorder::parser::css::Config;
use raffia::{ast::Stylesheet, Syntax};
use raffia_codegen::{CodeGenerator, Emit};

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

    let mut key_weight_map = HashMap::new();

    key_weight_map.insert("width".to_string(), 2000);
    key_weight_map.insert("height".to_string(), 1999);

    cssorder::parser::css::run(
        &mut ast,
        Config {
            weight_map: Some(key_weight_map),
            default_weight: Some(1000),
        },
    );

    let stdout = std::io::stdout();

    let writer = raffia_codegen::CssWriter::new(stdout);

    let mut code_gen = CodeGenerator::new(writer);

    code_gen.emit(&mut ast).unwrap();
}
