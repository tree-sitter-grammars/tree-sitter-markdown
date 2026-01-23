package tree_sitter_markdown

// #cgo CFLAGS: -std=c11 -fPIC
// #include "../../../tree-sitter-markdown/src/parser.c"
// #if __has_include("../../../tree-sitter-markdown/src/scanner.c")
// #include "../../../tree-sitter-markdown/src/scanner.c"
// #endif
import "C"

import "unsafe"

// Get the tree-sitter Language for this grammar.
func Language() unsafe.Pointer {
	return unsafe.Pointer(C.tree_sitter_markdown())
}
