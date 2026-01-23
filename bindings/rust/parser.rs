use core::num::NonZeroU16;
use std::collections::HashMap;

use tree_sitter::{
    InputEdit, Language, Node, ParseOptions, Parser, Point, Range, Tree, TreeCursor,
};

use crate::{INLINE_LANGUAGE, LANGUAGE};

/// A parser that produces [`MarkdownTree`]s.
///
/// This is a convenience wrapper around [`LANGUAGE`] and [`INLINE_LANGUAGE`].
pub struct MarkdownParser {
    parser: Parser,
    block_language: Language,
    inline_language: Language,
}

/// A stateful object for walking a [`MarkdownTree`] efficiently.
///
/// This exposes the same methods as [`TreeCursor`], but abstracts away the
/// double block / inline structure of [`MarkdownTree`].
#[derive(Clone)]
pub struct MarkdownCursor<'tree> {
    markdown_tree: &'tree MarkdownTree,
    block_cursor: TreeCursor<'tree>,
    inline_cursor: Option<TreeCursor<'tree>>,
}

impl<'tree> MarkdownCursor<'tree> {
    /// Get the cursor's current [`Node`].
    #[doc(alias = "ts_tree_cursor_current_node")]
    #[must_use]
    pub fn node(&self) -> Node<'tree> {
        match &self.inline_cursor {
            Some(cursor) => cursor.node(),
            None => self.block_cursor.node(),
        }
    }

    /// Returns `true` if the current node is from the [INLINE_LANGUAGE](inline language)
    ///
    /// This information is needed to handle "tree-sitter internal" data like
    /// [`field_id`](Self::field_id) correctly.
    #[must_use]
    pub fn is_inline(&self) -> bool {
        self.inline_cursor.is_some()
    }

    /// Get the numerical field id of this tree cursor’s current node.
    ///
    /// You will need to call [`is_inline`](Self::is_inline) to find out if the
    /// current node is an inline or block node.
    ///
    /// See also [`field_name`](Self::field_name).
    #[doc(alias = "ts_tree_cursor_current_field_id")]
    #[must_use]
    pub fn field_id(&self) -> Option<NonZeroU16> {
        match &self.inline_cursor {
            Some(cursor) => cursor.field_id(),
            None => self.block_cursor.field_id(),
        }
    }

    /// Get the field name of this tree cursor’s current node.
    ///
    /// You will need to call [`is_inline`](Self::is_inline) to find out if the
    /// current node is an inline or block node.
    #[doc(alias = "ts_tree_cursor_current_field_name")]
    #[must_use]
    pub fn field_name(&self) -> Option<&'static str> {
        match &self.inline_cursor {
            Some(cursor) => cursor.field_name(),
            None => self.block_cursor.field_name(),
        }
    }

    /// Get the depth of the cursor's current node.
    #[doc(alias = "ts_tree_cursor_current_depth")]
    #[must_use]
    pub fn depth(&self) -> u32 {
        self.block_cursor.depth()
            + self
                .inline_cursor
                .as_ref()
                .map(TreeCursor::depth)
                .unwrap_or(0)
    }

    fn move_to_inline_tree(&mut self) -> bool {
        let node = self.block_cursor.node();
        match node.kind() {
            "inline" | "pipe_table_cell" => {
                if let Some(inline_tree) = self.markdown_tree.inline_tree(&node) {
                    self.inline_cursor = Some(inline_tree.walk());
                    return true;
                }
            }
            _ => (),
        }
        false
    }

    fn move_to_block_tree(&mut self) {
        self.inline_cursor = None;
    }

    /// Move this cursor to the first child of its current node.
    ///
    /// This returns `true` if the cursor successfully moved, and returns `false` if there were no
    /// children.
    ///
    /// If the cursor is currently at a node in the block tree and it has an associated inline tree,
    /// it will descend into the inline tree.
    #[doc(alias = "ts_tree_cursor_goto_first_child")]
    pub fn goto_first_child(&mut self) -> bool {
        match &mut self.inline_cursor {
            Some(cursor) => cursor.goto_first_child(),
            None => {
                if self.move_to_inline_tree() {
                    if !self.inline_cursor.as_mut().unwrap().goto_first_child() {
                        self.move_to_block_tree();
                        false
                    } else {
                        true
                    }
                } else {
                    self.block_cursor.goto_first_child()
                }
            }
        }
    }

    /// Move this cursor to the last child of its current node.
    ///
    /// This returns `true` if the cursor successfully moved, and returns
    /// `false` if there were no children.
    ///
    /// Note that this function may be slower than
    /// [`goto_first_child`](MarkdownCursor::goto_first_child) because it needs to
    /// iterate through all the children to compute the child's position.
    #[doc(alias = "ts_tree_cursor_goto_last_child")]
    pub fn goto_last_child(&mut self) -> bool {
        match &mut self.inline_cursor {
            Some(cursor) => cursor.goto_last_child(),
            None => {
                if self.move_to_inline_tree() {
                    if !self.inline_cursor.as_mut().unwrap().goto_last_child() {
                        self.move_to_block_tree();
                        false
                    } else {
                        true
                    }
                } else {
                    self.block_cursor.goto_last_child()
                }
            }
        }
    }

    /// Move this cursor to the parent of its current node.
    ///
    /// This returns `true` if the cursor successfully moved, and returns `false` if there was no
    /// parent node (the cursor was already on the root node).
    ///
    /// If the cursor moves to the root node of an inline tree, then it ascents to the associated
    /// node in the block tree.
    #[doc(alias = "ts_tree_cursor_goto_parent")]
    pub fn goto_parent(&mut self) -> bool {
        match &mut self.inline_cursor {
            Some(inline_cursor) => {
                inline_cursor.goto_parent();
                if inline_cursor.node().parent().is_none() {
                    self.move_to_block_tree();
                }
                true
            }
            None => self.block_cursor.goto_parent(),
        }
    }

    /// Move this cursor to the next sibling of its current node.
    ///
    /// This returns `true` if the cursor successfully moved, and returns `false` if there was no
    /// next sibling node.
    #[doc(alias = "ts_tree_cursor_goto_next_sibling")]
    pub fn goto_next_sibling(&mut self) -> bool {
        match &mut self.inline_cursor {
            Some(inline_cursor) => inline_cursor.goto_next_sibling(),
            None => self.block_cursor.goto_next_sibling(),
        }
    }

    /// Move this cursor to the previous sibling of its current node.
    ///
    /// This returns `true` if the cursor successfully moved, and returns `false` if there was no
    /// previous sibling node.
    ///
    /// Note, that this function may be slower than
    /// [`goto_next_sibling`](MarkdownCursor::goto_next_sibling) due to how node positions are
    /// stored. In the worst case, this will need to iterate through all the children up to the
    /// previous sibling node to recalculate its position.
    #[doc(alias = "ts_tree_cursor_goto_previous_sibling")]
    pub fn goto_previous_sibling(&mut self) -> bool {
        match &mut self.inline_cursor {
            Some(inline_cursor) => inline_cursor.goto_previous_sibling(),
            None => self.block_cursor.goto_previous_sibling(),
        }
    }

    /// Move this cursor to the first child of its current node that extends beyond the given byte offset.
    ///
    /// This returns the index of the child node if one was found, and returns None if no such child was found.
    /// If the cursor is currently at a node in the block tree and it has an associated inline tree, it
    /// will descend into the inline tree.
    #[doc(alias = "ts_tree_cursor_goto_first_child_for_byte")]
    pub fn goto_first_child_for_byte(&mut self, index: usize) -> Option<usize> {
        match &mut self.inline_cursor {
            Some(cursor) => cursor.goto_first_child_for_byte(index),
            None => {
                if self.move_to_inline_tree() {
                    self.inline_cursor
                        .as_mut()
                        .unwrap()
                        .goto_first_child_for_byte(index)
                } else {
                    self.block_cursor.goto_first_child_for_byte(index)
                }
            }
        }
    }

    /// Move this cursor to the first child of its current node that extends beyond the given point.
    ///
    /// This returns the index of the child node if one was found, and returns None if no such child was found.
    /// If the cursor is currently at a node in the block tree and it has an associated inline tree, it
    /// will descend into the inline tree.
    #[doc(alias = "ts_tree_cursor_goto_first_child_for_point")]
    pub fn goto_first_child_for_point(&mut self, index: Point) -> Option<usize> {
        match &mut self.inline_cursor {
            Some(cursor) => cursor.goto_first_child_for_point(index),
            None => {
                if self.move_to_inline_tree() {
                    self.inline_cursor
                        .as_mut()
                        .unwrap()
                        .goto_first_child_for_point(index)
                } else {
                    self.block_cursor.goto_first_child_for_point(index)
                }
            }
        }
    }
}

