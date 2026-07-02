#ifndef LUNA_MODEL_CONSTRAINT_COLLECTION_HPP
#define LUNA_MODEL_CONSTRAINT_COLLECTION_HPP

#include <luna_model.h>
#include <vector>

#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/error.hpp"
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
    std::vector<Vref> compelte_vars() const {
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

  private:
    LmCC *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_CONSTRAINT_COLLECTION_HPP
