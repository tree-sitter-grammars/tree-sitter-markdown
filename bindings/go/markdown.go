package tree_sitter_markdown

// #cgo CPPFLAGS: -I../../tree-sitter-markdown
// #cgo CFLAGS: -std=c11 -fPIC
// #include "../../tree-sitter-markdown/src/parser.c"
// #include "../../tree-sitter-markdown/src/scanner.c"
import "C"

import "unsafe"

// Get the tree-sitter Language for the block grammar.
func Language() unsafe.Pointer {
	return unsafe.Pointer(C.tree_sitter_markdown())
}
