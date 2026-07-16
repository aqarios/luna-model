#ifndef LUNA_MODEL_CONSTRAINT_COLLECTION_HPP
#define LUNA_MODEL_CONSTRAINT_COLLECTION_HPP

#include <cstdint>
#include <map>
#include <optional>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include <luna_model.h>

#include "luna_model/constraint.hpp"
#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/error.hpp"
#include "luna_model/expr.hpp"
#include "luna_model/vref.hpp"

namespace lm {

/* Owning handle to a named collection of constraints (frees on destruction). */
class ConstraintCollection {
  public:
    /* Creates an empty collection. */
    ConstraintCollection() : handle_(lm_cc_new()) {
        if (handle_ == nullptr) {
            throw Error(LmStatus_Panic, "failed to allocate a LunaModel constraint collection");
        }
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit ConstraintCollection(LmCC *handle) noexcept : handle_(handle) {}

    ~ConstraintCollection() {
        if (handle_ != nullptr) {
            lm_cc_free(handle_);
        }
    }

    ConstraintCollection(ConstraintCollection &&other) noexcept : handle_(other.handle_) {
        other.handle_ = nullptr;
    }
    ConstraintCollection &operator=(ConstraintCollection &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_cc_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    ConstraintCollection(const ConstraintCollection &) = delete;
    ConstraintCollection &operator=(const ConstraintCollection &) = delete;

    // --- size / membership ---

    /* Number of constraints in the collection. */
    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_cc_len(handle_, &out));
        return out;
    }

    /* Whether the collection has no constraints. */
    bool is_empty() const {
        bool out = false;
        detail::check(lm_cc_is_empty(handle_, &out));
        return out;
    }

    /* Whether a constraint named `key` exists. */
    bool contains(const std::string &key) const {
        bool out = false;
        detail::check(lm_cc_contains(handle_, key.c_str(), &out));
        return out;
    }

