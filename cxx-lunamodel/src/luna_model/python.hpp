#ifndef LUNA_MODEL_PYTHON_HPP
#define LUNA_MODEL_PYTHON_HPP

// Binding-agnostic interop between luna-model's Python objects and the C++ wrappers.
//
// luna-model's Python types (luna_model.Model / Solution / Environment) carry their underlying C
// handle across an extension boundary via a PyCapsule — the same transport the pyo3 SDK uses. These
// helpers let arbitrary C++ code (exposed to Python with *any* binding) receive such an object as an
// `lm::Model`/`lm::Solution`/`lm::Env` and hand one back, without depending on nanobind or pybind11.
// nanobind users get this implicitly through the type casters in `luna_model/nanobind.hpp`.
//
// The Python type is a thin wrapper around a pyo3 object that actually owns the handle: the wrapper
// holds it under an attribute (`_m`/`_s`/`_env`), the pyo3 class lives in `luna_model._lm`, and the
// capsule methods (`_to_c_capsule` / `_from_c_capsule`) are on the pyo3 object. So extraction goes
// wrapper -> inner -> capsule -> handle, and construction goes handle -> capsule ->
// `_lm.Py<T>._from_c_capsule` -> `luna_model.<T>._from_py<...>`.
//
// Available only when building a Python extension: it needs <Python.h> and dispatches the lm_*
// handle calls through the imported C API table.

// The whole header compiles to nothing unless building a Python extension
// (LUNA_MODEL_PYTHON_EXTENSION provides <Python.h> and the lm_* vtable), so it is safe to include
// unconditionally — the helpers exist only in that configuration.
#if defined(LUNA_MODEL_PYTHON_EXTENSION)

#include <cstdlib>
#include <string>

#include <luna_model.h>

#include "luna_model/constraint.hpp"
#include "luna_model/constraint_collection.hpp"
#include "luna_model/env.hpp"
#include "luna_model/error.hpp"
#include "luna_model/expr.hpp"
#include "luna_model/lazy_bounds.hpp"
#include "luna_model/model.hpp"
#include "luna_model/solution.hpp"
#include "luna_model/specs.hpp"
#include "luna_model/vref.hpp"

