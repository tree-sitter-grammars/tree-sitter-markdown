package tree_sitter_markdown

// #cgo CFLAGS: -std=c11 -fPIC
// #cgo LDFLAGS: -L../.. -l:libtree-sitter-markdown.a
// #include "../c/tree-sitter-markdown.h"
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
