cmake_minimum_required(VERSION 3.13)

project(tree-sitter-markdown
        VERSION "0.5.1"
        DESCRIPTION "Markdown grammar for tree-sitter"
        HOMEPAGE_URL "https://github.com/tree-sitter-grammars/tree-sitter-markdown"
        LANGUAGES C)

option(BUILD_SHARED_LIBS "Build using shared libraries" ON)
option(TREE_SITTER_REUSE_ALLOCATOR "Reuse the library allocator" OFF)
option(ALL_EXTENSIONS "Enable all Markdown extensions" OFF)

set(TREE_SITTER_ABI_VERSION 14 CACHE STRING "Tree-sitter ABI version")
if(NOT ${TREE_SITTER_ABI_VERSION} MATCHES "^[0-9]+$")
    unset(TREE_SITTER_ABI_VERSION CACHE)
    message(FATAL_ERROR "TREE_SITTER_ABI_VERSION must be an integer")
endif()

find_program(TREE_SITTER_CLI tree-sitter DOC "Tree-sitter CLI")

include(GNUInstallDirs)

macro(add_parser name)
    add_custom_command(OUTPUT "${CMAKE_CURRENT_SOURCE_DIR}/src/parser.c"
                       DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/grammar.js"
                       COMMAND "${CMAKE_COMMAND}" -E env ALL_EXTENSIONS=$<BOOL:${ALL_EXTENSIONS}>
                                -- "${TREE_SITTER_CLI}" generate --abi=${TREE_SITTER_ABI_VERSION}
                       BYPRODUCTS "${CMAKE_CURRENT_SOURCE_DIR}/src/grammar.json"
                                  "${CMAKE_CURRENT_SOURCE_DIR}/src/node-types.json"
                                  "${CMAKE_CURRENT_SOURCE_DIR}/src/tree_sitter/alloc.h"
                                  "${CMAKE_CURRENT_SOURCE_DIR}/src/tree_sitter/array.h"
                                  "${CMAKE_CURRENT_SOURCE_DIR}/src/tree_sitter/parser.h"
                       WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
                       COMMENT "Generating parser.c")

    add_library(tree-sitter-${name}
                "${CMAKE_CURRENT_SOURCE_DIR}/src/parser.c"
                "${CMAKE_CURRENT_SOURCE_DIR}/src/scanner.c")
    target_include_directories(tree-sitter-${name} PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/src")

    target_compile_definitions(tree-sitter-${name} PRIVATE
                               $<$<BOOL:${TREE_SITTER_REUSE_ALLOCATOR}>:TREE_SITTER_REUSE_ALLOCATOR>
                               $<$<CONFIG:Debug>:TREE_SITTER_DEBUG>)

    set_target_properties(tree-sitter-${name}
                          PROPERTIES
                          C_STANDARD 11
                          POSITION_INDEPENDENT_CODE ON
                          SOVERSION "${TREE_SITTER_ABI_VERSION}.${PROJECT_VERSION_MAJOR}"
                          DEFINE_SYMBOL "")

    configure_file("${CMAKE_CURRENT_SOURCE_DIR}/bindings/c/tree-sitter-${name}.pc.in"
                   "${CMAKE_CURRENT_BINARY_DIR}/tree-sitter-${name}.pc" @ONLY)

    install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/bindings/c/tree-sitter-${name}.h"
            DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}/tree_sitter")
    install(FILES "${CMAKE_CURRENT_BINARY_DIR}/tree-sitter-${name}.pc"
            DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig")
    install(TARGETS tree-sitter-${name}
            LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}")
endmacro()

add_subdirectory(tree-sitter-markdown tree-sitter-markdown)

add_subdirectory(tree-sitter-markdown-inline tree-sitter-markdown-inline)
