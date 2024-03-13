package tree_sitter_markdown

// #cgo CFLAGS: -std=c11 -fPIC
// #cgo LDFLAGS: -L../../tree-sitter-markdown -l:libtree-sitter-markdown.a
// #cgo LDFLAGS: -L../../tree-sitter-markdown-inline -l:libtree-sitter-markdown-inline.a
// #include "../../tree-sitter-markdown/bindings/c/tree-sitter-markdown.h"
// #include "../../tree-sitter-markdown-inline/bindings/c/tree-sitter-markdown-inline.h"
import "C"

import "unsafe"

// Get the tree-sitter Language for the block grammar.
func Language() unsafe.Pointer {
	return unsafe.Pointer(C.tree_sitter_markdown())
}

// Get the tree-sitter Language for the inline grammar.
func InlineLanguage() unsafe.Pointer {
	return unsafe.Pointer(C.tree_sitter_markdown_inline())
}