/// An object that holds a combined markdown tree.
#[derive(Debug, Clone)]
pub struct MarkdownTree {
    block_tree: Tree,
    inline_trees: Vec<Tree>,
    inline_indices: HashMap<usize, usize>,
}

impl MarkdownTree {
    /// Edit the block tree and inline trees to keep them in sync with source code that has been
    /// edited.
    ///
    /// You must describe the edit both in terms of byte offsets and in terms of
    /// row/column coordinates.
    pub fn edit(&mut self, edit: &InputEdit) {
        self.block_tree.edit(edit);
        for inline_tree in self.inline_trees.iter_mut() {
            inline_tree.edit(edit);
        }
    }

    /// Returns the block tree for the parsed document
    pub fn block_tree(&self) -> &Tree {
        &self.block_tree
    }

    /// Returns the inline tree for the given inline node.
    ///
    /// Returns `None` if the given node does not have an associated inline tree. Either because
    /// the nodes type is not `inline` or because the inline content is empty.
    pub fn inline_tree(&self, parent: &Node) -> Option<&Tree> {
        let index = *self.inline_indices.get(&parent.id())?;
        Some(&self.inline_trees[index])
    }

    /// Returns the list of all inline trees
    pub fn inline_trees(&self) -> &[Tree] {
        &self.inline_trees
    }

