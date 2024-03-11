#include <Python.h>

typedef struct TSLanguage TSLanguage;

TSLanguage *tree_sitter_markdown(void);

TSLanguage *tree_sitter_markdown_inline(void);

static PyObject* _binding_language(PyObject *self, PyObject *args) {
    return PyLong_FromVoidPtr(tree_sitter_markdown());
}

static PyObject* _binding_inline_language(PyObject *self, PyObject *args) {
    return PyLong_FromVoidPtr(tree_sitter_markdown_inline());
}

static PyMethodDef methods[] = {
    {"language", _binding_language, METH_NOARGS,
     "Get the tree-sitter language for the block grammar."},
    {"inline_language", _binding_inline_language, METH_NOARGS,
     "Get the tree-sitter language for the inline grammar."},
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef module = {
    .m_base = PyModuleDef_HEAD_INIT,
    .m_name = "_binding",
    .m_doc = NULL,
    .m_size = -1,
    .m_methods = methods
};

PyMODINIT_FUNC PyInit__binding(void) {
    return PyModule_Create(&module);
}
