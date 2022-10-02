use raffia::ast::{
    Declaration, Ident, InterpolableIdent, QualifiedRule, SimpleBlock, Statement, Stylesheet,
};
use std::collections::HashMap;

struct Sorter {
    weight_map: HashMap<String, isize>,
    default_weight: isize,
}

pub struct Config {
    pub weight_map: Option<HashMap<String, isize>>,
    pub default_weight: Option<isize>,
}

impl Sorter {
    fn new(weight_map: HashMap<String, isize>, default_weight: isize) -> Self {
        Sorter {
            weight_map,
            default_weight,
        }
    }

    fn sheet<'a>(&self, sheet: &mut Stylesheet<'a>) {
        sheet.statements.iter_mut().for_each(|mut stat| {
            match stat {
                Statement::QualifiedRule(rule) => self.qualified_rule(rule),
                _ => (),
            };
        });
    }

    fn qualified_rule<'a>(&self, qualified_rule: &mut QualifiedRule<'a>) {
        self.simple_block(&mut qualified_rule.block);
    }

    fn simple_block<'a>(&self, block: &mut SimpleBlock<'a>) {
        block.statements.sort_by(|a, b| {
            let a = self.declaration(a.as_declaration().unwrap());
            let b = self.declaration(b.as_declaration().unwrap());

            return b.cmp(&a);
        });
    }

    fn declaration<'a>(&self, declar: &Declaration<'a>) -> isize {
        let name = self.ident(declar.name.as_literal().unwrap());

        let name = name.as_str();

        if self.weight_map.contains_key(name) {
            return self.weight_map.get(name).unwrap().clone();
        }

        return self.default_weight.clone();
    }

    fn ident(&self, ident: &Ident) -> String {
        return ident.name.clone().to_string();
    }
}

pub fn run(ast: &mut Stylesheet, config: Config) {
    let mut map = HashMap::new();
    let mut weight = 0;

    if let Some(weight_map) = config.weight_map {
        map.extend(weight_map);
    }

    if let Some(default_weight) = config.default_weight {
        weight = default_weight;
    }

    let sorter = Sorter::new(map, weight);

    sorter.sheet(ast);
}
