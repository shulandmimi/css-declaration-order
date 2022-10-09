#![deny(clippy::all)]
#![allow(clippy::needless_update)]
pub use std::fmt::Result;

use codegen_macro::emitter;
use raffia::{
    ast::{
        self, AnPlusB, AtRule, AtRulePrelude, ClassSelector, ComplexSelector, ComplexSelectorChild,
        ComponentValue, CompoundSelector, Declaration, Dimension, Duration, IdSelector,
        InterpolableIdent, InterpolableStr, Length, MediaCondition, MediaConditionKind,
        MediaFeature, MediaFeatureBoolean, MediaFeatureName, MediaFeaturePlain, MediaInParens,
        MediaQuery, MediaQueryList, NsPrefix, NsPrefixKind, NsPrefixUniversal, PseudoClassSelector,
        PseudoClassSelectorArg, PseudoElementSelector, PseudoElementSelectorArg, QualifiedRule,
        SelectorList, SimpleBlock, SimpleSelector, Statement, Str, Stylesheet, SupportsInParens,
        TagNameSelector, TokenSeq, TypeSelector, UniversalSelector, WqName, Flex,
    },
    token::{self, Comma, Hash, Token, TokenWithSpan},
};

mod emit;
mod sep;
mod types;
mod writer;

pub use crate::sep::SepRule;
pub use emit::Emit;
pub use sep::{FormatSep, SepSerialize};
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
        self.emit_list(node.statements[..].into(), FormatSep::NONE)?;
    }

    #[emitter]
    pub fn emit_statements(&mut self, node: &Statement<'_>) -> crate::Result {
        match node {
            Statement::AtRule(rule) => emit!(self, rule),
            Statement::Declaration(declaration) => emit!(self, declaration),
            Statement::KeyframeBlock(keyframe_block) => emit!(self, keyframe_block),
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
        write_str!(self, "@")?;
        emit!(self, at_rule.name);
        emit!(self, at_rule.prelude);

        match at_rule.name.raw {
            "media" | "supports" if at_rule.block.is_some() => {
                self.emit_simple_block_with_sep(at_rule.block.as_ref().unwrap(), FormatSep::NONE)?
            }
            _ => emit!(self, at_rule.block),
        }
        if at_rule.block.is_none() {
            write_str!(self, ";")?;
        }
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
            AtRulePrelude::Charset(v) => {
                space!(self);
                emit!(self, v)
            }
            AtRulePrelude::ColorProfile(_) => todo!(),
            AtRulePrelude::Container(_) => todo!(),
            AtRulePrelude::CounterStyle(_) => todo!(),
            AtRulePrelude::CustomMedia(_) => todo!(),
            AtRulePrelude::Document(_) => todo!(),
            AtRulePrelude::FontFeatureValues(_) => todo!(),
            AtRulePrelude::FontPaletteValues(_) => todo!(),
            AtRulePrelude::Import(_) => todo!(),
            AtRulePrelude::Keyframes(keyframes) => {
                space!(self);
                emit!(self, keyframes)
            }
            AtRulePrelude::Layer(_) => todo!(),
            AtRulePrelude::Media(media) => {
                space!(self);
                emit!(self, media)
            }
            AtRulePrelude::Namespace(_) => todo!(),
            AtRulePrelude::Nest(_) => todo!(),
            AtRulePrelude::Page(_) => todo!(),
            AtRulePrelude::PositionFallback(_) => todo!(),
            AtRulePrelude::Property(_) => todo!(),
            AtRulePrelude::ScrollTimeline(_) => todo!(),
            AtRulePrelude::Supports(support) => {
                space!(self);
                emit!(self, support)
            }
            AtRulePrelude::Unknown(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_ast_supports_condition(
        &mut self,
        support: &ast::SupportsCondition<'_>,
    ) -> crate::Result {
        write_str!(self, "(")?;
        self.emit_list(support.conditions[..].into(), FormatSep::NONE)?;
        write_str!(self, ")")?;
    }

    #[emitter]
    pub fn emit_ast_supports_condition_kind(
        &mut self,
        support_kind: &ast::SupportsConditionKind<'_>,
    ) -> crate::Result {
        match support_kind {
            ast::SupportsConditionKind::Not(not) => emit!(self, not),
            ast::SupportsConditionKind::And(and) => emit!(self, and),
            ast::SupportsConditionKind::Or(or) => emit!(self, or),
            ast::SupportsConditionKind::SupportsInParens(parens) => emit!(self, parens),
        }
    }

    #[emitter]
    pub fn emit_ast_supports_condition_not(&mut self, not: &ast::SupportsNot<'_>) -> crate::Result {
        emit!(self, not.keyword);
        emit!(self, not.condition);
    }

    #[emitter]
    pub fn emit_ast_supports_condition_and(&mut self, and: &ast::SupportsAnd<'_>) -> crate::Result {
        emit!(self, and.keyword);
        emit!(self, and.condition);
    }

    #[emitter]
    pub fn emit_ast_supports_condition_or(&mut self, or: &ast::SupportsOr<'_>) -> crate::Result {
        emit!(self, or.keyword);
        emit!(self, or.condition);
    }

    #[emitter]
    pub fn emit_ast_supports_condition_support_in_parens(
        &mut self,
        parens: &SupportsInParens<'_>,
    ) -> crate::Result {
        match parens {
            SupportsInParens::SupportsCondition(condition) => emit!(self, condition),
            SupportsInParens::Feature(feature) => emit!(self, feature),
        }
    }

    #[emitter]
    pub fn emit_ast_supports_decl(&mut self, decl: &ast::SupportsDecl<'_>) -> crate::Result {
        emit!(self, decl.decl);
    }

    #[emitter]
    pub fn emit_ast_keyframes_name(&mut self, keyframes: &ast::KeyframesName<'_>) -> crate::Result {
        match keyframes {
            ast::KeyframesName::Ident(ident) => emit!(self, ident),
            ast::KeyframesName::Str(str) => emit!(self, str),
        }
    }

    #[emitter]
    pub fn emit_ast_keyframes_block(
        &mut self,
        keyframes: &ast::KeyframeBlock<'_>,
    ) -> crate::Result {
        self.emit_list(keyframes.selectors[..].into(), FormatSep::COMMA)?;
        self.emit_simple_block_with_sep(&keyframes.block, FormatSep::SEMICOLON)?;
    }

    #[emitter]
    pub fn emit_ast_keyframes_selector(
        &mut self,
        selector: &ast::KeyframeSelector<'_>,
    ) -> crate::Result {
        match selector {
            ast::KeyframeSelector::Ident(ident) => emit!(self, ident),
            ast::KeyframeSelector::Percentage(percentage) => emit!(self, percentage),
        }
    }

    /// `query`
    ///
    /// ```css
    /// /** (max-width: 1199.98px) */
    /// @media (max-width: 1199.98px) {}
    /// ```
    #[emitter]
    pub fn emit_media_query_list(
        &mut self,
        media_query_list: &MediaQueryList<'_>,
    ) -> crate::Result {
        self.emit_list(media_query_list.queries[..].into(), FormatSep::COMMA)?
    }

    #[emitter]
    pub fn emit_media_query(&mut self, media_query: &MediaQuery<'_>) -> crate::Result {
        match media_query {
            MediaQuery::ConditionOnly(condition_only) => emit!(self, condition_only),
            MediaQuery::WithType(with_type) => emit!(self, with_type),
        }
    }

    #[emitter]
    pub fn emit_ast_with_type(&mut self, with_type: &ast::MediaQueryWithType<'_>) -> crate::Result {
        emit!(self, with_type.modifier);
        if with_type.modifier.is_some() {
            space!(self);
        }
        emit!(self, with_type.media_type);
        if with_type.condition.is_some() {
            space!(self);
            write_str!(self, "and")?;
            space!(self);
        }
        emit!(self, with_type.condition);
    }

    #[emitter]
    pub fn emit_media_condition(&mut self, media_condition: &MediaCondition<'_>) -> crate::Result {
        self.emit_list(media_condition.conditions[..].into(), FormatSep::NONE)?;
    }

    #[emitter]
    pub fn emit_media_condition_kind(
        &mut self,
        media_condition_kind: &MediaConditionKind<'_>,
    ) -> crate::Result {
        match media_condition_kind {
            MediaConditionKind::MediaInParens(media_in_parens) => {
                emit!(self, media_in_parens)
            }
            MediaConditionKind::And(and) => emit!(self, and),
            MediaConditionKind::Or(or) => emit!(self, or),
            MediaConditionKind::Not(not) => emit!(self, not),
        }
    }

    #[emitter]
    pub fn emit_media_in_parens(&mut self, media_in_parens: &MediaInParens<'_>) -> crate::Result {
        write_str!(self, "(")?;
        match media_in_parens {
            MediaInParens::MediaCondition(condition) => {
                emit!(self, condition);
            }
            MediaInParens::MediaFeature(feature) => {
                emit!(self, feature);
            }
        }
        write_str!(self, ")")?;
    }

    #[emitter]
    pub fn emit_media_and(&mut self, and: &ast::MediaAnd<'_>) -> crate::Result {
        emit!(self, and.keyword);
        space!(self);
        emit!(self, and.media_in_parens);
    }

    #[emitter]
    pub fn emit_media_or(&mut self, or: &ast::MediaOr<'_>) -> crate::Result {
        emit!(self, or.keyword);
        space!(self);
        emit!(self, or.media_in_parens);
    }

    #[emitter]
    pub fn emit_media_not(&mut self, not: &ast::MediaNot<'_>) -> crate::Result {
        emit!(self, not.keyword);
        space!(self);
        emit!(self, not.media_in_parens);
    }

    #[emitter]
    pub fn emit_media_feature(&mut self, media_feature: &MediaFeature<'_>) -> crate::Result {
        match media_feature {
            MediaFeature::Plain(plain) => emit!(self, plain),
            MediaFeature::Boolean(bool) => emit!(self, bool),
            MediaFeature::Range(range) => emit!(self, range),
            MediaFeature::RangeInterval(range_interval) => emit!(self, range_interval),
        }
    }

    #[emitter]
    pub fn emit_media_feature_range_interval(
        &mut self,
        range_interval: &ast::MediaFeatureRangeInterval<'_>,
    ) -> crate::Result {
        emit!(self, range_interval.left);
        emit!(self, range_interval.left_comparison);
        emit!(self, range_interval.name);
        emit!(self, range_interval.right_comparison);
        emit!(self, range_interval.right);
    }

    #[emitter]
    pub fn emit_media_feature_boolean(&mut self, bool: &MediaFeatureBoolean<'_>) -> crate::Result {
        emit!(self, bool.name);
    }

    #[emitter]
    pub fn emit_media_feature_name(&mut self, name: &MediaFeatureName<'_>) -> crate::Result {
        match name {
            MediaFeatureName::Ident(ident) => emit!(self, ident),
        }
    }

    #[emitter]
    pub fn emit_ast_range(&mut self, range: &ast::MediaFeatureRange<'_>) -> crate::Result {
        emit!(self, range.left);
        emit!(self, range.comparison);
        emit!(self, range.right);
    }

    #[emitter]
    pub fn emit_ast_comparison(
        &mut self,
        comparison: &ast::MediaFeatureComparison,
    ) -> crate::Result {
        match comparison.kind {
            ast::MediaFeatureComparisonKind::LessThan => write_str!(self, "<")?,
            ast::MediaFeatureComparisonKind::LessThanOrEqual => write_str!(self, "<=")?,
            ast::MediaFeatureComparisonKind::GreaterThan => write_str!(self, ">")?,
            ast::MediaFeatureComparisonKind::GreaterThanOrEqual => write_str!(self, ">=")?,
            ast::MediaFeatureComparisonKind::Equal => write_str!(self, "=")?,
        }
    }

    #[emitter]
    pub fn emit_media_feature_plain(
        &mut self,
        media_feature_plain: &MediaFeaturePlain<'_>,
    ) -> crate::Result {
        emit!(self, media_feature_plain.name);
        write_raw!(self, Some(":".into()))?;
        emit!(self, media_feature_plain.value);
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

    fn emit_list_with_option<Elem, F>(&mut self, nodes: &[Elem], sep: F) -> crate::Result
    where
        Self: Emit<Elem>,
        F: Fn(Option<&Elem>) -> FormatSep,
    {
        let end = nodes.len();
        new_line!(self, sep(None));
        for (idx, node) in nodes.iter().enumerate() {
            emit!(self, node);

            if idx != end - 1 {
                write_raw!(self, translate!(self, sep(Some(node))))?;
            } else {
                write_raw!(self, write_last!(self, sep(Some(node))))?;
            }

            new_line!(self, sep(None))
        }

        Ok(())
    }

    #[emitter]
    pub fn emit_complex_selector(&mut self, selector: &ComplexSelector<'_>) -> crate::Result {
        self.emit_list(selector.children[..].into(), FormatSep::NONE)?;
    }

    #[emitter]
    pub fn emit_computed_selector_child(
        &mut self,
        selector: &ComplexSelectorChild<'_>,
    ) -> crate::Result {
        match selector {
            ComplexSelectorChild::CompoundSelector(selector) => {
                emit!(self, selector);
            }
            ComplexSelectorChild::Combinator(combinator) => emit!(self, combinator),
        }
    }

    #[emitter]
    pub fn emit_ast_combinator(&mut self, combinator: &ast::Combinator) -> crate::Result {
        emit!(self, combinator.kind);
    }

    #[emitter]
    pub fn emit_ast_combinator_kind(
        &mut self,
        combinator_kind: &ast::CombinatorKind,
    ) -> crate::Result {
        match combinator_kind {
            ast::CombinatorKind::Descendant => write_str!(self, " ")?,
            ast::CombinatorKind::NextSibling => write_str!(self, "+")?,
            ast::CombinatorKind::Child => write_str!(self, ">")?,
            ast::CombinatorKind::LaterSibling => write_str!(self, "~")?,
            ast::CombinatorKind::Column => write_str!(self, "||")?,
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
            SimpleSelector::Attribute(attribute) => emit!(self, attribute),
            SimpleSelector::PseudoClass(pseudo) => emit!(self, pseudo),
            SimpleSelector::PseudoElement(element) => emit!(self, element),
            SimpleSelector::Nesting(_) => todo!(),
            SimpleSelector::SassPlaceholder(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_attribute(
        &mut self,
        attribute_selector: &ast::AttributeSelector<'_>,
    ) -> crate::Result {
        write_str!(self, "[")?;
        emit!(self, attribute_selector.name);
        write_str!(self, "]")?;
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
    pub fn emit_ns_prefix_universal(&mut self, _universal: &NsPrefixUniversal) -> crate::Result {
        write_raw!(self, Some("*".into()))?;
    }

    #[emitter]
    pub fn emit_pseudo_class(&mut self, pseudo: &PseudoClassSelector<'_>) -> crate::Result {
        write_str!(self, ":")?;
        emit!(self, pseudo.name);
        emit!(self, pseudo.arg);
    }

    #[emitter]
    pub fn emit_pseudo_class_selector_arg(
        &mut self,
        pseudo: &PseudoClassSelectorArg<'_>,
    ) -> crate::Result {
        write_str!(self, "(")?;
        match pseudo {
            PseudoClassSelectorArg::CompoundSelector(_) => todo!(),
            PseudoClassSelectorArg::CompoundSelectorList(_) => todo!(),
            PseudoClassSelectorArg::Ident(_) => todo!(),
            PseudoClassSelectorArg::LanguageRangeList(_) => todo!(),
            PseudoClassSelectorArg::Nth(nth) => emit!(self, nth),
            PseudoClassSelectorArg::Number(_) => todo!(),
            PseudoClassSelectorArg::RelativeSelectorList(_) => todo!(),
            PseudoClassSelectorArg::SelectorList(selecto_list) => emit!(self, selecto_list),
            PseudoClassSelectorArg::TokenSeq(_) => todo!(),
        }
        write_str!(self, ")")?;
    }

    #[emitter]
    pub fn emit_pseudo_class_selector_arg_nth(&mut self, nth: &ast::Nth<'_>) -> crate::Result {
        match nth {
            ast::Nth::Odd(odd) => emit!(self, odd),
            ast::Nth::Even(even) => emit!(self, even),
            ast::Nth::Integer(int) => emit!(self, int),
            ast::Nth::AnPlusB(plus) => emit!(self, plus),
        }
    }

    #[emitter]
    pub fn emit_ast_an_plus(&mut self, an_plus: &AnPlusB) -> crate::Result {
        write_str!(self, an_plus.a.to_string())?;
        if an_plus.b != 0 {
            if an_plus.b > 0 {
                write_str!(self, '+')?;
            }
            write_str!(self, an_plus.b.to_string())?;
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
        self.emit_simple_block_with_sep(rule, FormatSep::SEMICOLON)?;
    }

    pub fn emit_simple_block_with_sep(
        &mut self,
        rule: &SimpleBlock<'_>,
        sep: FormatSep,
    ) -> crate::Result {
        write_raw!(self, translate!(self, SepRule::BlockLeft))?;
        self.emit_list_with_option(rule.statements[..].into(), move |elem| match elem {
            Some(e) => match e {
                Statement::KeyframeBlock(_) => FormatSep::NONE,
                _ => sep,
            },
            _ => sep,
        })?;
        write_raw!(self, translate!(self, SepRule::BlockRight))?;
        Ok(())
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
            ComponentValue::Calc(calc) => emit!(self, calc),
            ComponentValue::Delimiter(delimiter) => emit!(self, delimiter),
            ComponentValue::Dimension(dimension) => emit!(self, dimension),
            ComponentValue::Function(fun) => emit!(self, fun),
            ComponentValue::HexColor(color) => emit!(self, color),
            ComponentValue::IdSelector(_) => todo!(),
            ComponentValue::InterpolableIdent(ident) => emit!(self, ident),
            ComponentValue::InterpolableStr(interpolable_str) => emit!(self, interpolable_str),
            ComponentValue::LayerName(_) => todo!(),
            ComponentValue::LessVariable(_) => todo!(),
            ComponentValue::LessVariableVariable(_) => todo!(),
            ComponentValue::Number(number) => emit!(self, number),
            ComponentValue::Percentage(percentage) => emit!(self, percentage),
            ComponentValue::Ratio(ratio) => emit!(self, ratio),
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
            ComponentValue::Url(url) => emit!(self, url),
        }
    }

    #[emitter]
    pub fn emit_ast_ratio(&mut self, ratio: &ast::Ratio<'_>) -> crate::Result {
        emit!(self, ratio.numerator);
        write_str!(self, "/")?;
        emit!(self, ratio.denominator);
    }

    #[emitter]
    pub fn emit_ast_url(&mut self, url: &ast::Url<'_>) -> crate::Result {
        emit!(self, url.name);
        emit!(self, url.value);
        self.emit_list(url.modifiers[..].into(), FormatSep::NONE)?;
    }

    #[emitter]
    pub fn emit_ast_url_value(&mut self, url_value: &ast::UrlValue<'_>) -> crate::Result {
        write_str!(self, "(")?;
        match url_value {
            ast::UrlValue::Raw(raw) => emit!(self, raw),
            ast::UrlValue::SassInterpolated(_) => todo!(),
            ast::UrlValue::Str(str) => emit!(self, str),
        }
        write_str!(self, ")")?;
    }

    #[emitter]
    pub fn emit_ast_url_value_raw(&mut self, url_raw: &ast::UrlRaw<'_>) -> crate::Result {
        write_str!(self, url_raw.raw)?;
    }

    #[emitter]
    pub fn emit_ast_url_modifier(&mut self, _url_modifier: &ast::UrlModifier<'_>) -> crate::Result {
    }

    #[emitter]
    pub fn emit_interpolable_str(&mut self, str: &InterpolableStr<'_>) -> crate::Result {
        match str {
            InterpolableStr::Literal(literal) => emit!(self, literal),
            InterpolableStr::SassInterpolated(_) => todo!(),
            InterpolableStr::LessInterpolated(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_hex_color(&mut self, color: &ast::HexColor<'_>) -> crate::Result {
        write_str!(self, "#")?;
        write_str!(self, color.raw)?;
    }

    #[emitter]
    pub fn emit_ast_calc(&mut self, calc: &ast::Calc<'_>) -> crate::Result {
        emit!(self, calc.left);
        emit!(self, calc.op);
        emit!(self, calc.right);
    }

    #[emitter]
    pub fn emit_ast_calc_operation(&mut self, operation: &ast::CalcOperator) -> crate::Result {
        emit!(self, operation.kind);
    }

    #[emitter]
    pub fn emit_ast_calc_operation_kind(
        &mut self,
        operation: &ast::CalcOperatorKind,
    ) -> crate::Result {
        match operation {
            ast::CalcOperatorKind::Plus => write_str!(self, "+")?,
            ast::CalcOperatorKind::Minus => write_str!(self, "-")?,
            ast::CalcOperatorKind::Multiply => write_str!(self, "*")?,
            ast::CalcOperatorKind::Division => write_str!(self, "/")?,
        };
    }

    #[emitter]
    pub fn emit_ast_delimiter(&mut self, delimiter: &ast::Delimiter) -> crate::Result {
        emit!(self, delimiter.kind);
    }

    #[emitter]
    pub fn emit_ast_delimiter_kind(
        &mut self,
        delimiter_kind: &ast::DelimiterKind,
    ) -> crate::Result {
        match delimiter_kind {
            ast::DelimiterKind::Comma => write_str!(self, ",")?,
            ast::DelimiterKind::Solidus => write_str!(self, "/")?,
            ast::DelimiterKind::Semicolon => write_str!(self, ";")?,
        };
    }

    #[emitter]
    pub fn emit_ast_percentage(&mut self, percentage: &ast::Percentage<'_>) -> crate::Result {
        emit!(self, percentage.value);
        write_str!(self, "%")?;
    }

    #[emitter]
    pub fn emit_function(&mut self, fun: &ast::Function<'_>) -> crate::Result {
        emit!(self, fun.name);
        write_str!(self, "(")?;
        self.emit_list(fun.args[..].into(), FormatSep::NONE)?;
        write_str!(self, ")")?;
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
            Token::Percentage(percentage) => emit!(self, percentage),
            Token::Plus(plus) => emit!(self, plus),
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
    pub fn emit_token_plus(&mut self, _plus: &token::Plus) -> crate::Result {
        write_str!(self, "+")?;
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
    pub fn emit_token_percentage(&mut self, _percentage: &token::Percentage<'_>) -> crate::Result {
        write_str!(self, "%")?;
    }

    #[emitter]
    pub fn emit_token_dimension(&mut self, dimension: &token::Dimension<'_>) -> crate::Result {
        emit!(self, dimension.value);
        emit!(self, dimension.unit);
    }

    /// `#abcdef`
    #[emitter]
    pub fn emit_hash(&mut self, hash: &Hash<'_>) -> crate::Result {
        write_raw!(self, Some(format!("#{}", hash.raw)))?;
    }

    /// `666`
    #[emitter]
    pub fn emit_token_number(&mut self, number: &token::Number<'_>) -> crate::Result {
        write_raw!(self, Some(number.raw.to_string()))?;
    }

    #[emitter]
    pub fn emit_ast_number(&mut self, number: &ast::Number<'_>) -> crate::Result {
        write_str!(self, number.raw)?;
    }

    /// `,`
    #[emitter]
    pub fn emit_comma(&mut self, _comma: &Comma) -> crate::Result {
        write_raw!(self, Some(",".to_string()))?;
    }

    #[emitter]
    pub fn emit_ident(&mut self, ident: &token::Ident<'_>) -> crate::Result {
        write_str!(self, ident.raw)?;
    }

    #[emitter]
    pub fn emit_ast_ident(&mut self, ident: &ast::Ident<'_>) -> crate::Result {
        write_str!(self, ident.raw)?;
    }

    #[emitter]
    pub fn emit_dimension(&mut self, dimension: &Dimension<'_>) -> crate::Result {
        match dimension {
            Dimension::Length(len) => emit!(self, len),
            Dimension::Angle(angle) => emit!(self, angle),
            Dimension::Duration(duration) => emit!(self, duration),
            Dimension::Frequency(_) => todo!(),
            Dimension::Resolution(resolution) => emit!(self, resolution),
            Dimension::Flex(flex) => emit!(self, flex),
            Dimension::Unknown(_) => todo!(),
        }
    }

    #[emitter]
    pub fn emit_ast_flex(&mut self, flex: &Flex<'_>) -> crate::Result {
        emit!(self, flex.value);
        emit!(self, flex.unit);
    }

    #[emitter]
    pub fn emit_resolution(&mut self, resolution: &ast::Resolution<'_>) -> crate::Result {
        emit!(self, resolution.value);
        emit!(self, resolution.unit);
    }

    #[emitter]
    pub fn emit_ast_angle(&mut self, angle: &ast::Angle<'_>) -> crate::Result {
        emit!(self, angle.value);
        emit!(self, angle.unit);
    }

    #[emitter]
    pub fn emit_token_str(&mut self, str: &token::Str<'_>) -> crate::Result {
        write_raw!(self, Some(str.raw.to_string()))?;
    }

    /// `1px`
    #[emitter]
    pub fn emit_length(&mut self, length: &Length<'_>) -> crate::Result {
        self.writer.write_raw(String::from(length.value.raw))?;
        emit!(self, length.unit);
    }

    /// `1s`
    #[emitter]
    pub fn emit_duration(&mut self, duration: &Duration<'_>) -> crate::Result {
        self.writer.write_raw(duration.value.raw.to_string())?;
        self.writer.write_raw(duration.unit.raw.to_string())?;
    }

    #[emitter]
    pub fn emit_interpolable_ident(&mut self, ident: &InterpolableIdent<'_>) -> crate::Result {
        match ident {
            InterpolableIdent::Literal(literal) => emit!(self, literal),
            InterpolableIdent::SassInterpolated(_) => todo!(),
            InterpolableIdent::LessInterpolated(_) => todo!(),
        }
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
