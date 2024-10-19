"Markdown grammar for tree-sitter"

from ._binding import language, inline_language

from .parser import MarkdownParser, MarkdownTree, MarkdownCursor

__all__ = ["language", "inline_language", "MarkdownCursor", "MarkdownTree", "MarkdownParser"]
