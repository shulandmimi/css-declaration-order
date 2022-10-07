use raffia::{ast::Stylesheet, Parser, Syntax};
use raffia_codegen::{CodeGenerator, FormatSep, SepRule, SepSerialize, Writer};

pub fn code_gen_factor<W, S>(source: &str, writer: W, sep: S)
where
    W: Writer,
    S: SepSerialize<FormatSep> + SepSerialize<SepRule>,
{
    let mut code_gen = CodeGenerator::new(writer, sep);

    let mut parser = Parser::new(source, Syntax::Css);

    let ast = parser.parse::<Stylesheet>().unwrap();

    code_gen.emit_stylesheet(&ast).unwrap();
}

#[macro_export]
macro_rules! code_gen {
    ($source:expr) => {{
        use raffia_codegen::{CssSep, CssWriter};

        let source: &str = $source;
        let mut result = Vec::new();
        let writer = CssWriter::new(&mut result);
        let css = CssSep::new();
        crate::macros::code_gen_factor(source, writer, css);

        String::from_utf8_lossy(result[..].into())
            .trim()
            .to_string()
    }};
}
