#![deny(clippy::all)]
#![allow(clippy::needless_update)]

pub use std::fmt::Result;

use codegen_macro::emitter;
use raffia::ast::{
    ClassSelector, ComplexSelector, ComplexSelectorChild, ComponentValue, CompoundSelector,
    Declaration, Dimension, Function, IdSelector, Ident, InterpolableIdent, Length, QualifiedRule,
    SelectorList, SimpleBlock, SimpleSelector, Statement, Stylesheet,
};

mod emit;
mod sep;
mod types;
mod writer;

use crate::sep::Sep;
pub use emit::Emit;
use sep::SepSerialize;
pub use types::css::{sep::CssSep, CssWriter::CssWriter};
pub use writer::Writer;

#[macro_use]
mod macros;

pub struct CodeGenerator<W, S>
where
    W: Writer,
    S: SepSerialize,
{
    writer: W,
    serialize: S,
}

impl<W, S> CodeGenerator<W, S>
where
    W: Writer,
    S: SepSerialize,
{
    pub fn new(write: W, serialize: S) -> Self {
        CodeGenerator {
            writer: write,
            serialize,
        }
    }

    #[emitter]
    pub fn emit_stylesheet(&mut self, node: &Stylesheet<'_>) -> crate::Result {
        self.emit_list(node.statements[..].into())?;
    }

    #[emitter]
    pub fn emit_statements(&mut self, node: &Statement<'_>) -> crate::Result {
        match node {
            Statement::AtRule(_) => todo!(),
            Statement::Declaration(declaration) => emit!(self, declaration),
            Statement::KeyframeBlock(_) => todo!(),
            Statement::LessVariableDeclaration(_) => todo!(),
            Statement::QualifiedRule(rule) => emit!(self, rule),
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

        return Ok(());
    }

    #[emitter]
    pub fn emit_qualified_rule(&mut self, rule: &QualifiedRule<'_>) -> crate::Result {
        emit!(self, &rule.selector);
        emit!(self, &rule.block);
    }

    #[emitter]
    pub fn emit_selector_list(&mut self, rule: &SelectorList<'_>) -> crate::Result {
        self.emit_list(rule.selectors[..].into())?;
    }

    fn emit_list<Elem>(&mut self, nodes: &[Elem]) -> crate::Result
    where
        Self: Emit<Elem>,
    {
        for (idx, node) in nodes.iter().enumerate() {
            if idx != 0 {
                self.writer.write_raw(",".to_string())?;
            }
            emit!(self, node);
        }

        Ok(())
    }

    #[emitter]
    pub fn emit_complex_selector(&mut self, selector: &ComplexSelector<'_>) -> crate::Result {
        self.emit_list(selector.children[..].into())?;
    }

    #[emitter]
    pub fn emit_computed_selector(&mut self, selector: &ComplexSelectorChild<'_>) -> crate::Result {
        match selector {
            ComplexSelectorChild::CompoundSelector(selector) => {
                emit!(self, selector);
            }
            ComplexSelectorChild::Combinator(combinator) => {}
        }
    }

    #[emitter]
    pub fn emit_compound_selector(&mut self, selector: &CompoundSelector<'_>) -> crate::Result {
        self.emit_list(selector.children[..].into())?;
    }

    #[emitter]
    pub fn emit_simple_selector(&mut self, selector: &SimpleSelector<'_>) -> crate::Result {
        match selector {
            SimpleSelector::Class(class) => emit!(self, class),
            SimpleSelector::Id(id) => emit!(self, id),
            SimpleSelector::Type(_) => todo!(),
            SimpleSelector::Attribute(_) => todo!(),
            SimpleSelector::PseudoClass(_) => todo!(),
            SimpleSelector::PseudoElement(_) => todo!(),
            SimpleSelector::Nesting(_) => todo!(),
            SimpleSelector::SassPlaceholder(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_class_selector(&mut self, selector: &ClassSelector<'_>) -> crate::Result {
        self.writer.write_raw(".".to_string())?;
        emit!(self, selector.name);
    }

    #[emitter]
    pub fn emit_id_selector(&mut self, selector: &IdSelector<'_>) -> crate::Result {
        self.writer.write_raw("#".to_string())?;
        emit!(self, selector.name);
    }

    #[emitter]
    pub fn emit_simple_block(&mut self, rule: &SimpleBlock<'_>) -> crate::Result {
        self.writer.write_raw("{".to_string())?;

        self.writer.write_raw(serialize!(self, Sep::BlockLeft))?;
        self.emit_list(rule.statements[..].into())?;
        self.writer.write_raw(serialize!(self, Sep::BlockRight))?;
    }

    #[emitter]
    pub fn emit_declaration(&mut self, declar: &Declaration<'_>) -> crate::Result {
        emit!(self, declar.name);
        self.writer.write_raw(":".to_string())?;
        self.emit_list(declar.value[..].into())?;
    }

    #[emitter]
    pub fn emit_component_value(&mut self, component_val: &ComponentValue<'_>) -> crate::Result {
        match component_val {
            ComponentValue::BracketBlock(_) => todo!(),
            ComponentValue::Calc(_) => todo!(),
            ComponentValue::Delimiter(_) => todo!(),
            ComponentValue::Dimension(dimension) => emit!(self, dimension),
            ComponentValue::Function(fun) => todo!(),
            ComponentValue::HexColor(_) => todo!(),
            ComponentValue::IdSelector(_) => todo!(),
            ComponentValue::InterpolableIdent(ident) => emit!(self, ident),
            ComponentValue::InterpolableStr(_) => todo!(),
            ComponentValue::LayerName(_) => todo!(),
            ComponentValue::LessVariable(_) => todo!(),
            ComponentValue::LessVariableVariable(_) => todo!(),
            ComponentValue::Number(_) => todo!(),
            ComponentValue::Percentage(_) => todo!(),
            ComponentValue::Ratio(_) => todo!(),
            ComponentValue::SassBinaryExpression(_) => todo!(),
            ComponentValue::SassMap(_) => todo!(),
            ComponentValue::SassNamespacedExpression(_) => todo!(),
            ComponentValue::SassNestingDeclaration(_) => todo!(),
            ComponentValue::SassParenthesizedExpression(_) => todo!(),
            ComponentValue::SassParentSelector(_) => todo!(),
            ComponentValue::SassUnaryExpression(_) => todo!(),
            ComponentValue::SassVariable(_) => todo!(),
            ComponentValue::TokenWithSpan(_) => todo!(),
            ComponentValue::UnicodeRange(_) => todo!(),
            ComponentValue::Url(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_dimension(&mut self, dimension: &Dimension<'_>) -> crate::Result {
        match dimension {
            Dimension::Length(len) => emit!(self, len),
            Dimension::Angle(_) => todo!(),
            Dimension::Duration(_) => todo!(),
            Dimension::Frequency(_) => todo!(),
            Dimension::Resolution(_) => todo!(),
            Dimension::Flex(_) => todo!(),
            Dimension::Unknown(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_length(&mut self, length: &Length<'_>) -> crate::Result {
        self.writer.write_raw(String::from(length.value.raw))?;
        emit!(self, length.unit);
    }

    #[emitter]
    pub fn emit_interpolable_ident(&mut self, ident: &InterpolableIdent<'_>) -> crate::Result {
        // emit!(self, ident.);
        match ident {
            InterpolableIdent::Literal(literal) => emit!(self, literal),
            InterpolableIdent::SassInterpolated(_) => todo!(),
            InterpolableIdent::LessInterpolated(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_literal(&mut self, literal: &Ident<'_>) -> crate::Result {
        self.writer.write_raw(literal.raw.to_string())?;
    }
}
