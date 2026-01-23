package tree_sitter_markdown_inline_test

import (
	"testing"

	tree_sitter_markdown_inline "github.com/tree-sitter-grammars/tree-sitter-markdown/tree-sitter-markdown-inline/bindings/go"
	tree_sitter "github.com/tree-sitter/go-tree-sitter"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_markdown_inline.Language())
	if language == nil {
		t.Errorf("Error loading MarkdownInline grammar")
	}
}
