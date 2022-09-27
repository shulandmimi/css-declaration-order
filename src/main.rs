#![feature(fs_try_exists)]
#![feature(is_some_with)]
use std::{
    fs::{read, try_exists},
    path::PathBuf,
    str::FromStr,
};
// use raffia::Parser;

use clap::Parser;
use raffia::{ast::Stylesheet, Syntax};

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
    let args = Cli::parse();

    // args.paths

    // println!("{:?}", args.paths);

    if args.paths.len() == 0 {
        return;
    }

    let filename = PathBuf::from_str(args.paths[0].as_str()).unwrap();
    if !exists(&filename) {
        return;
    };

    let content = read(filename).unwrap();

    let content = String::from_utf8_lossy(content[..].into()).to_string();

    let mut parser = raffia::Parser::new(&content, Syntax::Css);

    let mut ast = parser.parse::<Stylesheet>().unwrap();

    cssorder::parser::css::run(&mut ast);

    println!("{:#?}", ast);
}
