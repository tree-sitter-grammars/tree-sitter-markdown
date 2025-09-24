from dataclasses import dataclass
from typing import Callable

from tree_sitter import Language, Node, Parser, Tree, TreeCursor

from . import inline_language, language


def is_inline_type(type: str) -> bool:
    return type in ["inline", "pipe_table_cell"]


def walk(cursor: TreeCursor, callback: Callable[[Node], None]):
    if cursor.node is not None:
        callback(cursor.node)
    if not cursor.goto_first_child():
        return
    walk(cursor, callback)
    while cursor.goto_next_sibling():
        walk(cursor, callback)
    cursor.goto_parent()


@dataclass
class MarkdownTree:
    block_tree: Tree
    inline_trees: dict[int, Tree]

    @property
    def root_node(self):
        return self.block_tree.root_node

    def edit(self, *args, **kwargs):
        self.block_tree.edit(*args, **kwargs)
        for _, inline_tree in self.inline_trees.items():
            inline_tree.edit(*args, **kwargs)

    def walk(self) -> "MarkdownCursor":
        return MarkdownCursor(self, self.block_tree.walk(), None)

    def inline_tree(self, parent: Node) -> Tree | None:
        return self.inline_trees.get(parent.id, None)


@dataclass
class MarkdownCursor:
    tree: MarkdownTree
    block_cursor: TreeCursor
    inline_cursor: TreeCursor | None

    @property
    def node(self) -> Node | None:
        if self.inline_cursor is not None:
            return self.inline_cursor.node
        else:
            return self.block_cursor.node

    @property
    def is_inline(self) -> bool:
        return self.inline_cursor is not None

    @property
    def field_id(self) -> int | None:
        if self.inline_cursor is not None:
            return self.inline_cursor.field_id
        else:
            return self.block_cursor.field_id

    @property
    def field_name(self) -> str | None:
        if self.inline_cursor is not None:
            return self.inline_cursor.field_name
        else:
            return self.block_cursor.field_name

    def move_to_inline_tree(self) -> bool:
        if self.inline_cursor is not None:
            return False
        node = self.block_cursor.node
        if node is None or not is_inline_type(node.type):
            return False
        else:
            inline_tree = self.tree.inline_tree(node)
            assert inline_tree is not None
            self.inline_cursor = inline_tree.walk()
            return True

    def move_to_block_tree(self) -> bool:
        if self.inline_cursor is None:
            return False
        self.inline_cursor = None
        return True

    def goto_first_child(self) -> bool:
        if self.inline_cursor is not None:
            return self.inline_cursor.goto_first_child()
        if self.move_to_inline_tree():
            if not self.inline_cursor.goto_first_child():  # type: ignore
                self.move_to_block_tree()
                return False
            else:
                return True
        else:
            return self.block_cursor.goto_first_child()

    def goto_parent(self) -> bool:
        if self.inline_cursor is not None:
            self.inline_cursor.goto_parent()
            if self.inline_cursor.node is None or self.inline_cursor.node.parent is None:
                self.move_to_block_tree()
            return True
        else:
            return self.block_cursor.goto_parent()

    def goto_next_sibling(self) -> bool:
        if self.inline_cursor is not None:
            return self.inline_cursor.goto_next_sibling()
        else:
            return self.block_cursor.goto_next_sibling()


class MarkdownParser:
    block_parser: Parser
    inline_parser: Parser

    def __init__(self, *args, **kwargs) -> None:
        self.block_parser = Parser(Language(language()), *args, **kwargs)
        self.inline_parser = Parser(Language(inline_language()), *args, **kwargs)

    def parse(self, *args, **kwargs) -> MarkdownTree:
        block_tree = self.block_parser.parse(*args, **kwargs)
        inline_trees = {}

        def parse_inline_node(node: Node) -> None:
            if is_inline_type(node.type):
                self.inline_parser.included_ranges = [node.range]
                inline_tree = self.inline_parser.parse(*args, **kwargs)
                inline_trees[node.id] = inline_tree

        walk(block_tree.walk(), parse_inline_node)

        return MarkdownTree(block_tree, inline_trees)
