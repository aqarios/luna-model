#ifndef LUNA_MODEL_DETAIL_CAPI_HPP
#define LUNA_MODEL_DETAIL_CAPI_HPP

#include <exception>
#include <string>

#include <luna_model.h>

#include "luna_model/error.hpp"

namespace lm {
namespace detail {

/* Throws an lm::Error built from the thread-local last structured error. */
[[noreturn]] inline void raise(LmStatus status) {
    LmError *err = lm_last_error();
    if (err == nullptr) {
        throw Error(status, std::string("unknown LunaModel error"));
    }
    std::string message("unknown LunaModel error");
    LmBuffer buf = {nullptr, 0, 0};
    if (lm_error_message(err, &buf) == LmStatus_Ok) {
        message.assign(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
    }
    lm_error_free(err);
    throw Error(status, message);
}

inline void check(LmStatus status) {
    if (status != LmStatus_Ok) {
        raise(status);
    }
}

/* A stable kind tag for a status code, used to populate the reported error's kind when a plugin
 * author's exception doesn't otherwise carry a more specific one. */
inline const char *status_kind(LmStatus status) {
    switch (status) {
        case LmStatus_NullPointer:
            return "null_pointer";
        case LmStatus_InvalidArgument:
            return "invalid_argument";
        case LmStatus_Serialization:
            return "serialization";
        case LmStatus_Deserialization:
            return "deserialization";
        case LmStatus_Panic:
            return "panic";
        case LmStatus_Absent:
            return "absent";
        case LmStatus_Ok:
        case LmStatus_Errored:
        default:
            return "errored";
    }
}

/* Reports the currently-active exception to the host via `lm_set_last_error` before a plugin
 * trampoline returns its status, so the exception's message crosses the ABI instead of being
 * silently discarded by a bare `catch (...)`. Must be called from inside a `catch` block.
 * Returns the status code the trampoline should return. */
inline LmStatus report_exception() {
    try {
        throw;
    } catch (const lm::Error &e) {
        lm_set_last_error(status_kind(e.status()), e.what(), nullptr);
        return e.status() == LmStatus_Ok ? LmStatus_Errored : e.status();
    } catch (const std::exception &e) {
        lm_set_last_error("errored", e.what(), nullptr);
        return LmStatus_Errored;
    } catch (...) {
        lm_set_last_error("errored", "unknown C++ exception", nullptr);
        return LmStatus_Errored;
    }
}

} // namespace detail
} // namespace lm

#endif // LUNA_MODEL_DETAIL_CAPI_HPP
