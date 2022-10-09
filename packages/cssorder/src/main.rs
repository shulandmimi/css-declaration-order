#![feature(is_some_with)]
#![warn(dead_code)]

use glob::glob;
use std::{collections::HashMap, fs::read, path::PathBuf};

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

// fn exists(path: &PathBuf) -> bool {
//     return try_exists(path).is_ok_and(|ok| *ok);
// }

fn main() {
    let args = Cli::parse();

    if args.paths.len() == 0 {
        return;
    }

    let filenames: Vec<PathBuf> = args
        .paths
        .into_iter()
        .map(|path| {
            glob(&path).expect(format!("not match files, please check path {}", path).as_str())
        })
        .map(|paths| {
            paths.filter_map(|path| match path {
                Ok(filename) => Some(filename),
                Err(_) => None,
            })
        })
        .flatten()
        .collect();

    let mut key_weight_map = HashMap::new();

    key_weight_map.insert("width".to_string(), 2000);
    key_weight_map.insert("height".to_string(), 1999);

    let stdout = std::io::stdout();

    let writer = raffia_codegen::CssWriter::new(stdout);

    let css_serialize = raffia_codegen::CssSep::new();

    let mut code_gen = CodeGenerator::new(writer, css_serialize);

    for file in filenames {
        let content = read(file).unwrap();

        let content = String::from_utf8_lossy(content[..].into()).to_string();

        let mut parser = raffia::Parser::new(&content, Syntax::Css);
        let mut ast = parser.parse::<Stylesheet>().unwrap();

        cssorder::parser::css::run(
            &mut ast,
            Config {
                weight_map: Some(key_weight_map.clone()),
                default_weight: Some(1000),
            },
        );

        code_gen.emit(&mut ast).unwrap();
    }
}