    /// Create a new [`MarkdownCursor`] starting from the root of the tree.
    pub fn walk(&self) -> MarkdownCursor<'_> {
        MarkdownCursor {
            markdown_tree: self,
            block_cursor: self.block_tree.walk(),
            inline_cursor: None,
        }
    }
}

/// The options used while parsing a [`MarkdownTree`].
///
/// This abstracts away the double block / inline structure of [`MarkdownParser`].
#[derive(Default)]
pub struct MarkdownParseOptions<'a> {
    block_options: Option<ParseOptions<'a>>,
    inline_options: Option<ParseOptions<'a>>,
}

impl<'a> MarkdownParseOptions<'a> {
    /// Creates a new [MarkdownParseOptions] instance.
    pub fn new(
        block_options: Option<ParseOptions<'a>>,
        inline_options: Option<ParseOptions<'a>>,
    ) -> Self {
        MarkdownParseOptions {
            block_options,
            inline_options,
        }
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        let block_language = LANGUAGE.into();
        let inline_language = INLINE_LANGUAGE.into();
        let parser = Parser::new();
        MarkdownParser {
            parser,
            block_language,
            inline_language,
        }
    }
}

impl MarkdownParser {
    /// Parse a slice of UTF8 text.
    ///
    /// # Arguments:
    /// * `text` The UTF8-encoded text to parse.
    /// * `old_tree` A previous syntax tree parsed from the same document.
    ///   If the text of the document has changed since `old_tree` was
    ///   created, then you must edit `old_tree` to match the new text using
    ///   [MarkdownTree::edit].
    /// * `options` The [options][MarkdownParseOptions] used for parsing.
    ///   Use `MarkdownParseOptions::default()` if you don't need to pass any options.
    ///
    /// Returns a [MarkdownTree] if parsing succeeded, or `None` if:
    ///  * The timeout set with [tree_sitter::Parser::set_timeout_micros] expired
    ///  * The cancellation flag set with [tree_sitter::Parser::set_cancellation_flag] was flipped
    pub fn parse_with_options<T: AsRef<[u8]>, F: FnMut(usize, Point) -> T>(
        &mut self,
        callback: &mut F,
        old_tree: Option<&MarkdownTree>,
        mut options: MarkdownParseOptions<'_>,
    ) -> Option<MarkdownTree> {
        let MarkdownParser {
            parser,
            block_language,
            inline_language,
        } = self;
        parser
            .set_included_ranges(&[])
            .expect("Can not set included ranges to whole document");
        parser
            .set_language(block_language)
            .expect("Could not load block grammar");
        let block_tree = parser.parse_with_options(
            callback,
            old_tree.map(|tree| &tree.block_tree),
            options.block_options.as_mut().map(|b_opt| b_opt.reborrow()),
        )?;
        let (mut inline_trees, mut inline_indices) = if let Some(old_tree) = old_tree {
            let len = old_tree.inline_trees.len();
            (Vec::with_capacity(len), HashMap::with_capacity(len))
        } else {
            (Vec::new(), HashMap::new())
        };
        parser
            .set_language(inline_language)
            .expect("Could not load inline grammar");
        let mut tree_cursor = block_tree.walk();

        let mut i = 0;
        'outer: loop {
            let node = loop {
                let kind = tree_cursor.node().kind();
                if kind == "inline" || kind == "pipe_table_cell" || !tree_cursor.goto_first_child()
                {
                    while !tree_cursor.goto_next_sibling() {
                        if !tree_cursor.goto_parent() {
                            break 'outer;
                        }
                    }
                }
                let kind = tree_cursor.node().kind();
                if kind == "inline" || kind == "pipe_table_cell" {
                    break tree_cursor.node();
                }
            };
            let mut range = node.range();
            let mut ranges = Vec::new();
            if tree_cursor.goto_first_child() {
                while tree_cursor.goto_next_sibling() {
                    if !tree_cursor.node().is_named() {
                        continue;
                    }
                    let child_range = tree_cursor.node().range();
                    ranges.push(Range {
                        start_byte: range.start_byte,
                        start_point: range.start_point,
                        end_byte: child_range.start_byte,
                        end_point: child_range.start_point,
                    });
                    range.start_byte = child_range.end_byte;
                    range.start_point = child_range.end_point;
                }
                tree_cursor.goto_parent();
            }
            ranges.push(range);
            parser.set_included_ranges(&ranges).ok()?;
            let inline_tree = parser.parse_with_options(
                callback,
                old_tree.and_then(|old_tree| old_tree.inline_trees.get(i)),
                options
                    .inline_options
                    .as_mut()
                    .map(|b_opt| b_opt.reborrow()),
            )?;
            inline_trees.push(inline_tree);
            inline_indices.insert(node.id(), i);
            i += 1;
        }
        drop(tree_cursor);
        inline_trees.shrink_to_fit();
        inline_indices.shrink_to_fit();
        Some(MarkdownTree {
            block_tree,
            inline_trees,
            inline_indices,
        })
    }

    /// Parse a slice of UTF8 text.
    ///
    /// # Arguments:
    /// * `text` The UTF8-encoded text to parse.
    /// * `old_tree` A previous syntax tree parsed from the same document.
    ///   If the text of the document has changed since `old_tree` was
    ///   created, then you must edit `old_tree` to match the new text using
    ///   [MarkdownTree::edit].
    ///
    /// Returns a [MarkdownTree] if parsing succeeded, or `None` if:
    ///  * The timeout set with [tree_sitter::Parser::set_timeout_micros] expired
    ///  * The cancellation flag set with [tree_sitter::Parser::set_cancellation_flag] was flipped
    pub fn parse(&mut self, text: &[u8], old_tree: Option<&MarkdownTree>) -> Option<MarkdownTree> {
        self.parse_with_options(
            &mut |byte, _| &text[byte..],
            old_tree,
            MarkdownParseOptions::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use tree_sitter::{InputEdit, Point};

    use super::*;

    #[test]
    fn inline_ranges() {
        let code = "\
            # title\n\
            \n\
            Inline [content].\n\
        ";
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

        let code = "\
            # Title\n\
            \n\
            Inline [content].\n\
        ";
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
        let code = "\
            # title\n\
            \n\
            Inline [content].\n\
        ";
        let mut parser = MarkdownParser::default();
        let tree = parser.parse(code.as_bytes(), None).unwrap();
        let mut cursor = tree.walk();

        assert_eq!(cursor.node().kind(), "document");
        assert_eq!(cursor.depth(), 0);

        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "section");
        assert_eq!(cursor.depth(), 1);

        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "atx_heading");
        assert_eq!(cursor.depth(), 2);

        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), "paragraph");
        assert_eq!(cursor.depth(), 2);

        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "inline");
        assert_eq!(cursor.depth(), 3);

        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), "shortcut_link");
        assert_eq!(cursor.depth(), 4);

        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), ".");
        assert_eq!(cursor.depth(), 4);

        assert!(cursor.goto_parent());
        assert_eq!(cursor.depth(), 3);

        assert!(cursor.goto_parent());
        assert_eq!(cursor.depth(), 2);

        assert!(cursor.goto_parent());
        assert_eq!(cursor.depth(), 1);

        assert!(cursor.goto_parent());
        assert_eq!(cursor.node().kind(), "document");
        assert_eq!(cursor.depth(), 0);

        assert!(cursor.goto_last_child());
        assert_eq!(cursor.node().kind(), "section");
        assert_eq!(cursor.depth(), 1);

        assert!(cursor.goto_last_child());
        assert_eq!(cursor.node().kind(), "paragraph");
        assert_eq!(cursor.depth(), 2);

        assert!(cursor.goto_previous_sibling());
        assert_eq!(cursor.node().kind(), "atx_heading");
        assert_eq!(cursor.depth(), 2);

        assert!(cursor.goto_next_sibling());

        assert!(cursor.goto_last_child());
        assert_eq!(cursor.node().kind(), "inline");
        assert_eq!(cursor.depth(), 3);

        assert!(cursor.goto_last_child());
        assert_eq!(cursor.node().kind(), ".");
        assert_eq!(cursor.depth(), 4);

        assert!(cursor.goto_previous_sibling());
        assert_eq!(cursor.node().kind(), "shortcut_link");
        assert_eq!(cursor.depth(), 4);

        let mut cursor2 = cursor.clone();

        assert!(cursor2.goto_parent());
        assert_eq!(cursor2.depth(), 3);
        assert_eq!(cursor.depth(), 4);
    }

    #[test]
    fn table() {
        let code = "\
            | foo |\n\
            | --- |\n\
            | *bar*|\n\
        ";
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
