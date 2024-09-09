from unittest import TestCase

import tree_sitter, tree_sitter_markdown


class TestLanguage(TestCase):
    def test_can_load_block_grammar(self):
        try:
            tree_sitter.Language(tree_sitter_markdown.language())
        except Exception:
            self.fail("Error loading Markdown block grammar")

    def test_can_load_block_grammar(self):
        try:
            tree_sitter.Language(tree_sitter_markdown.inline_language())
        except Exception:
            self.fail("Error loading Markdown inline grammar")
