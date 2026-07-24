#ifndef LUNA_MODEL_NANOBIND_HPP
#define LUNA_MODEL_NANOBIND_HPP

// nanobind type casters for luna-model's C++ wrappers.
//
// Include this (in addition to your usual nanobind headers) in a nanobind extension so that free
// functions and methods can take and return luna-model objects directly — Model, Solution,
// Environment, Expression, Constraint, ConstraintCollection, Variable, Bounds, ModelSpecs —
// converting to and from the corresponding luna_model Python objects. This is the nanobind analog of
// the pyo3 SDK's FromPyObject/IntoPyObject, and covers the same set of transferable types. It is
// opt-in: only this header pulls in nanobind, so the rest of the C++ SDK stays binding-agnostic. The
// conversion itself lives in `python.hpp`.
//
//     #include <luna_model/nanobind.hpp>
//     m.def("num_variables", [](lm::Model model) { return model.num_variables(); });
//     m.def("empty_model",   [](std::string name) { lm::Model m; m.set_name(name); return m; });

// Compiles to nothing unless building a Python extension (LUNA_MODEL_PYTHON_EXTENSION), which is the
// only configuration where nanobind and the capsule transport are available.
#if defined(LUNA_MODEL_PYTHON_EXTENSION)

#include <array>
#include <cstring>
#include <optional>
#include <utility>

#include <nanobind/nanobind.h>

#include "luna_model/python.hpp"

namespace nanobind {
namespace detail {

/// Shared caster body for a luna-model wrapper `Wrapper`, delegating to the capsule transport in
/// `lm::python`. Uses optional storage rather than `NB_TYPE_CASTER` so it also works for wrappers
/// that are not default-constructible (Constraint, Variable, Bounds, ModelSpecs).
template <typename Wrapper>
struct luna_model_caster {
    using Value = Wrapper;
    static constexpr auto Name = const_name("luna_model object");
    template <typename T_>
    using Cast = movable_cast_t<T_>;
    template <typename T_>
    static constexpr bool can_cast() {
        return true;
    }

    std::optional<Wrapper> storage;

    bool from_python(handle src, uint8_t /*flags*/, cleanup_list * /*cleanup*/) noexcept {
        try {
            storage.emplace(lm::python::from_object<Wrapper>(src.ptr()));
            return true;
        } catch (...) {
            // from_python must not throw; a set Python error (if any) is reported by nanobind.
            return false;
        }
    }

    static handle from_cpp(Wrapper value, rv_policy /*policy*/, cleanup_list * /*cleanup*/) noexcept {
        return lm::python::to_object<Wrapper>(std::move(value));  // new reference (or null on error)
    }
    static handle from_cpp(Wrapper *value, rv_policy policy, cleanup_list *cleanup) noexcept {
        if (value == nullptr) {
            return none().release();
        }
        return from_cpp(std::move(*value), policy, cleanup);
    }

