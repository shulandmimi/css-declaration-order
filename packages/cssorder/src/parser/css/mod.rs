use raffia::ast::{
    Declaration, Ident, InterpolableIdent, QualifiedRule, SimpleBlock, Statement, Stylesheet,
};
use std::collections::HashMap;
use std::{borrow::BorrowMut, iter::Map};

struct Sorter {
    // sheet: Stylesheet,
    weight_map: HashMap<String, isize>,
}

impl Sorter {
    fn new(weight_map: HashMap<String, isize>) -> Self {
        Sorter { weight_map }
    }

    fn sheet<'a>(&self, sheet: &mut Stylesheet<'a>) {
        sheet.statements.iter_mut().for_each(|mut stat| {
            let mut t = stat.as_qualified_rule().unwrap().clone();
            self.qualified_rule(&mut t);
            *stat = Statement::QualifiedRule(t);
        });
    }

    fn qualified_rule<'a>(&self, qualified_rule: &mut QualifiedRule<'a>) {
        self.simple_block(&mut qualified_rule.block);
    }

    fn simple_block<'a>(&self, block: &mut SimpleBlock<'a>) {
        block.statements.sort_by(|a, b| {
            let a = self.declaration(a.as_declaration().unwrap());
            let b = self.declaration(b.as_declaration().unwrap());

            return a.cmp(&b);
        });
    }

    fn declaration<'a>(&self, declar: &Declaration<'a>) -> isize {
        let name = self.ident(declar.name.as_literal().unwrap());

        let name = name.as_str();

        if self.weight_map.contains_key(name) {
            return self.weight_map.get(name).unwrap().clone();
        }

        return 1;
    }

    fn ident(&self, ident: &Ident) -> String {
        return ident.name.clone().to_string();
    }
}

pub fn run(ast: &mut Stylesheet) {
    let mut map = HashMap::new();
    map.insert("width".to_string(), 0);
    let sorter = Sorter::new(map);

    sorter.sheet(ast);
}
