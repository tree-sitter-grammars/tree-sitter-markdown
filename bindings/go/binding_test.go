package tree_sitter_markdown_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter-grammars/tree-sitter-markdown"
)

func TestCanLoadBlockGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_markdown.Language())
	if language == nil {
		t.Errorf("Error loading Markdown block grammar")
	}
}

func TestCanLoadInlineGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_markdown.InlineLanguage())
	if language == nil {
		t.Errorf("Error loading Markdown inline grammar")
	}
}
