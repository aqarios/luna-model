#ifndef LUNA_MODEL_ERROR_HPP
#define LUNA_MODEL_ERROR_HPP

#include <stdexcept>
#include <string>
#include <utility>

#include <luna_model.h>

namespace lm {

/* Exception carrying a C API status code and the LunaModel error message. */
class Error : public std::runtime_error {
  public:
    Error(LmStatus status, std::string message)
        : std::runtime_error(std::move(message)), status_(status) {}

    LmStatus status() const noexcept { return status_; }

  private:
    LmStatus status_;
};

} // namespace lm

#endif // LUNA_MODEL_ERROR_HPP
