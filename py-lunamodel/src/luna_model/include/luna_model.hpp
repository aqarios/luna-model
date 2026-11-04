/*
 * LunaModel C++ API — header-only RAII wrapper over the LunaModel C API.
 *
 * Standalone: link liblunamodel; add cxx-lunamodel/src and dist/c/include to your
 * include path.
 *     #include <luna_model.hpp>
 *     lm::Model model;
 *     std::vector<std::uint8_t> bytes = model.serialize();
 *
 * Python extension: define LUNA_MODEL_PYTHON_EXTENSION before including, then call
 * import_luna_model() once during module init (lm_* then dispatch through the
 * wheel's _C_API capsule, same call syntax).
 */
#ifndef LUNA_MODEL_HPP
#define LUNA_MODEL_HPP

#include "luna_model/bound.hpp"                 // IWYU pragma: export
#include "luna_model/bounds.hpp"                // IWYU pragma: export
#include "luna_model/column.hpp"                // IWYU pragma: export
#include "luna_model/constraint.hpp"            // IWYU pragma: export
#include "luna_model/constraint_collection.hpp" // IWYU pragma: export
#include "luna_model/env.hpp"                   // IWYU pragma: export
#include "luna_model/error.hpp"                 // IWYU pragma: export
#include "luna_model/expr.hpp"                  // IWYU pragma: export
#include "luna_model/higher_order.hpp"          // IWYU pragma: export
#include "luna_model/lazy_bounds.hpp"           // IWYU pragma: export
#include "luna_model/linear.hpp"                // IWYU pragma: export
#include "luna_model/model.hpp"                 // IWYU pragma: export
#include "luna_model/ops.hpp"                   // IWYU pragma: export
#include "luna_model/quadratic.hpp"             // IWYU pragma: export
#include "luna_model/result_view.hpp"           // IWYU pragma: export
#include "luna_model/sample_view.hpp"           // IWYU pragma: export
#include "luna_model/sol_lookup.hpp"             // IWYU pragma: export
#include "luna_model/solution.hpp"              // IWYU pragma: export
#include "luna_model/specs.hpp"                 // IWYU pragma: export
#include "luna_model/subtimer.hpp"              // IWYU pragma: export
#include "luna_model/timer.hpp"                 // IWYU pragma: export
#include "luna_model/timing.hpp"                // IWYU pragma: export
#include "luna_model/variable.hpp"              // IWYU pragma: export
#include "luna_model/version.hpp"               // IWYU pragma: export
#include "luna_model/vref.hpp"                  // IWYU pragma: export

#endif // LUNA_MODEL_HPP
