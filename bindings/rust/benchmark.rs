use tree_sitter::{InputEdit, Point};
use tree_sitter_md::{MarkdownParser, MarkdownTree};

fn main() {
    let mut parser = MarkdownParser::default();
    let filename = std::env::args().nth(1).unwrap_or("README.md".to_string());
    let source = std::fs::read(filename).unwrap();
    let mut tree = parser.parse(&source, None).unwrap();
    tree.edit(&InputEdit {
        start_byte: 0,
        old_end_byte: 1,
        new_end_byte: 0,
        start_position: Point::new(0, 0),
        old_end_position: Point::new(0, 1),
        new_end_position: Point::new(0, 0),
    });
    reparse(&mut parser, &source[1..], tree);
}

fn reparse(parser: &mut MarkdownParser, source: &[u8], old_tree: MarkdownTree) {
    parser.parse(source, Some(&old_tree)).unwrap();
}