    explicit operator Wrapper *() { return storage ? &*storage : nullptr; }
    explicit operator Wrapper &() { return *storage; }
    explicit operator Wrapper &&() { return std::move(*storage); }
};

template <>
struct type_caster<lm::Model> : luna_model_caster<lm::Model> {};
template <>
struct type_caster<lm::Solution> : luna_model_caster<lm::Solution> {};
template <>
struct type_caster<lm::Env> : luna_model_caster<lm::Env> {};
template <>
struct type_caster<lm::Expression> : luna_model_caster<lm::Expression> {};
template <>
struct type_caster<lm::Constraint> : luna_model_caster<lm::Constraint> {};
template <>
struct type_caster<lm::ConstraintCollection> : luna_model_caster<lm::ConstraintCollection> {};
template <>
struct type_caster<lm::Vref> : luna_model_caster<lm::Vref> {};
template <>
struct type_caster<lm::LazyBounds> : luna_model_caster<lm::LazyBounds> {};
template <>
struct type_caster<lm::Specs> : luna_model_caster<lm::Specs> {};

// --- Enum casters: luna_model.<Enum> <-> the C enum -----------------------------------------------
//
// luna-model's Python enums are string-valued Enum members, so nanobind's built-in (int-based) enum
// handling can't convert them. These casters map by the Python member name, per the tables below
// (mirroring pyo3-lunamodel's `python_enum_wrapper!` variant lists). `TranslationTarget` has no
// C-ABI enum, so it is not included.

/// Per-enum interop data: the Python class under `luna_model`, and the member-name <-> C-value table.
template <typename Enum>
struct enum_traits;

template <>
struct enum_traits<LmSense> {
    static constexpr const char *py_class = "Sense";
    static constexpr std::array<std::pair<const char *, LmSense>, 2> members{{
        {"MIN", LmSense_Min},
        {"MAX", LmSense_Max},
    }};
};

template <>
struct enum_traits<LmVtype> {
    static constexpr const char *py_class = "Vtype";
    static constexpr std::array<std::pair<const char *, LmVtype>, 5> members{{
        {"BINARY", LmVtype_Binary},
        {"INVERTED_BINARY", LmVtype_InvertedBinary},
        {"SPIN", LmVtype_Spin},
        {"INTEGER", LmVtype_Integer},
        {"REAL", LmVtype_Real},
    }};
};

template <>
struct enum_traits<LmComparator> {
    static constexpr const char *py_class = "Comparator";
    static constexpr std::array<std::pair<const char *, LmComparator>, 3> members{{
        {"EQ", LmComparator_Eq},
        {"LE", LmComparator_Le},
        {"GE", LmComparator_Ge},
    }};
};

template <>
struct enum_traits<LmCtype> {
    static constexpr const char *py_class = "Ctype";
    static constexpr std::array<std::pair<const char *, LmCtype>, 5> members{{
        {"UNCONSTRAINED", LmCtype_Unconstrained},
        {"EQUALITY", LmCtype_Equality},
        {"INEQUALITY", LmCtype_Inequality},
        {"LESS_EQUAL", LmCtype_LessEqual},
        {"GREATER_EQUAL", LmCtype_GreaterEqual},
    }};
};

template <>
struct enum_traits<LmValueSource> {
    static constexpr const char *py_class = "ValueSource";
    static constexpr std::array<std::pair<const char *, LmValueSource>, 2> members{{
        {"RAW", LmValueSource_Raw},
        {"OBJ", LmValueSource_Obj},
    }};
};

/// Shared caster body for a luna-model C enum, matching a Python member by its `.name`.
template <typename Enum>
struct enum_caster {
    NB_TYPE_CASTER(Enum, const_name("luna_model enum"))

    bool from_python(handle src, uint8_t /*flags*/, cleanup_list * /*cleanup*/) noexcept {
        PyObject *name = PyObject_GetAttrString(src.ptr(), "name");
        if (name == nullptr) {
            PyErr_Clear();
            return false;
        }
        const char *member = PyUnicode_AsUTF8(name);
        bool matched = false;
        if (member != nullptr) {
            for (const auto &[py_name, c_value] : enum_traits<Enum>::members) {
                if (std::strcmp(py_name, member) == 0) {
                    value = c_value;
                    matched = true;
                    break;
                }
            }
        }
        Py_DECREF(name);
        return matched;
    }

    static handle from_cpp(Enum c_value, rv_policy /*policy*/, cleanup_list * /*cleanup*/) noexcept {
        const char *member = nullptr;
        for (const auto &[py_name, ev] : enum_traits<Enum>::members) {
            if (ev == c_value) {
                member = py_name;
                break;
            }
        }
        if (member == nullptr) {
            return handle();
        }
        PyObject *module = PyImport_ImportModule("luna_model");
        if (module == nullptr) {
            return handle();
        }
        PyObject *cls = PyObject_GetAttrString(module, enum_traits<Enum>::py_class);
        Py_DECREF(module);
        if (cls == nullptr) {
            return handle();
        }
        PyObject *result = PyObject_GetAttrString(cls, member);  // luna_model.<Class>.<MEMBER>
        Py_DECREF(cls);
        return handle(result);  // new reference
    }
};

template <>
struct type_caster<LmSense> : enum_caster<LmSense> {};
template <>
struct type_caster<LmVtype> : enum_caster<LmVtype> {};
template <>
struct type_caster<LmComparator> : enum_caster<LmComparator> {};
template <>
struct type_caster<LmCtype> : enum_caster<LmCtype> {};
template <>
struct type_caster<LmValueSource> : enum_caster<LmValueSource> {};

}  // namespace detail
}  // namespace nanobind

#endif  // LUNA_MODEL_PYTHON_EXTENSION

#endif  // LUNA_MODEL_NANOBIND_HPP