    /* The names of all constraints, in order. */
    std::vector<std::string> names() const {
        LmStrBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_names(handle_, &buf));
        std::vector<std::string> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.emplace_back(buf.data[i]);
        }
        lm_str_buffer_free(buf);
        return out;
    }

    /* The constraint named `key` (throws if absent). */
    Constraint get(const std::string &key) const {
        LmConstraint *out = nullptr;
        detail::check(lm_cc_get(handle_, key.c_str(), &out));
        return Constraint(out);
    }

    /* Every `(name, constraint)` pair, in name order. */
    std::vector<std::pair<std::string, Constraint>> iter() const {
        std::vector<std::pair<std::string, Constraint>> out;
        for (std::string &name : names()) {
            Constraint c = get(name);
            out.emplace_back(std::move(name), std::move(c));
        }
        return out;
    }

    // --- introspection ---

    /* The distinct variable types used across the collection. */
    std::vector<LmVtype> vtypes() const {
        LmVtypeBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_vtypes(handle_, &buf));
        std::vector<LmVtype> out(buf.data, buf.data + buf.len);
        lm_vtype_buffer_free(buf);
        return out;
    }

    /* The distinct constraint types used across the collection. */
    std::vector<LmComparator> ctypes() const {
        LmCmpBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_ctypes(handle_, &buf));
        std::vector<LmComparator> out(buf.data, buf.data + buf.len);
        lm_cmp_buffer_free(buf);
        return out;
    }

    /* The distinct variables used across the collection. */
    std::vector<Vref> vars() const {
        LmVrefBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_vars(handle_, &buf));
        return adopt_vrefs(buf);
    }

    /* The distinct variables, including inverted-binary companions. */
    std::vector<Vref> complete_vars() const {
        LmVrefBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_complete_vars(handle_, &buf));
        return adopt_vrefs(buf);
    }

    // --- mutation ---

    /* Adds a copy of `constr`, returning the name it was stored under.
     * `name == std::nullopt` keeps the constraint's own name. */
    std::string add_constraint(const Constraint &constr,
                               std::optional<std::string> name = std::nullopt) {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(
            lm_cc_add_constraint(handle_, constr.get(), name ? name->c_str() : nullptr, &buf));
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    /* Adds many constraints, each with an optional name, returning the names
     * they were stored under (in order). */
    std::vector<std::string> add_many_constraints(
        const std::vector<std::pair<const Constraint *, std::optional<std::string>>> &constraints) {
        std::vector<std::string> out;
        out.reserve(constraints.size());
        for (const auto &[constr, name] : constraints) {
            out.push_back(add_constraint(*constr, name));
        }
        return out;
    }

    /* Adds all constraints from `other` (optionally prefixing their names),
     * returning the names they were stored under. */
    std::vector<std::string> add_collection(const ConstraintCollection &other,
                                            std::optional<std::string> prefix = std::nullopt) {
        LmStrBuffer buf = {nullptr, 0, 0};
        detail::check(
            lm_cc_add_collection(handle_, other.handle_, prefix ? prefix->c_str() : nullptr, &buf));
        std::vector<std::string> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.emplace_back(buf.data[i]);
        }
        lm_str_buffer_free(buf);
        return out;
    }

    /* Stores `constr` under `key`, replacing any existing entry. */
    void set_constraint(const std::string &key, const Constraint &constr) {
        detail::check(lm_cc_set_constraint(handle_, key.c_str(), constr.get()));
    }

    /* Removes the constraint named `key`. */
    void remove_constraint(const std::string &key) {
        detail::check(lm_cc_remove_constraint(handle_, key.c_str()));
    }

    /* Substitutes `replacement` for `target` in every constraint. */
    void substitute(const Vref &target, const Expression &replacement) {
        detail::check(lm_cc_substitute(handle_, target.get(), replacement.get()));
    }

    /* Evaluates all constraints against a name-keyed sample. */
    std::unordered_map<std::string, bool>
    evaluate_sample(const std::map<std::string, double> &sample,
                    std::optional<double> tol = std::nullopt) const {
        std::vector<const char *> names;
        std::vector<double> values;
        names.reserve(sample.size());
        values.reserve(sample.size());
        for (const auto &[name, value] : sample) {
            names.push_back(name.c_str());
            values.push_back(value);
        }
        LmBoolMapBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_cc_evaluate_sample(handle_, names.data(), values.data(), names.size(),
                                            tol ? &*tol : nullptr, &buffer));
        return read_single_evaluations(buffer);
    }

    /* Evaluates all constraints against an index-aligned dense lookup. */
    std::unordered_map<std::string, bool>
    evaluate_sample_quick(const std::vector<double> &lookup,
                          std::optional<double> tol = std::nullopt) const {
        LmBoolMapBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_cc_evaluate_sample_quick(handle_, lookup.data(), lookup.size(),
                                                  tol ? &*tol : nullptr, &buffer));
        return read_single_evaluations(buffer);
    }

    // --- copying / comparison / serialization ---

    /* Clones into a new handle sharing the environment. */
    ConstraintCollection clone() const {
        LmCC *out = nullptr;
        detail::check(lm_cc_clone(handle_, &out));
        return ConstraintCollection(out);
    }

    /* Deep-clones into an independent collection rebound to `env`. */
    ConstraintCollection deep_clone(const Env &env) const {
        LmCC *out = nullptr;
        detail::check(lm_cc_deep_clone(handle_, env.get(), &out));
        return ConstraintCollection(out);
    }

    bool operator==(const ConstraintCollection &other) const {
        bool out = false;
        detail::check(lm_cc_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const ConstraintCollection &other) const { return !(*this == other); }

    /* Content equality, ignoring environment identity. */
    bool equal_contents(const ConstraintCollection &other) const {
        bool out = false;
        detail::check(lm_cc_equal_contents(handle_, other.handle_, &out));
        return out;
    }

    /* Serializes the collection to bytes. */
    std::vector<std::uint8_t> serialize() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_serialize(handle_, &buf));
        std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
        lm_buffer_free(buf);
        return bytes;
    }

    /* Deserializes a collection from bytes, rebinding it to `env`. */
    static ConstraintCollection deserialize(const std::uint8_t *data, std::size_t len,
                                            const Env &env) {
        LmCC *out = nullptr;
        detail::check(lm_cc_deserialize(data, len, env.get(), &out));
        return ConstraintCollection(out);
    }
    static ConstraintCollection deserialize(const std::vector<std::uint8_t> &bytes,
                                            const Env &env) {
        return deserialize(bytes.data(), bytes.size(), env);
    }

    LmCC *get() const noexcept { return handle_; }
    LmCC *release() noexcept {
        LmCC *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    static std::unordered_map<std::string, bool> read_single_evaluations(LmBoolMapBuffer buffer) {
        std::unordered_map<std::string, bool> out;
        for (std::size_t index = 0; index < buffer.len; ++index) {
            const LmBoolMapEntry &entry = buffer.data[index];
            if (entry.len != 0) {
                out.emplace(entry.key, entry.data[0]);
            }
        }
        lm_bool_map_buffer_free(buffer);
        return out;
    }

    /* Adopts an owned vref buffer into `Vref`s and releases the outer array. */
    static std::vector<Vref> adopt_vrefs(LmVrefBuffer buf) {
        std::vector<Vref> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.emplace_back(buf.data[i]); // Vref adopts the handle,
            buf.data[i] = nullptr;         // so the buffer must not free it.
        }
        lm_vref_buffer_free(buf);
        return out;
    }

    LmCC *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_CONSTRAINT_COLLECTION_HPP
