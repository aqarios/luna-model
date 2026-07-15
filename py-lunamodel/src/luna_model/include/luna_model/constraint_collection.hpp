#ifndef LUNA_MODEL_CONSTRAINT_COLLECTION_HPP
#define LUNA_MODEL_CONSTRAINT_COLLECTION_HPP

#include <cstddef>
#include <cstdint>
#include <luna_model.h>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>

#include "luna_model/constraint.hpp"
#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/error.hpp"
#include "luna_model/expr.hpp"
#include "luna_model/vref.hpp"

namespace lm {

class ConstraintCollection {
  public:
    /* Creates an empty ConstraintCollection (Rust `ConstraintCollection::default()`). */
    ConstraintCollection() : handle_(lm_cc_default()) {
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

    /* Rust `Clone`. */
    ConstraintCollection clone() const {
        LmCC *out = nullptr;
        detail::check(lm_cc_clone(handle_, &out));
        return ConstraintCollection(out);
    }

    /* Deep-clones into an independent collection whose constraints are rebound
     * to `env` (Rust `cc.deep_clone(env)`).
     */
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

    bool equal_contents(const ConstraintCollection &other) const {
        bool out = false;
        detail::check(lm_cc_equal_contents(handle_, other.handle_, &out));
        return out;
    }

    /* Returns the distinct variable types used by the collection
     * (Rust `cc.vtypes()`). */
    std::vector<LmVtype> vtypes() const {
        LmVtypeBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_vtypes(handle_, &buf));
        std::vector<LmVtype> out(buf.data, buf.data + buf.len);
        lm_vtype_buffer_free(buf);
        return out;
    }

    /* Returns the distinct constraint types used by the collection
     * (Rust `cc.ctypes()`). */
    std::vector<LmComparator> ctypes() const {
        LmCmpBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_ctypes(handle_, &buf));
        std::vector<LmComparator> out(buf.data, buf.data + buf.len);
        lm_cmp_buffer_free(buf);
        return out;
    }

    /* Returns the distinct variables used by the constraint-collection
     * (Rust `cc.vars()`). */
    std::vector<Vref> vars() const {
        LmVrefBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_vars(handle_, &buf));
        std::vector<Vref> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.emplace_back(buf.data[i]); // Vref adopts ownership of the handle,
            buf.data[i] = nullptr;         // so the buffer must not free it.
        }
        lm_vref_buffer_free(buf); // frees only the outer array (skips nulls)
        return out;
    }

    /* Returns the distinct variables used by the constraint-collection including inverted binary
     * references. (Rust `cc.complete_vars()`). */
    std::vector<Vref> complete_vars() const {
        LmVrefBuffer buf = {nullptr, 0, 0};
        detail::check(lm_cc_complete_vars(handle_, &buf));
        std::vector<Vref> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.emplace_back(buf.data[i]); // Vref adopts ownership of the handle,
            buf.data[i] = nullptr;         // so the buffer must not free it.
        }
        lm_vref_buffer_free(buf); // frees only the outer array (skips nulls)
        return out;
    }

    bool is_empty() const {
        bool out = false;
        detail::check(lm_cc_is_empty(handle_, &out));
        return out;
    }
    bool empty() const { return is_empty(); }

    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_cc_len(handle_, &out));
        return out;
    }

    std::vector<std::string> names() const {
        LmStrBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_cc_names(handle_, &buffer));
        std::vector<std::string> out;
        out.reserve(buffer.len);
        for (std::size_t index = 0; index < buffer.len; ++index) {
            out.emplace_back(buffer.data[index]);
        }
        lm_str_buffer_free(buffer);
        return out;
    }

    bool contains(const std::string &name) const {
        bool out = false;
        detail::check(lm_cc_contains(handle_, name.c_str(), &out));
        return out;
    }

    Constraint get(const std::string &name) const {
        LmConstraint *out = nullptr;
        detail::check(lm_cc_get(handle_, name.c_str(), &out));
        return Constraint(out);
    }

    std::string add_constraint(const Constraint &constraint,
                               std::optional<std::string> name = std::nullopt) {
        LmBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_cc_add_constraint(handle_, constraint.get(),
                                           name ? name->c_str() : nullptr, &buffer));
        std::string out =
            buffer.len == 0 ? std::string()
                            : std::string(reinterpret_cast<const char *>(buffer.data), buffer.len);
        lm_buffer_free(buffer);
        return out;
    }

    std::vector<std::string> add_collection(const ConstraintCollection &other,
                                            std::optional<std::string> prefix = std::nullopt) {
        LmStrBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_cc_add_collection(handle_, other.handle_,
                                           prefix ? prefix->c_str() : nullptr, &buffer));
        std::vector<std::string> out;
        out.reserve(buffer.len);
        for (std::size_t index = 0; index < buffer.len; ++index) {
            out.emplace_back(buffer.data[index]);
        }
        lm_str_buffer_free(buffer);
        return out;
    }

    void set_constraint(const std::string &name, const Constraint &constraint) {
        detail::check(lm_cc_set_constraint(handle_, name.c_str(), constraint.get()));
    }

    void remove_constraint(const std::string &name) {
        detail::check(lm_cc_remove_constraint(handle_, name.c_str()));
    }

    void substitute(const Vref &target, const Expression &replacement) {
        detail::check(lm_cc_substitute(handle_, target.get(), replacement.get()));
    }

    std::unordered_map<std::string, bool>
    evaluate_sample_quick(const std::vector<double> &lookup,
                          std::optional<double> tol = std::nullopt) const {
        LmBoolMapBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_cc_evaluate_sample_quick(handle_, lookup.data(), lookup.size(),
                                                  tol ? &*tol : nullptr, &buffer));
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

    std::vector<std::uint8_t> serialize() const {
        LmBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_cc_serialize(handle_, &buffer));
        std::vector<std::uint8_t> out(buffer.data, buffer.data + buffer.len);
        lm_buffer_free(buffer);
        return out;
    }

    static ConstraintCollection deserialize(const std::uint8_t *data, std::size_t len,
                                            const Env &env) {
        LmCC *out = nullptr;
        detail::check(lm_cc_deserialize(data, len, env.get(), &out));
        return ConstraintCollection(out);
    }
    static ConstraintCollection deserialize(const std::vector<std::uint8_t> &data, const Env &env) {
        return deserialize(data.data(), data.size(), env);
    }

    LmCC *get() const noexcept { return handle_; }
    LmCC *release() noexcept {
        LmCC *handle = handle_;
        handle_ = nullptr;
        return handle;
    }

  private:
    LmCC *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_CONSTRAINT_COLLECTION_HPP
