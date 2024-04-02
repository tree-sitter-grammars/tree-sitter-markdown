package tree_sitter_markdown

// #cgo CPPFLAGS: -I../../tree-sitter-markdown-inline
// #cgo CFLAGS: -std=c11 -fPIC
// #include "../../tree-sitter-markdown-inline/src/parser.c"
// #include "../../tree-sitter-markdown-inline/src/scanner.c"
import "C"

import "unsafe"

// Get the tree-sitter Language for the inline grammar.
func InlineLanguage() unsafe.Pointer {
	return unsafe.Pointer(C.tree_sitter_markdown_inline())
}
