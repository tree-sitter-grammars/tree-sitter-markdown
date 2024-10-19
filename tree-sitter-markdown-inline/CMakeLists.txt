set(PROJECT_DESCRIPTION "Markdown inline grammar for tree-sitter")

add_parser(markdown-inline)

add_custom_target(test-inline "${TREE_SITTER_CLI}" test
                  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
                  DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/parser.c"
                  COMMENT "tree-sitter test")
