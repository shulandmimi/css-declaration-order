#![deny(clippy::all)]
#![allow(clippy::needless_update)]

pub use std::fmt::Result;

use codegen_macro::emitter;
use raffia::{
    ast::{
        AtRule, AtRulePrelude, ClassSelector, ComplexSelector, ComplexSelectorChild,
        ComponentValue, CompoundSelector, Declaration, Dimension, Duration, Function, IdSelector,
        Ident, InterpolableIdent, Length, NsPrefix, NsPrefixKind, NsPrefixUniversal,
        PseudoClassSelector, PseudoClassSelectorArg, PseudoElementSelector,
        PseudoElementSelectorArg, QualifiedRule, SelectorList, SimpleBlock, SimpleSelector,
        Statement, Str, Stylesheet, TagNameSelector, TokenSeq, TypeSelector, UniversalSelector,
        WqName,
    },
    token::{self, Comma, Hash, Number, Token, TokenWithSpan},
};

mod emit;
mod sep;
mod types;
mod writer;

use crate::sep::SepRule;
pub use emit::Emit;
use sep::{FormatSep, SepSerialize};
pub use types::css::{sep::CssSep, CssWriter::CssWriter};
pub use writer::Writer;

#[macro_use]
mod macros;

pub struct CodeGenerator<W, S>
where
    W: Writer,
    S: SepSerialize<SepRule> + SepSerialize<FormatSep>,
{
    writer: W,
    serialize: S,
}

