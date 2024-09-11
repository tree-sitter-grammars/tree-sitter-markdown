//! This crate provides Markdown language support for the [tree-sitter][] parsing library.
//!
//! It contains two grammars: [`LANGUAGE`] to parse the block structure of markdown documents and
//! [`INLINE_LANGUAGE`] to parse inline content.
//!
//! It also supplies [`MarkdownParser`] as a convenience wrapper around the two grammars.
//! [`MarkdownParser::parse`] returns a [`MarkdownTree`] instread of a [`Tree`][Tree]. This struct
//! contains a block tree and an inline tree for each node in the block tree that has inline
//! content.
//!
//! [LanguageFn]: https://docs.rs/tree-sitter-language/*/tree_sitter_language/struct.LanguageFn.html
//! [Tree]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Tree.html
//! [tree-sitter]: https://tree-sitter.github.io/

#![cfg_attr(docsrs, feature(doc_cfg))]

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_markdown() -> *const ();
    fn tree_sitter_markdown_inline() -> *const ();
}

/// The tree-sitter [`LanguageFn`][LanguageFn] for the block grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_markdown) };

/// The tree-sitter [`LanguageFn`][LanguageFn] for the inline grammar.
pub const INLINE_LANGUAGE: LanguageFn =
    unsafe { LanguageFn::from_raw(tree_sitter_markdown_inline) };

/// The syntax highlighting queries for the block grammar.
pub const HIGHLIGHT_QUERY_BLOCK: &str =
    include_str!("../../tree-sitter-markdown/queries/highlights.scm");

/// The language injection queries for the block grammar.
pub const INJECTION_QUERY_BLOCK: &str =
    include_str!("../../tree-sitter-markdown/queries/injections.scm");

/// The syntax highlighting queries for the inline grammar.
pub const HIGHLIGHT_QUERY_INLINE: &str =
    include_str!("../../tree-sitter-markdown-inline/queries/highlights.scm");

/// The language injection queries for the inline grammar.
pub const INJECTION_QUERY_INLINE: &str =
    include_str!("../../tree-sitter-markdown-inline/queries/injections.scm");

/// The content of the [`node-types.json`][] file for the block grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_BLOCK: &str = include_str!("../../tree-sitter-markdown/src/node-types.json");

/// The content of the [`node-types.json`][] file for the inline grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_INLINE: &str =
    include_str!("../../tree-sitter-markdown-inline/src/node-types.json");

#[cfg(feature = "parser")]
#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
mod parser;

#[cfg(feature = "parser")]
#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
pub use parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_block_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading Markdown block grammar");
    }

    #[test]
    fn can_load_inline_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&INLINE_LANGUAGE.into())
            .expect("Error loading Markdown inline grammar");
    }
}
