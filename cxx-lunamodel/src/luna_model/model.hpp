#ifndef LUNA_MODEL_MODEL_HPP
#define LUNA_MODEL_MODEL_HPP

#include <cstddef>
#include <cstdint>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/error.hpp"

namespace lm {

/* Owning handle to a LunaModel model (frees on destruction). */
class Model {
  public:
    Model() : handle_(lm_model_new()) {
        if (handle_ == nullptr) {
            throw Error(LmStatus_Panic, "failed to allocate a LunaModel model");
        }
    }

    explicit Model(LmModel *handle) noexcept : handle_(handle) {}

    ~Model() {
        if (handle_ != nullptr) {
            lm_model_free(handle_);
        }
    }

    Model(Model &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Model &operator=(Model &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_model_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Model(const Model &) = delete;
    Model &operator=(const Model &) = delete;

    /* Deep-clone into an independent model. */
    Model clone() const {
        LmModel *out = nullptr;
        detail::check(lm_model_clone(handle_, &out));
        return Model(out);
    }

    /* Serialize the model to bytes. */
    std::vector<std::uint8_t> serialize() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_model_serialize(handle_, &buf));
        std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
        lm_buffer_free(buf);
        return bytes;
    }

    /* Deserialize a model from bytes. */
    static Model deserialize(const std::uint8_t *data, std::size_t len) {
        LmModel *out = nullptr;
        detail::check(lm_model_deserialize(data, len, &out));
        return Model(out);
    }
    static Model deserialize(const std::vector<std::uint8_t> &bytes) {
        return deserialize(bytes.data(), bytes.size());
    }

    LmModel *get() const noexcept { return handle_; }
    LmModel *release() noexcept {
        LmModel *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmModel *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_MODEL_HPP
