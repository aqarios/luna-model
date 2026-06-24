#ifndef LUNA_MODEL_SOLUTION_HPP
#define LUNA_MODEL_SOLUTION_HPP

#include <cstddef>
#include <cstdint>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* Owning handle to a LunaModel solution (frees on destruction). */
class Solution {
  public:
    explicit Solution(LmSolution *handle) noexcept : handle_(handle) {}

    ~Solution() {
        if (handle_ != nullptr) {
            lm_solution_free(handle_);
        }
    }

    Solution(Solution &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Solution &operator=(Solution &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_solution_free(handle_);
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
        detail::check(lm_solution_deserialize(data, len, &out));
        return Solution(out);
    }
    static Solution deserialize(const std::vector<std::uint8_t> &bytes) {
        return deserialize(bytes.data(), bytes.size());
    }

    LmSolution *get() const noexcept { return handle_; }

  private:
    LmSolution *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_SOLUTION_HPP
