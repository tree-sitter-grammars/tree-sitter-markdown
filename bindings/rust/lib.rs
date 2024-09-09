//! This crate provides Markdown language support for the [tree-sitter][] parsing library.
//!
//! It contains two grammars: [`language`] to parse the block structure of markdown documents and
//! [`inline_language`] to parse inline content.
//!
//! It also supplies [`MarkdownParser`] as a convenience wrapper around the two grammars.
//! [`MarkdownParser::parse`] returns a [`MarkdownTree`] instread of a [`Tree`][Tree]. This struct
//! contains a block tree and an inline tree for each node in the block tree that has inline
//! content
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [Tree]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Tree.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_markdown() -> *const ();
    fn tree_sitter_markdown_inline() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for the block grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_markdown) };

/// The tree-sitter [`LanguageFn`] for the inline grammar.
pub const INLINE_LANGUAGE: LanguageFn =
    unsafe { LanguageFn::from_raw(tree_sitter_markdown_inline) };

pub const HIGHLIGHT_QUERY_BLOCK: &str =
    include_str!("../../tree-sitter-markdown/queries/highlights.scm");
pub const INJECTION_QUERY_BLOCK: &str =
    include_str!("../../tree-sitter-markdown/queries/injections.scm");
pub const HIGHLIGHT_QUERY_INLINE: &str =
    include_str!("../../tree-sitter-markdown-inline/queries/highlights.scm");
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
mod parser;

#[cfg(feature = "parser")]
pub use parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading Markdown language");
        let mut inline_parser = tree_sitter::Parser::new();
        inline_parser
            .set_language(&INLINE_LANGUAGE.into())
            .expect("Error loading Markdown language");
    }
}

#[cfg(test)]
#[cfg(feature = "parser")]
mod parser_tests {
    use tree_sitter::{InputEdit, Point};

    use super::parser::*;

    #[test]
    fn inline_ranges() {
        let code = "# title\n\nInline [content].\n";
        let mut parser = MarkdownParser::default();
        let mut tree = parser.parse(code.as_bytes(), None).unwrap();

        let section = tree.block_tree().root_node().child(0).unwrap();
        assert_eq!(section.kind(), "section");
        let heading = section.child(0).unwrap();
        assert_eq!(heading.kind(), "atx_heading");
        let paragraph = section.child(1).unwrap();
        assert_eq!(paragraph.kind(), "paragraph");
        let inline = paragraph.child(0).unwrap();
        assert_eq!(inline.kind(), "inline");
        assert_eq!(
            tree.inline_tree(&inline)
                .unwrap()
                .root_node()
                .child(0)
                .unwrap()
                .kind(),
            "shortcut_link"
        );

        let code = "# Title\n\nInline [content].\n";
        tree.edit(&InputEdit {
            start_byte: 2,
            old_end_byte: 3,
            new_end_byte: 3,
            start_position: Point { row: 0, column: 2 },
            old_end_position: Point { row: 0, column: 3 },
            new_end_position: Point { row: 0, column: 3 },
        });
        let tree = parser.parse(code.as_bytes(), Some(&tree)).unwrap();

        let section = tree.block_tree().root_node().child(0).unwrap();
        assert_eq!(section.kind(), "section");
        let heading = section.child(0).unwrap();
        assert_eq!(heading.kind(), "atx_heading");
        let paragraph = section.child(1).unwrap();
        assert_eq!(paragraph.kind(), "paragraph");
        let inline = paragraph.child(0).unwrap();
        assert_eq!(inline.kind(), "inline");
        assert_eq!(
            tree.inline_tree(&inline)
                .unwrap()
                .root_node()
                .named_child(0)
                .unwrap()
                .kind(),
            "shortcut_link"
        );
    }

    #[test]
    fn markdown_cursor() {
        let code = "# title\n\nInline [content].\n";
        let mut parser = MarkdownParser::default();
        let tree = parser.parse(code.as_bytes(), None).unwrap();
        let mut cursor = tree.walk();
        assert_eq!(cursor.node().kind(), "document");
        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "section");
        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "atx_heading");
        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), "paragraph");
        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "inline");
        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "shortcut_link");
        assert!(cursor.goto_parent());
        assert!(cursor.goto_parent());
        assert!(cursor.goto_parent());
        assert!(cursor.goto_parent());
        assert_eq!(cursor.node().kind(), "document");
    }

    #[test]
    fn table() {
        let code = "| foo |\n| --- |\n| *bar*|\n";
        let mut parser = MarkdownParser::default();
        let tree = parser.parse(code.as_bytes(), None).unwrap();
        dbg!(&tree.inline_trees());
        let mut cursor = tree.walk();

        assert_eq!(cursor.node().kind(), "document");
        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "section");
        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "pipe_table");
        assert!(cursor.goto_first_child());
        assert!(cursor.goto_next_sibling());
        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), "pipe_table_row");
        assert!(cursor.goto_first_child());
        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), "pipe_table_cell");
        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "emphasis");
    }
}