namespace lm {
namespace python {

/// Maps a C++ wrapper to the luna-model Python-interop protocol for its type: the raw handle type,
/// the versioned capsule name, how to free a handle, and the names used to reach through the Python
/// wrapper to the pyo3 object that owns the handle. Specialized per transferable type; the transport
/// below is written once against it.
template <typename Wrapper>
struct capsule_traits;

template <>
struct capsule_traits<Model> {
    using handle = LmModel;
    static constexpr const char *capsule_name = "luna_model.capi.Model.v1";
    static constexpr const char *py_class = "Model";        // luna_model.Model (the wrapper)
    static constexpr const char *raw_class = "PyModel";     // luna_model._lm.PyModel (the pyo3 obj)
    static constexpr const char *inner_attr = "_m";         // wrapper -> pyo3 obj
    static constexpr const char *from_py = "_from_pym";     // wrapper._from_pym(pyo3 obj)
    static void free(LmModel *h) { lm_model_free(h); }
};

template <>
struct capsule_traits<Solution> {
    using handle = LmSol;
    static constexpr const char *capsule_name = "luna_model.capi.Solution.v1";
    static constexpr const char *py_class = "Solution";
    static constexpr const char *raw_class = "PySolution";
    static constexpr const char *inner_attr = "_s";
    static constexpr const char *from_py = "_from_pys";
    static void free(LmSol *h) { lm_sol_free(h); }
};

template <>
struct capsule_traits<Env> {
    using handle = LmEnv;
    static constexpr const char *capsule_name = "luna_model.capi.Environment.v1";
    static constexpr const char *py_class = "Environment";
    static constexpr const char *raw_class = "PyEnvironment";
    static constexpr const char *inner_attr = "_env";
    static constexpr const char *from_py = "_from_pyenv";
    static void free(LmEnv *h) { lm_env_free(h); }
};

template <>
struct capsule_traits<Expression> {
    using handle = LmExpr;
    static constexpr const char *capsule_name = "luna_model.capi.Expression.v1";
    static constexpr const char *py_class = "Expression";
    static constexpr const char *raw_class = "PyExpression";
    static constexpr const char *inner_attr = "_expr";
    static constexpr const char *from_py = "_from_pyexpr";
    static void free(LmExpr *h) { lm_expr_free(h); }
};

template <>
struct capsule_traits<Constraint> {
    using handle = LmConstraint;
    static constexpr const char *capsule_name = "luna_model.capi.Constraint.v1";
    static constexpr const char *py_class = "Constraint";
    static constexpr const char *raw_class = "PyConstraint";
    static constexpr const char *inner_attr = "_c";
    static constexpr const char *from_py = "_from_pyc";
    static void free(LmConstraint *h) { lm_constr_free(h); }
};

template <>
struct capsule_traits<ConstraintCollection> {
    using handle = LmCC;
    static constexpr const char *capsule_name = "luna_model.capi.ConstraintCollection.v1";
    static constexpr const char *py_class = "ConstraintCollection";
    static constexpr const char *raw_class = "PyConstraintCollection";
    static constexpr const char *inner_attr = "_cc";
    static constexpr const char *from_py = "_from_pycc";
    static void free(LmCC *h) { lm_cc_free(h); }
};

template <>
struct capsule_traits<Vref> {
    using handle = LmVref;
    static constexpr const char *capsule_name = "luna_model.capi.Variable.v1";
    static constexpr const char *py_class = "Variable";
    static constexpr const char *raw_class = "PyVariable";
    static constexpr const char *inner_attr = "_v";
    static constexpr const char *from_py = "_from_pyvar";
    static void free(LmVref *h) { lm_vref_free(h); }
};

template <>
struct capsule_traits<LazyBounds> {
    using handle = LmLazyBounds;
    static constexpr const char *capsule_name = "luna_model.capi.Bounds.v1";
    static constexpr const char *py_class = "Bounds";
    static constexpr const char *raw_class = "PyBounds";
    static constexpr const char *inner_attr = "_b";
    static constexpr const char *from_py = "_from_pyb";
    static void free(LmLazyBounds *h) { lm_lazy_bounds_free(h); }
};

template <>
struct capsule_traits<Specs> {
    using handle = LmSpecs;
    static constexpr const char *capsule_name = "luna_model.capi.ModelSpecs.v1";
    static constexpr const char *py_class = "ModelSpecs";
    static constexpr const char *raw_class = "PyModelSpecs";
    static constexpr const char *inner_attr = "_sp";
    static constexpr const char *from_py = "_from_pysp";
    static void free(LmSpecs *h) { lm_specs_free(h); }
};

namespace detail {

// The capsule payload begins with the handle pointer, matching the host's transport struct. Reading
// the handle and nulling it in place transfers ownership: the still-live owner then frees nothing.
struct HandleSlot {
    void *handle;
};

}  // namespace detail

/// Extracts an owned `Wrapper` from a luna-model Python object, consuming the handle its pyo3 object
/// exposes via `_to_c_capsule()`. Throws `lm::Error` (with a Python exception set) if `obj` is not
/// the expected luna-model type or its capsule was already consumed.
template <typename Wrapper>
Wrapper from_object(PyObject *obj) {
    using traits = capsule_traits<Wrapper>;

    // The wrapper holds the pyo3 object under `inner_attr`; fall back to `obj` if it is already raw.
    PyObject *inner = PyObject_GetAttrString(obj, traits::inner_attr);
    if (inner == nullptr) {
        PyErr_Clear();
        Py_INCREF(obj);
        inner = obj;
    }
    PyObject *capsule = PyObject_CallMethod(inner, "_to_c_capsule", nullptr);
    Py_DECREF(inner);
    if (capsule == nullptr) {
        throw Error(LmStatus_InvalidArgument,
                    std::string("expected a luna_model.") + traits::py_class);
    }

    typename traits::handle *handle = nullptr;
    void *raw = PyCapsule_GetPointer(capsule, traits::capsule_name);
    if (raw != nullptr) {
        auto *slot = static_cast<detail::HandleSlot *>(raw);
        handle = static_cast<typename traits::handle *>(slot->handle);
        slot->handle = nullptr;  // consume: the owning capsule must not free what we now hold
    }
    Py_DECREF(capsule);

    if (handle == nullptr) {
        if (PyErr_Occurred() == nullptr) {
            PyErr_Format(PyExc_ValueError, "%s capsule was empty or already consumed",
                         traits::capsule_name);
        }
        throw Error(LmStatus_InvalidArgument, "failed to read handle from capsule");
    }
    return Wrapper(handle);  // adopt ownership
}

/// Wraps an owned `Wrapper` as the corresponding luna-model Python object, transferring ownership.
/// Returns a new reference, or `nullptr` with a Python exception set.
template <typename Wrapper>
PyObject *to_object(Wrapper wrapper) {
    using traits = capsule_traits<Wrapper>;

    typename traits::handle *handle = wrapper.release();

    auto *slot = static_cast<detail::HandleSlot *>(std::malloc(sizeof(detail::HandleSlot)));
    if (slot == nullptr) {
        traits::free(handle);
        return PyErr_NoMemory();
    }
    slot->handle = handle;

    PyObject *capsule = PyCapsule_New(slot, traits::capsule_name, [](PyObject *cap) {
        void *p = PyCapsule_GetPointer(cap, capsule_traits<Wrapper>::capsule_name);
        if (p == nullptr) {
            return;
        }
        auto *s = static_cast<detail::HandleSlot *>(p);
        if (s->handle != nullptr) {  // not consumed by _from_c_capsule -> release it
            capsule_traits<Wrapper>::free(static_cast<typename traits::handle *>(s->handle));
        }
        std::free(s);
    });
    if (capsule == nullptr) {
        traits::free(handle);
        std::free(slot);
        return nullptr;
    }

    PyObject *module = PyImport_ImportModule("luna_model");
    if (module == nullptr) {
        Py_DECREF(capsule);  // its destructor frees handle + slot
        return nullptr;
    }

    // inner = luna_model._lm.Py<T>._from_c_capsule(capsule)   (consumes the handle)
    PyObject *lm_sub = PyObject_GetAttrString(module, "_lm");
    PyObject *raw_cls = lm_sub ? PyObject_GetAttrString(lm_sub, traits::raw_class) : nullptr;
    Py_XDECREF(lm_sub);
    PyObject *inner =
        raw_cls ? PyObject_CallMethod(raw_cls, "_from_c_capsule", "O", capsule) : nullptr;
    Py_XDECREF(raw_cls);
    Py_DECREF(capsule);  // handle now owned by `inner`; destructor frees only the slot
    if (inner == nullptr) {
        Py_DECREF(module);
        return nullptr;
    }

    // result = luna_model.<T>._from_py<...>(inner)
    PyObject *cls = PyObject_GetAttrString(module, traits::py_class);
    Py_DECREF(module);
    if (cls == nullptr) {
        Py_DECREF(inner);
        return nullptr;
    }
    PyObject *result = PyObject_CallMethod(cls, traits::from_py, "O", inner);
    Py_DECREF(cls);
    Py_DECREF(inner);
    return result;
}

}  // namespace python
}  // namespace lm

#endif  // LUNA_MODEL_PYTHON_EXTENSION

#endif  // LUNA_MODEL_PYTHON_HPP
