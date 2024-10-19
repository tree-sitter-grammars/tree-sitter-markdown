set(PROJECT_DESCRIPTION "Markdown block grammar for tree-sitter")
set(TS_REQUIRES tree-sitter-markdown-inline)

add_parser(markdown)

add_custom_target(test "${TREE_SITTER_CLI}" test
                  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
                  DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/parser.c"
                  COMMENT "tree-sitter test")
