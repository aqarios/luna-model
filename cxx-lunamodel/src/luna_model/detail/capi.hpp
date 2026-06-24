#ifndef LUNA_MODEL_DETAIL_CAPI_HPP
#define LUNA_MODEL_DETAIL_CAPI_HPP

#include <string>

#include <luna_model.h>

#include "luna_model/error.hpp"

namespace lm {
namespace detail {

/* Throws an lm::Error built from the thread-local last-error message. */
[[noreturn]] inline void raise(LmStatus status) {
    const char *msg = lm_last_error_message();
    throw Error(status, msg ? std::string(msg) : std::string("unknown LunaModel error"));
}

inline void check(LmStatus status) {
    if (status != LmStatus_Ok) {
        raise(status);
    }
}

} // namespace detail
} // namespace lm

#endif // LUNA_MODEL_DETAIL_CAPI_HPP
