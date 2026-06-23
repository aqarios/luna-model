/*
 * LunaModel C API — public header.
 *
 * Two modes:
 *
 *   1. Standalone (default)
 *      Link against liblunamodel_capi and call the lm_* functions directly:
 *
 *          #include <luna_model.h>
 *
 *          LmModel *m = lm_model_new();
 *          LmBuffer buf;
 *          if (lm_model_serialize(m, &buf) == LmStatus_Ok) {
 *              // ... use buf.data / buf.len ...
 *              lm_buffer_free(buf);
 *          }
 *          lm_model_free(m);
 *
 *   2. Python extension
 *      When LunaModel is used from inside the `luna_model` wheel, the C API is
 *      published as a PyCapsule (`luna_model._lm._C_API`) rather than linked.
 *      Define LUNA_MODEL_PYTHON_EXTENSION before including this header, include
 *      <Python.h> first, and call import_luna_model() once during module init.
 *      Then call through lunamodel_api():
 *
 *          #define LUNA_MODEL_PYTHON_EXTENSION
 *          #include <Python.h>
 *          #include <luna_model.h>
 *
 *          static int my_module_exec(PyObject *module) {
 *              if (import_luna_model() < 0) {
 *                  return -1;
 *              }
 *              LmModel *m = lunamodel_api()->model_new();
 *              // ...
 *              lunamodel_api()->model_free(m);
 *              return 0;
 *          }
 *
 *      Calls through the table reach the single LunaModel instance already
 *      loaded by the wheel, with a stable ABI across compiler and pyo3
 *      versions. Do not call the bare lm_* symbols in this mode; they are not
 *      linked.
 */
#ifndef LUNA_MODEL_H
#define LUNA_MODEL_H

#include "luna_model_decls.h"

#ifdef LUNA_MODEL_PYTHON_EXTENSION

#include <Python.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Name of the capsule published by the luna_model wheel. */
#define LUNA_MODEL_CAPI_CAPSULE_NAME "luna_model._lm._C_API"

/*
 * The C API table imported from the wheel. NULL until import_luna_model()
 * succeeds. Each translation unit that includes this header in extension mode
 * has its own pointer; call import_luna_model() once per such unit.
 */
static const LunaModelCApi *lunamodel__api = NULL;

/* Returns the imported C API table, or NULL if import_luna_model() has not run.
 */
static inline const LunaModelCApi *lunamodel_api(void) {
  return lunamodel__api;
}

/*
 * Imports `luna_model._lm` and binds its `_C_API` capsule.
 *
 * Returns 0 on success, or -1 on failure with a Python exception set. Call once
 * during your extension module's initialization, before any lunamodel_api()
 * use.
 */
static inline int import_luna_model(void) {
  PyObject *module = PyImport_ImportModule("luna_model._lm");
  if (module == NULL) {
    return -1;
  }

  PyObject *capsule = PyObject_GetAttrString(module, "_C_API");
  Py_DECREF(module);
  if (capsule == NULL) {
    return -1;
  }

  void *pointer = PyCapsule_GetPointer(capsule, LUNA_MODEL_CAPI_CAPSULE_NAME);
  Py_DECREF(capsule);
  if (pointer == NULL) {
    return -1;
  }

  const LunaModelCApi *api = (const LunaModelCApi *)pointer;
  if (api->abi_version != LUNA_MODEL_CAPI_ABI_VERSION) {
    PyErr_Format(
        PyExc_ImportError,
        "luna_model C API ABI mismatch: this extension was built against "
        "version %d, but the installed luna_model provides version %u",
        (int)LUNA_MODEL_CAPI_ABI_VERSION, api->abi_version);
    return -1;
  }

  lunamodel__api = api;
  return 0;
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* LUNA_MODEL_PYTHON_EXTENSION */

#endif /* LUNA_MODEL_H */
