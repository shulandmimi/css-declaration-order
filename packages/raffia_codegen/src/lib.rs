#![deny(clippy::all)]
#![allow(clippy::needless_update)]

pub use std::fmt::Result;

use codegen_macro::emitter;
use raffia::ast::{Declaration, QualifiedRule, SelectorList, SimpleBlock, Statement, Stylesheet};

mod emit;

// pub use self::emit::*;
// pub use self::emit
pub use emit::Emit;

#[macro_use]
mod macros;

pub struct CodeGenerator {}

impl CodeGenerator {
    #[emitter]
    pub fn emit_stylesheet(&mut self, node: &Stylesheet<'_>) -> crate::Result {
        emit!(self, node.statements);
        emit!(self, node.statements);
    }

    #[emitter]
    pub fn emit_statements(&mut self, node: &Vec<Statement<'_>>) -> crate::Result {
        node.iter().for_each(|item| {
            match item {
                Statement::AtRule(_) => todo!(),
                Statement::Declaration(_) => todo!(),
                Statement::KeyframeBlock(_) => todo!(),
                Statement::LessVariableDeclaration(_) => todo!(),
                Statement::QualifiedRule(rule) => {
                    // emit!(self, rule);
                }
                Statement::SassContentAtRule(_) => todo!(),
                Statement::SassDebugAtRule(_) => todo!(),
                Statement::SassEachAtRule(_) => todo!(),
                Statement::SassErrorAtRule(_) => todo!(),
                Statement::SassExtendAtRule(_) => todo!(),
                Statement::SassForAtRule(_) => todo!(),
                Statement::SassForwardAtRule(_) => todo!(),
                Statement::SassFunctionAtRule(_) => todo!(),
                Statement::SassIfAtRule(_) => todo!(),
                Statement::SassIncludeAtRule(_) => todo!(),
                Statement::SassMixinAtRule(_) => todo!(),
                Statement::SassReturnAtRule(_) => todo!(),
                Statement::SassUseAtRule(_) => todo!(),
                Statement::SassVariableDeclaration(_) => todo!(),
                Statement::SassWarnAtRule(_) => todo!(),
                Statement::SassWhileAtRule(_) => todo!(),
                // _ => Ok(()),
            };

            // Ok(())
        });

        // Ok(())
    }

    #[emitter]
    pub fn emit_qualified_rule(&mut self, rule: &QualifiedRule<'_>) -> crate::Result {
        emit!(self, &rule.selector);
        emit!(self, &rule.block);
        // Ok(())
    }

    #[emitter]
    pub fn emit_selector_list(&mut self, rule: &SelectorList<'_>) -> crate::Result {
        // Ok(())
    }

    #[emitter]
    pub fn emit_simple_block(&mut self, rule: &SimpleBlock<'_>) -> crate::Result {
        // Ok(())
    }

    #[emitter]
    pub fn emit_declaration(&mut self, declar: &Declaration<'_>) -> crate::Result {
        // Ok(())
    }
}
