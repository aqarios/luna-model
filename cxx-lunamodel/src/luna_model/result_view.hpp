#ifndef LUNA_MODEL_RESULT_VIEW_HPP
#define LUNA_MODEL_RESULT_VIEW_HPP

#include <cstddef>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/sample_view.hpp"

namespace lm {

/* Result metadata for one solution sample row.
 *
 * A value snapshot read eagerly from a borrowed `LmResultView` handle. Scalar
 * fields are `std::nullopt` when the corresponding quantity was not evaluated;
 * `constraints` / `variable_bounds` are empty when unevaluated. `sample` owns
 * the row's `SampleView`.
 */
struct ResultView {
    std::size_t counts = 0;
    std::optional<double> obj_value;
    std::optional<double> raw_energy;
    std::optional<bool> feasible;
    std::map<std::string, std::vector<bool>> constraints;
    std::map<std::string, std::vector<bool>> variable_bounds;
    std::optional<SampleView> sample;
};

namespace detail {

/* Reads a bool-map buffer into a `std::map`, releasing the buffer. */
inline std::map<std::string, std::vector<bool>> read_bool_map(LmBoolMapBuffer buf) {
    std::map<std::string, std::vector<bool>> out;
    for (std::size_t i = 0; i < buf.len; ++i) {
        const LmBoolMapEntry &e = buf.data[i];
        out.emplace(std::string(e.key), std::vector<bool>(e.data, e.data + e.len));
    }
    lm_bool_map_buffer_free(buf);
    return out;
}

/* Reads an owned `LmResultView` handle into a value `ResultView`.
 *
 * Does not free the handle; the caller (or its buffer) retains ownership. */
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

    LmBoolMapBuffer cbuf = {nullptr, 0, 0};
    check(lm_result_view_constraints(view, &cbuf));
    out.constraints = read_bool_map(cbuf);

    LmBoolMapBuffer vbuf = {nullptr, 0, 0};
    check(lm_result_view_variable_bounds(view, &vbuf));
    out.variable_bounds = read_bool_map(vbuf);

    LmSampleView *sv = nullptr;
    st = lm_result_view_sample(view, &sv);
    if (st != LmStatus_Absent) {
        check(st);
        out.sample.emplace(sv);
    }

    return out;
}

} // namespace detail
} // namespace lm

#endif // LUNA_MODEL_RESULT_VIEW_HPP
