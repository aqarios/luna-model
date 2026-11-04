/*
 * LunaModel C API — umbrella header.
 *
 *   Standalone (default): link liblunamodel and call lm_* directly.
 *       #include <luna_model.h>
 *       LmModel *m = lm_model_new();
 *
 *   Python extension: define LUNA_MODEL_PYTHON_EXTENSION before including this
 *   header, then call import_luna_model() once during your module init. The
 * lm_* calls then dispatch through the wheel's `_C_API` capsule (same call
 * syntax). #define LUNA_MODEL_PYTHON_EXTENSION #include <luna_model.h>   //
 * pulls in <Python.h> for you
 */
#ifndef LUNA_MODEL_H
#define LUNA_MODEL_H

#if defined(LUNA_MODEL_PYTHON_EXTENSION)
#include <Python.h>
#endif

#include "luna_model/attributes.h"
#include "luna_model/types.h"
#include "luna_model/version.h"

#if defined(LUNA_MODEL_PYTHON_EXTENSION)
#include "luna_model/funcs_py.h"
#else
#include "luna_model/funcs.h"
#endif

#endif /* LUNA_MODEL_H */
