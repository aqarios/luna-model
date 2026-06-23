// LunaModel C++ API — a header-only RAII wrapper over the LunaModel C API.
//
// It mirrors the Rust/Python API shape (a `Model` you build, serialize, and
// deserialize), trading C status codes for RAII handles and exceptions.
//
// Works in both modes of the underlying C header:
//   * Standalone (default): link liblunamodel_capi and just include this
//   header.
//         #include <luna_model.hpp>
//         lm::Model model;
//         std::vector<std::uint8_t> bytes = model.serialize();
//   * Python extension: define LUNA_MODEL_PYTHON_EXTENSION (and include
//   <Python.h>) before this header, then call lm::import() once during module
//   init.
#ifndef LUNA_MODEL_HPP
#define LUNA_MODEL_HPP

#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include <luna_model.h> // the dual-mode C API

namespace lm {

/// Exception carrying a C API status code and the LunaModel error message.
class Error : public std::runtime_error {
public:
  Error(LmStatus status, std::string message)
      : std::runtime_error(std::move(message)), status_(status) {}
  LmStatus status() const noexcept { return status_; }

private:
  LmStatus status_;
};

namespace detail {

#ifdef LUNA_MODEL_PYTHON_EXTENSION
/// Returns the capsule-imported C API table, throwing if import() was not
/// called.
inline const LunaModelCApi &api() {
  const LunaModelCApi *table = lunamodel_api();
  if (table == nullptr) {
    throw Error(
        LmStatus_InvalidArgument,
        "LunaModel C API not imported; call lm::import() in module init");
  }
  return *table;
}
#define LM_CALL(fn) (lm::detail::api().fn)
#else
#define LM_CALL(fn) (lm_##fn)
#endif

/// Throws an Error built from the thread-local last-error message.
[[noreturn]] inline void raise(LmStatus status) {
  const char *msg = LM_CALL(last_error_message)();
  throw Error(status,
              msg ? std::string(msg) : std::string("unknown LunaModel error"));
}

/// Converts a non-Ok status into an exception.
inline void check(LmStatus status) {
  if (status != LmStatus_Ok) {
    raise(status);
  }
}

} // namespace detail

/// Owning handle to a LunaModel model (frees on destruction).
class Model {
public:
  /// Creates a new, empty model.
  Model() : handle_(LM_CALL(model_new)()) {
    if (handle_ == nullptr) {
      throw Error(LmStatus_Panic, "failed to allocate a LunaModel model");
    }
  }

  /// Adopts ownership of a raw handle (e.g. one obtained from the C API).
  explicit Model(LmModel *handle) noexcept : handle_(handle) {}

  ~Model() {
    if (handle_ != nullptr) {
      LM_CALL(model_free)(handle_);
    }
  }

  Model(Model &&other) noexcept : handle_(other.handle_) {
    other.handle_ = nullptr;
  }
  Model &operator=(Model &&other) noexcept {
    if (this != &other) {
      if (handle_ != nullptr) {
        LM_CALL(model_free)(handle_);
      }
      handle_ = other.handle_;
      other.handle_ = nullptr;
    }
    return *this;
  }
  Model(const Model &) = delete;
  Model &operator=(const Model &) = delete;

  /// Deep-clones into an independent model.
  Model clone() const {
    LmModel *out = nullptr;
    detail::check(LM_CALL(model_clone)(handle_, &out));
    return Model(out);
  }

  /// Serializes the model to bytes.
  std::vector<std::uint8_t> serialize() const {
    LmBuffer buf{nullptr, 0, 0};
    detail::check(LM_CALL(model_serialize)(handle_, &buf));
    std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
    LM_CALL(buffer_free)(buf);
    return bytes;
  }

  /// Deserializes a model from bytes.
  static Model deserialize(const std::uint8_t *data, std::size_t len) {
    LmModel *out = nullptr;
    detail::check(LM_CALL(model_deserialize)(data, len, &out));
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

/// Owning handle to a LunaModel solution (frees on destruction).
class Solution {
public:
  explicit Solution(LmSolution *handle) noexcept : handle_(handle) {}
  ~Solution() {
    if (handle_ != nullptr) {
      LM_CALL(solution_free)(handle_);
    }
  }

  Solution(Solution &&other) noexcept : handle_(other.handle_) {
    other.handle_ = nullptr;
  }
  Solution &operator=(Solution &&other) noexcept {
    if (this != &other) {
      if (handle_ != nullptr) {
        LM_CALL(solution_free)(handle_);
      }
      handle_ = other.handle_;
      other.handle_ = nullptr;
    }
    return *this;
  }
  Solution(const Solution &) = delete;
  Solution &operator=(const Solution &) = delete;

  static Solution deserialize(const std::uint8_t *data, std::size_t len) {
    LmSolution *out = nullptr;
    detail::check(LM_CALL(solution_deserialize)(data, len, &out));
    return Solution(out);
  }
  static Solution deserialize(const std::vector<std::uint8_t> &bytes) {
    return deserialize(bytes.data(), bytes.size());
  }

  LmSolution *get() const noexcept { return handle_; }

private:
  LmSolution *handle_;
};

/// Returns the LunaModel library version string.
inline std::string version() { return std::string(LM_CALL(version)()); }

#ifdef LUNA_MODEL_PYTHON_EXTENSION
/// Imports the C API from the installed luna_model wheel. Call once during your
/// extension module's init; returns 0 on success, -1 on failure (Python error
/// set).
inline int import() { return import_luna_model(); }
#endif

} // namespace lm

#undef LM_CALL

#endif // LUNA_MODEL_HPP
