#include <napi.h>

typedef struct TSLanguage TSLanguage;

extern "C" TSLanguage * tree_sitter_markdown();
extern "C" TSLanguage * tree_sitter_markdown_inline();

// "tree-sitter", "language" hashed with BLAKE2
const napi_type_tag LANGUAGE_TYPE_TAG = {
  0x8AF2E5212AD58ABF, 0xD5006CAD83ABBA16
};

Napi::Object Init(Napi::Env env, Napi::Object exports) {
    exports["name"] = Napi::String::New(env, "markdown");
    auto markdown_language = Napi::External<TSLanguage>::New(env, tree_sitter_markdown());
    markdown_language.TypeTag(&LANGUAGE_TYPE_TAG);
    exports["language"] = markdown_language;

    auto md_inline = Napi::Object::New(env);
    md_inline["name"] = Napi::String::New(env, "markdown_inline");
    auto md_inline_language = Napi::External<TSLanguage>::New(env, tree_sitter_markdown_inline());
    md_inline_language.TypeTag(&LANGUAGE_TYPE_TAG);
    md_inline["language"] = md_inline_language;
    exports["inline"] = md_inline;

    return exports;
}

NODE_API_MODULE(tree_sitter_markdown_binding, Init);