impl<W, S> CodeGenerator<W, S>
where
    W: Writer,
    S: SepSerialize<SepRule> + SepSerialize<FormatSep>,
{
    pub fn new(write: W, serialize: S) -> Self {
        CodeGenerator {
            writer: write,
            serialize,
        }
    }

    #[emitter]
    pub fn emit_stylesheet(&mut self, node: &Stylesheet<'_>) -> crate::Result {
        self.emit_list(node.statements[..].into(), FormatSep::MUTIPLE_LINE)?;
    }

    #[emitter]
    pub fn emit_statements(&mut self, node: &Statement<'_>) -> crate::Result {
        match node {
            Statement::AtRule(rule) => emit!(self, rule),
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

    /// `@xxxx`
    ///
    /// ```css
    /// @charset xxx
    /// @media
    /// ...
    /// ```
    #[emitter]
    pub fn emit_at_rule(&mut self, at_rule: &AtRule<'_>) -> crate::Result {
        emit!(self, at_rule.name);
        write_raw!(self, Some(":".to_string()))?;
        emit!(self, at_rule.prelude);
        emit!(self, at_rule.block);
    }

    /// `prelude`
    ///
    /// ```css
    /// /** (max-width: 1199.98px) */
    /// @media (max-width: 1199.98px) {}
    /// ```
    #[emitter]
    pub fn emit_prelude(&mut self, prelude: &AtRulePrelude<'_>) -> crate::Result {
        match prelude {
            AtRulePrelude::Charset(v) => emit!(self, v),
            AtRulePrelude::ColorProfile(v) => todo!(),
            AtRulePrelude::Container(_) => todo!(),
            AtRulePrelude::CounterStyle(_) => todo!(),
            AtRulePrelude::CustomMedia(_) => todo!(),
            AtRulePrelude::Document(_) => todo!(),
            AtRulePrelude::FontFeatureValues(_) => todo!(),
            AtRulePrelude::FontPaletteValues(_) => todo!(),
            AtRulePrelude::Import(_) => todo!(),
            AtRulePrelude::Keyframes(_) => todo!(),
            AtRulePrelude::Layer(_) => todo!(),
            AtRulePrelude::Media(_) => todo!(),
            AtRulePrelude::Namespace(_) => todo!(),
            AtRulePrelude::Nest(_) => todo!(),
            AtRulePrelude::Page(_) => todo!(),
            AtRulePrelude::PositionFallback(_) => todo!(),
            AtRulePrelude::Property(_) => todo!(),
            AtRulePrelude::ScrollTimeline(_) => todo!(),
            AtRulePrelude::Supports(_) => todo!(),
            AtRulePrelude::Unknown(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_str(&mut self, str: &Str<'_>) -> crate::Result {
        write_raw!(self, Some(str.raw.to_string()))?;
    }

    #[emitter]
    pub fn emit_qualified_rule(&mut self, rule: &QualifiedRule<'_>) -> crate::Result {
        emit!(self, &rule.selector);
        emit!(self, &rule.block);
    }

    #[emitter]
    pub fn emit_selector_list(&mut self, rule: &SelectorList<'_>) -> crate::Result {
        self.emit_list(
            rule.selectors[..].into(),
            FormatSep::COMMA | FormatSep::SINGLE_LINE,
        )?;
    }

    fn emit_list<Elem>(&mut self, nodes: &[Elem], sep: FormatSep) -> crate::Result
    where
        Self: Emit<Elem>,
    {
        let end = nodes.len();
        new_line!(self, sep);
        for (idx, node) in nodes.iter().enumerate() {
            emit!(self, node);

            if idx != end - 1 {
                write_raw!(self, translate!(self, sep))?;
            } else {
                write_raw!(self, write_last!(self, sep))?;
            }

            new_line!(self, sep)
        }

        Ok(())
    }

    #[emitter]
    pub fn emit_complex_selector(&mut self, selector: &ComplexSelector<'_>) -> crate::Result {
        self.emit_list(
            selector.children[..].into(),
            FormatSep::SPACE | FormatSep::SINGLE_LINE,
        )?;
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
        self.emit_list(
            selector.children[..].into(),
            FormatSep::NONE | FormatSep::SINGLE_LINE,
        )?;
    }

    /// `.foo` | `#foo` | `tag` | `[xx=xx]` | `:last-child` | `::before`
    #[emitter]
    pub fn emit_simple_selector(&mut self, selector: &SimpleSelector<'_>) -> crate::Result {
        match selector {
            SimpleSelector::Class(class) => emit!(self, class),
            SimpleSelector::Id(id) => emit!(self, id),
            SimpleSelector::Type(ty) => emit!(self, ty),
            SimpleSelector::Attribute(_) => todo!(),
            SimpleSelector::PseudoClass(pseudo) => emit!(self, pseudo),
            SimpleSelector::PseudoElement(element) => emit!(self, element),
            SimpleSelector::Nesting(_) => todo!(),
            SimpleSelector::SassPlaceholder(_) => todo!(),
        }
    }

    /// `::before` | `::after`
    #[emitter]
    pub fn emit_pseudo_element_selector(
        &mut self,
        selector: &PseudoElementSelector<'_>,
    ) -> crate::Result {
        write_raw!(self, Some("::".into()))?;
        emit!(self, selector.name);
        emit!(self, selector.arg);
    }

    #[emitter]
    pub fn emit_pseudo_element_selector_arg(
        &mut self,
        selector_arg: &PseudoElementSelectorArg<'_>,
    ) -> crate::Result {
        match selector_arg {
            PseudoElementSelectorArg::CompoundSelector(selector) => emit!(self, selector),
            PseudoElementSelectorArg::Ident(ident) => emit!(self, ident),
            PseudoElementSelectorArg::TokenSeq(token_seq) => emit!(self, token_seq),
        }
    }

    #[emitter]
    pub fn emit_token_seq(&mut self, token_seq: &TokenSeq<'_>) -> crate::Result {
        self.emit_list(token_seq.tokens[..].into(), FormatSep::COMMA)?;
    }

    #[emitter]
    pub fn emit_selector_type(&mut self, selector_type: &TypeSelector<'_>) -> crate::Result {
        match selector_type {
            TypeSelector::TagName(tag_name) => emit!(self, tag_name),
            TypeSelector::Universal(selector) => emit!(self, selector),
        }
    }

    #[emitter]
    pub fn emit_universal_selector(&mut self, selector: &UniversalSelector<'_>) -> crate::Result {
        write_raw!(self, Some("*".into()))?;
        emit!(self, selector.prefix);
    }

    #[emitter]
    pub fn emit_tag_name_selector(
        &mut self,
        tag_name_selector: &TagNameSelector<'_>,
    ) -> crate::Result {
        emit!(self, tag_name_selector.name);
    }

    #[emitter]
    pub fn emit_wq_name(&mut self, name: &WqName<'_>) -> crate::Result {
        emit!(self, name.prefix);
        emit!(self, name.name);
    }

    #[emitter]
    pub fn emit_ns_prefix(&mut self, ns_prefix: &NsPrefix<'_>) -> crate::Result {
        emit!(self, ns_prefix.kind);
    }

    #[emitter]
    pub fn emit_ns_prefix_kind(&mut self, ns_prefix_kind: &NsPrefixKind<'_>) -> crate::Result {
        match ns_prefix_kind {
            NsPrefixKind::Ident(ident) => emit!(self, ident),
            NsPrefixKind::Universal(universal) => emit!(self, universal),
        }
    }

    #[emitter]
    pub fn emit_ns_prefix_universal(&mut self, universal: &NsPrefixUniversal) -> crate::Result {
        write_raw!(self, Some("*".into()))?;
    }

    #[emitter]
    pub fn emit_pseudo_class(&mut self, pseudo: &PseudoClassSelector<'_>) -> crate::Result {
        emit!(self, pseudo.name);
        emit!(self, pseudo.arg);
    }

    #[emitter]
    pub fn emit_pseudo_class_selector_arg(
        &mut self,
        pseudo: &PseudoClassSelectorArg<'_>,
    ) -> crate::Result {
        match pseudo {
            PseudoClassSelectorArg::CompoundSelector(_) => todo!(),
            PseudoClassSelectorArg::CompoundSelectorList(_) => todo!(),
            PseudoClassSelectorArg::Ident(_) => todo!(),
            PseudoClassSelectorArg::LanguageRangeList(_) => todo!(),
            PseudoClassSelectorArg::Nth(_) => todo!(),
            PseudoClassSelectorArg::Number(_) => todo!(),
            PseudoClassSelectorArg::RelativeSelectorList(_) => todo!(),
            PseudoClassSelectorArg::SelectorList(_) => todo!(),
            PseudoClassSelectorArg::TokenSeq(_) => todo!(),
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
        write_raw!(self, translate!(self, SepRule::BlockLeft))?;
        self.emit_list(
            rule.statements[..].into(),
            FormatSep::SEMICOLON | FormatSep::MUTIPLE_LINE,
        )?;
        write_raw!(self, translate!(self, SepRule::BlockRight))?;
    }

    #[emitter]
    pub fn emit_declaration(&mut self, declar: &Declaration<'_>) -> crate::Result {
        emit!(self, declar.name);
        self.writer.write_raw(":".to_string())?;
        self.emit_list(
            declar.value[..].into(),
            FormatSep::SPACE | FormatSep::SINGLE_LINE,
        )?;
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
            ComponentValue::TokenWithSpan(token) => emit!(self, token),
            ComponentValue::UnicodeRange(_) => todo!(),
            ComponentValue::Url(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_token_with_span(&mut self, token_with_span: &TokenWithSpan<'_>) -> crate::Result {
        emit!(self, token_with_span.token);
    }

    #[emitter]
    pub fn emit_token(&mut self, token: &Token<'_>) -> crate::Result {
        match token {
            Token::Eof(_) => todo!(),
            Token::Ampersand(_) => todo!(),
            Token::Asterisk(_) => todo!(),
            Token::AsteriskEqual(_) => todo!(),
            Token::At(_) => todo!(),
            Token::AtKeyword(_) => todo!(),
            Token::AtLBraceVar(_) => todo!(),
            Token::BadStr(_) => todo!(),
            Token::Bar(_) => todo!(),
            Token::BarBar(_) => todo!(),
            Token::BarEqual(_) => todo!(),
            Token::CaretEqual(_) => todo!(),
            Token::Cdc(_) => todo!(),
            Token::Cdo(_) => todo!(),
            Token::Colon(_) => todo!(),
            Token::ColonColon(_) => todo!(),
            Token::Comma(comma) => emit!(self, comma),
            Token::Dedent(_) => todo!(),
            Token::Dimension(dimension) => emit!(self, dimension),
            Token::DollarEqual(_) => todo!(),
            Token::DollarVar(_) => todo!(),
            Token::Dot(_) => todo!(),
            Token::DotDotDot(_) => todo!(),
            Token::Equal(_) => todo!(),
            Token::EqualEqual(_) => todo!(),
            Token::Exclamation(_) => todo!(),
            Token::ExclamationEqual(_) => todo!(),
            Token::GreaterThan(_) => todo!(),
            Token::GreaterThanEqual(_) => todo!(),
            Token::Hash(hash) => emit!(self, hash),
            Token::HashLBrace(_) => todo!(),
            Token::Ident(ident) => emit!(self, ident),
            Token::Indent(_) => todo!(),
            Token::LBrace(_) => todo!(),
            Token::LBracket(_) => todo!(),
            Token::LessThan(_) => todo!(),
            Token::LessThanEqual(_) => todo!(),
            Token::Linebreak(_) => todo!(),
            Token::LParen(lparen) => emit!(self, lparen),
            Token::Minus(_) => todo!(),
            Token::Number(number) => emit!(self, number),
            Token::NumberSign(_) => todo!(),
            Token::Percent(_) => todo!(),
            Token::Percentage(_) => todo!(),
            Token::Plus(_) => todo!(),
            Token::PlusUnderscore(_) => todo!(),
            Token::Question(_) => todo!(),
            Token::RBrace(_) => todo!(),
            Token::RBracket(_) => todo!(),
            Token::RParen(rparen) => emit!(self, rparen),
            Token::Semicolon(_) => todo!(),
            Token::Solidus(_) => todo!(),
            Token::Str(str) => emit!(self, str),
            Token::StrTemplate(_) => todo!(),
            Token::Tilde(_) => todo!(),
            Token::TildeEqual(_) => todo!(),
            Token::UrlRaw(_) => todo!(),
            Token::UrlTemplate(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_token_lparen(&mut self, _lparen: &token::LParen) -> crate::Result {
        write_raw!(self, Some("(".to_string()))?;
    }
    #[emitter]
    pub fn emit_token_rparen(&mut self, _rparen: &token::RParen) -> crate::Result {
        write_raw!(self, Some(")".to_string()))?;
    }

    #[emitter]
    pub fn emit_token_dimension(&mut self, dimension: &token::Dimension<'_>) -> crate::Result {
        emit!(self, dimension.value);
        emit!(self, dimension.unit);
    }

    #[emitter]
    pub fn emit_hash(&mut self, hash: &Hash<'_>) -> crate::Result {
        write_raw!(self, Some(format!("#{}", hash.raw)))?;
    }

    #[emitter]
    pub fn emit_number(&mut self, number: &Number<'_>) -> crate::Result {
        write_raw!(self, Some(number.raw.to_string()))?;
    }

    #[emitter]
    pub fn emit_comma(&mut self, _comma: &Comma) -> crate::Result {
        write_raw!(self, Some(",".to_string()))?;
    }

    #[emitter]
    pub fn emit_ident(&mut self, ident: &token::Ident<'_>) -> crate::Result {
        write_raw!(self, Some(ident.raw.to_string()))?;
    }

    #[emitter]
    pub fn emit_dimension(&mut self, dimension: &Dimension<'_>) -> crate::Result {
        match dimension {
            Dimension::Length(len) => emit!(self, len),
            Dimension::Angle(_) => todo!(),
            Dimension::Duration(duration) => emit!(self, duration),
            Dimension::Frequency(_) => todo!(),
            Dimension::Resolution(_) => todo!(),
            Dimension::Flex(_) => todo!(),
            Dimension::Unknown(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_token_str(&mut self, str: &token::Str<'_>) -> crate::Result {
        write_raw!(self, Some(str.raw.to_string()))?;
    }

    #[emitter]
    pub fn emit_length(&mut self, length: &Length<'_>) -> crate::Result {
        self.writer.write_raw(String::from(length.value.raw))?;
        emit!(self, length.unit);
    }

    #[emitter]
    pub fn emit_duration(&mut self, duration: &Duration<'_>) -> crate::Result {
        self.writer.write_raw(duration.value.raw.to_string())?;
        self.writer.write_raw(duration.unit.raw.to_string())?;
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

trait EmitList<T> {
    fn emit_list<Elem>(&mut self, node: &[Elem], sep: T) -> crate::Result;
}

// impl<W, S> EmitList<SepRule> for CodeGenerator<W, S>
// where
//     W: Writer,
//     S: SepSerialize<SepRule> + SepSerialize<FormatSep>,
// {
//     fn emit_list<Elem>(&mut self, node: &[Elem], sep: SepRule) -> crate::Result {
//         todo!()
//     }
// }

// impl<W, S> EmitList<FormatSep> for CodeGenerator<W, S>
// where
//     W: Writer,
//     S: SepSerialize<SepRule> + SepSerialize<FormatSep>,
// {
//     fn emit_list<Elem>(&mut self, node: &[Elem], sep: FormatSep) -> crate::Result {
//         todo!()
//     }
// }
