#ifndef LUNA_MODEL_RESULT_VIEW_HPP
#define LUNA_MODEL_RESULT_VIEW_HPP

#include <cstddef>
#include <optional>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* Result metadata for one solution sample row (Rust `ResultView`).
 *
 * A plain value snapshot: the C `LmResultView` handle borrows its solution and
 * is owned by the buffer it came from, so the fields are read out eagerly rather
 * than holding the handle. `obj_value` / `raw_energy` / `feasible` are
 * `std::nullopt` when the corresponding field was not evaluated.
 */
struct ResultView {
    std::size_t counts = 0;
    std::optional<double> obj_value;
    std::optional<double> raw_energy;
    std::optional<bool> feasible;
};

namespace detail {

/* Reads an owned `LmResultView` handle into a value `ResultView`.
 *
 * Does not free the handle; the caller (or its buffer) retains ownership.
 */
inline ResultView read_result_view(const LmResultView *view) {
    ResultView out;
    check(lm_result_view_counts(view, &out.counts));

    double d = 0.0;
    LmStatus st = lm_result_view_obj_value(view, &d);
    if (st != LmStatus_Absent) {
        check(st);
        out.obj_value = d;
    }

    st = lm_result_view_raw_energy(view, &d);
    if (st != LmStatus_Absent) {
        check(st);
        out.raw_energy = d;
    }

    bool b = false;
    st = lm_result_view_feasible(view, &b);
    if (st != LmStatus_Absent) {
        check(st);
        out.feasible = b;
    }

    return out;
}

} // namespace detail
} // namespace lm

#endif // LUNA_MODEL_RESULT_VIEW_HPP
