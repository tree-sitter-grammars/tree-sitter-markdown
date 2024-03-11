{
  "targets": [
    {
      "target_name": "tree_sitter_markdown_binding",
      "dependencies": [
        "<!(node -p \"require('node-addon-api').targets\"):node_addon_api_except",
      ],
      "include_dirs": [
        "tree-sitter-markdown/src",
      ],
      "sources": [
        "bindings/node/binding.cc",
        "tree-sitter-markdown/src/parser.c",
        "tree-sitter-markdown/src/scanner.c",
        "tree-sitter-markdown-inline/src/parser.c",
        "tree-sitter-markdown-inline/src/scanner.c",
      ],
      "cflags_c": [
        "-std=c11",
      ],
    }
  ]
}
