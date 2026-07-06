#ifndef LUNA_MODEL_SOLUTION_HPP
#define LUNA_MODEL_SOLUTION_HPP

#include <cstddef>
#include <cstdint>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/column.hpp"
#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/error.hpp"
#include "luna_model/result_view.hpp"
#include "luna_model/timing.hpp"

namespace lm {

/* One `(variable name, column)` entry of a solution's sample map, carrying its
 * position in the underlying `IndexMap` (Rust `Solution::samples` entry). */
struct SampleColumn {
    std::size_t index;
    std::string key;
    Column column;
};

/* Owning handle to a LunaModel solution (frees on destruction).
 *
 * Mirrors the Rust `Solution`: move-only, with accessors for the stored sample
 * data and per-sample metadata.
 */
class Solution {
  public:
    /* Creates an empty minimization solution (Rust `Solution::default()`). */
    Solution() : handle_(lm_sol_default()) {
        if (handle_ == nullptr) {
            throw Error(LmStatus_Panic, "failed to allocate a LunaModel solution");
        }
    }

    /* Creates an empty solution for a given sense (Rust `Solution::with_sense`). */
    explicit Solution(LmSense sense) : handle_(lm_sol_with_sense(sense)) {
        if (handle_ == nullptr) {
            throw Error(LmStatus_Panic, "failed to allocate a LunaModel solution");
        }
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Solution(LmSol *handle) noexcept : handle_(handle) {}

    ~Solution() {
        if (handle_ != nullptr) {
            lm_sol_free(handle_);
        }
    }

    Solution(Solution &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Solution &operator=(Solution &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_sol_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Solution(const Solution &) = delete;
    Solution &operator=(const Solution &) = delete;

    /* Creates an empty solution with reserved capacity (Rust `with_capacity`). */
    static Solution with_capacity(std::size_t capacity) {
        LmSol *handle = lm_sol_with_capcity(capacity);
        if (handle == nullptr) {
            throw Error(LmStatus_Panic, "failed to allocate a LunaModel solution");
        }
        return Solution(handle);
    }

    /* Builds an empty solution shaped from an environment (Rust `from_env`). */
    static Solution from_env(const Env &env) {
        LmSol *out = nullptr;
        detail::check(lm_sol_from_env(env.get(), &out));
        return Solution(out);
    }

    /* Builds a random solution (Rust `Solution::from_random`).
     *
     * `context` / `model` are optional raw handles used to shape and evaluate the
     * samples; pass `nullptr` (or `env.get()` / `model.get()`) as needed.
     */
    static Solution from_random(std::size_t n_samples,
                                std::optional<std::uint64_t> seed = std::nullopt,
                                const LmEnv *context = nullptr, const LmModel *model = nullptr,
                                std::optional<LmSense> sense = std::nullopt,
                                std::optional<double> tol = std::nullopt) {
        LmSol *out = nullptr;
        detail::check(lm_sol_from_random(n_samples, seed ? &*seed : nullptr, context, model,
                                         sense ? &*sense : nullptr, tol ? &*tol : nullptr, &out));
        return Solution(out);
    }

    /* Number of unique samples (Rust `Solution::n_samples`). */
    std::size_t n_samples() const {
        std::size_t out = 0;
        detail::check(lm_sol_n_samples(handle_, &out));
        return out;
    }

    /* Number of sample rows (Rust `Solution::len`). */
    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_sol_len(handle_, &out));
        return out;
    }

    /* Whether the solution has no sample rows (Rust `Solution::is_empty`). */
    bool is_empty() const {
        bool out = false;
        detail::check(lm_sol_is_empty(handle_, &out));
        return out;
    }
    bool empty() const { return is_empty(); }

    /* Number of variables per sample row (Rust `Solution::sample_len`). */
    std::size_t sample_len() const {
        std::size_t out = 0;
        detail::check(lm_sol_sample_len(handle_, &out));
        return out;
    }

    /* Raw stored value for `var` in `sample`, or `std::nullopt` if absent
     * (Rust `Solution::value`). */
    std::optional<double> value(std::size_t sample, const std::string &var) const {
        double out = 0.0;
        LmStatus st = lm_sol_value(handle_, sample, var.c_str(), &out);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return out;
    }

    /* Typed assignment for `var` in `sample` (Rust `Solution::try_assignment`).
     * Throws `lm::Error` when the sample or variable is absent. */
    LmAssignment assignment(std::size_t sample, const std::string &var) const {
        LmAssignment out;
        detail::check(lm_sol_assignment(handle_, sample, var.c_str(), &out));
        return out;
    }

    /* Typed assignment by column position (Rust `Solution::try_assignment_idx`). */
    LmAssignment assignment_idx(std::size_t sample, std::size_t var_index) const {
        LmAssignment out;
        detail::check(lm_sol_assignment_idx(handle_, sample, var_index, &out));
        return out;
    }

    /* Stored variable names in column order (Rust `Solution::variable_names`). */
    std::vector<std::string> variable_names() const {
        LmStrBuffer buf = {nullptr, 0, 0};
        detail::check(lm_sol_variable_names(handle_, &buf));
        std::vector<std::string> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.emplace_back(buf.data[i]);
        }
        lm_str_buffer_free(buf);
        return out;
    }

    /* A copy of the column stored for `var` (Rust `Solution::samples[var]`). */
    Column column(const std::string &var) const {
        LmColumn *out = nullptr;
        detail::check(lm_sol_column(handle_, var.c_str(), &out));
        return Column(out);
    }

    /* The `(index, name, column)` entries of the sample map (Rust `samples`). */
    std::vector<SampleColumn> samples() const {
        LmIndexMapItemBuffer buf = {nullptr, 0, 0};
        detail::check(lm_sol_samples(handle_, &buf));
        std::vector<SampleColumn> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            LmIndexMapItem &item = buf.data[i];
            out.push_back(SampleColumn{item.index, std::string(item.key), Column(item.value)});
            // Ownership of the column moved into the C++ handle; stop the buffer
            // free from also freeing it.
            item.value = nullptr;
        }
        lm_index_map_buffer_free(buf);
        return out;
    }

    /* Per-sample occurrence counts (Rust `Solution::counts`). */
    std::vector<std::size_t> counts() const {
        const std::size_t *data = nullptr;
        std::size_t len = 0;
        detail::check(lm_sol_counts(handle_, &data, &len));
        return std::vector<std::size_t>(data, data + len);
    }

    /* Raw solver energies, or `std::nullopt` if unevaluated (Rust `raw_energies`). */
    std::optional<std::vector<double>> raw_energies() const {
        const double *data = nullptr;
        std::size_t len = 0;
        LmStatus st = lm_sol_raw_energies(handle_, &data, &len);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return std::vector<double>(data, data + len);
    }

    /* Model-evaluated objective values, or `std::nullopt` (Rust `obj_values`). */
    std::optional<std::vector<double>> obj_values() const {
        const double *data = nullptr;
        std::size_t len = 0;
        LmStatus st = lm_sol_obj_values(handle_, &data, &len);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return std::vector<double>(data, data + len);
    }

    /* Per-sample feasibility flags, or `std::nullopt` (Rust `feasible`). */
    std::optional<std::vector<bool>> feasible() const {
        const bool *data = nullptr;
        std::size_t len = 0;
        LmStatus st = lm_sol_feasible(handle_, &data, &len);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return std::vector<bool>(data, data + len);
    }

    /* Per-constraint feasibility flags keyed by name (Rust `constraints`). */
    std::map<std::string, std::vector<bool>> constraints() const {
        LmBoolMapBuffer buf = {nullptr, 0, 0};
        detail::check(lm_sol_constraints(handle_, &buf));
        std::map<std::string, std::vector<bool>> out;
        for (std::size_t i = 0; i < buf.len; ++i) {
            const LmBoolMapEntry &e = buf.data[i];
            out.emplace(std::string(e.key), std::vector<bool>(e.data, e.data + e.len));
        }
        lm_bool_map_buffer_free(buf);
        return out;
    }

    /* Per-variable bound-check flags keyed by name (Rust `variable_bounds`). */
    std::map<std::string, std::vector<bool>> variable_bounds() const {
        LmBoolMapBuffer buf = {nullptr, 0, 0};
        detail::check(lm_sol_variable_bounds(handle_, &buf));
        std::map<std::string, std::vector<bool>> out;
        for (std::size_t i = 0; i < buf.len; ++i) {
            const LmBoolMapEntry &e = buf.data[i];
            out.emplace(std::string(e.key), std::vector<bool>(e.data, e.data + e.len));
        }
        lm_bool_map_buffer_free(buf);
        return out;
    }

    /* Timing metadata, or `std::nullopt` if unset (Rust `Solution::timing`). */
    std::optional<Timing> timing() const {
        LmTiming *out = nullptr;
        LmStatus st = lm_sol_timing(handle_, &out);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return Timing(out);
    }

    /* Optimization sense the solution was created with (Rust `Solution::sense`). */
    LmSense sense() const {
        LmSense out;
        detail::check(lm_sol_sense(handle_, &out));
        return out;
    }

    /* Result view for the row at `index`, or `std::nullopt` if out of range
     * (Rust `Solution::result`). */
    std::optional<ResultView> result(std::size_t index) const {
        LmResultView *view = nullptr;
        LmStatus st = lm_sol_result(handle_, index, &view);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        ResultView out = detail::read_result_view(view);
        lm_result_view_free(view);
        return out;
    }

    /* Result views over all rows (Rust `Solution::results`). */
    std::vector<ResultView> results() const {
        LmResultViewBuffer buf = {nullptr, 0, 0};
        detail::check(lm_sol_results(handle_, &buf));
        std::vector<ResultView> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.push_back(detail::read_result_view(buf.data[i]));
        }
        lm_result_view_buffer_free(buf);
        return out;
    }

    /* Best feasible result views, or `std::nullopt` when unavailable
     * (Rust `Solution::best`). */
    std::optional<std::vector<ResultView>> best() const {
        LmResultViewBuffer buf = {nullptr, 0, 0};
        LmStatus st = lm_sol_best(handle_, &buf);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        std::vector<ResultView> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.push_back(detail::read_result_view(buf.data[i]));
        }
        lm_result_view_buffer_free(buf);
        return out;
    }

    /* Conditional value at risk (Rust `Solution::cvar`). */
    double cvar(double alpha, std::optional<LmValueSource> toggle = std::nullopt) const {
        double out = 0.0;
        detail::check(lm_sol_cvar(handle_, alpha, toggle ? &*toggle : nullptr, &out));
        return out;
    }

    /* Expectation value of the selected series (Rust `Solution::expectation_value`). */
    double expectation_value(std::optional<LmValueSource> toggle = std::nullopt) const {
        double out = 0.0;
        detail::check(lm_sol_expectation_value(handle_, toggle ? &*toggle : nullptr, &out));
        return out;
    }

    /* Temperature-weighted average (Rust `Solution::temperature_weighted`). */
    double temperature_weighted(double beta,
                                std::optional<LmValueSource> toggle = std::nullopt) const {
        double out = 0.0;
        detail::check(
            lm_sol_temperature_weighted(handle_, beta, toggle ? &*toggle : nullptr, &out));
        return out;
    }

    /* Fraction of feasible samples (Rust `Solution::feasibility_ratio`). */
    double feasibility_ratio() const {
        double out = 0.0;
        detail::check(lm_sol_feasibility_ratio(handle_, &out));
        return out;
    }

    /* Name of the most frequently violated constraint, or `std::nullopt`
     * (Rust `Solution::highest_constraint_violations`). */
    std::optional<std::string> highest_constraint_violations() const {
        LmBuffer buf = {nullptr, 0, 0};
        LmStatus st = lm_sol_highest_constraint_violations(handle_, &buf);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    void add_binary(const std::string &name, const std::vector<double> &data,
                    std::optional<double> tol = std::nullopt) {
        detail::check(lm_sol_add_binary(handle_, name.c_str(), data.data(), data.size(),
                                        tol ? &*tol : nullptr));
    }
    void add_spin(const std::string &name, const std::vector<double> &data,
                  std::optional<double> tol = std::nullopt) {
        detail::check(lm_sol_add_spin(handle_, name.c_str(), data.data(), data.size(),
                                      tol ? &*tol : nullptr));
    }
    void add_integer(const std::string &name, const std::vector<double> &data,
                     std::optional<double> tol = std::nullopt) {
        detail::check(lm_sol_add_integer(handle_, name.c_str(), data.data(), data.size(),
                                         tol ? &*tol : nullptr));
    }
    void add_real(const std::string &name, const std::vector<double> &data) {
        detail::check(lm_sol_add_real(handle_, name.c_str(), data.data(), data.size()));
    }

    void add_col(LmVtype vtype, const std::string &name, const std::vector<double> &data,
                 std::optional<double> tol = std::nullopt) {
        detail::check(lm_sol_add_col(handle_, vtype, name.c_str(), data.data(), data.size(),
                                     tol ? &*tol : nullptr));
    }

    void add_empty_binary(const std::string &name) {
        detail::check(lm_sol_add_empty_binary(handle_, name.c_str()));
    }
    void add_empty_spin(const std::string &name) {
        detail::check(lm_sol_add_empty_spin(handle_, name.c_str()));
    }
    void add_empty_integer(const std::string &name) {
        detail::check(lm_sol_add_empty_integer(handle_, name.c_str()));
    }
    void add_empty_real(const std::string &name) {
        detail::check(lm_sol_add_empty_real(handle_, name.c_str()));
    }

    void remove_col(const std::string &name) {
        detail::check(lm_sol_remove_col(handle_, name.c_str()));
    }
    void remove_cols(const std::vector<std::string> &names) {
        std::vector<const char *> ptrs;
        ptrs.reserve(names.size());
        for (const std::string &name : names) {
            ptrs.push_back(name.c_str());
        }
        detail::check(lm_sol_remove_cols(handle_, ptrs.data(), ptrs.size()));
    }

    /* Aggregates duplicate sample rows in place (Rust `Solution::aggregate`). */
    void aggregate() { detail::check(lm_sol_aggregate(handle_)); }

    /* Clones into an independent solution (Rust `Clone`). */
    Solution clone() const {
        LmSol *out = nullptr;
        detail::check(lm_sol_clone(handle_, &out));
        return Solution(out);
    }

    /* Structural equality (Rust `PartialEq`). */
    bool operator==(const Solution &other) const {
        bool out = false;
        detail::check(lm_sol_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Solution &other) const { return !(*this == other); }

    /* Order-independent content equality (Rust `Solution::equal_contents`). */
    bool equal_contents(const Solution &other) const {
        bool out = false;
        detail::check(lm_sol_equal_contents(handle_, other.handle_, &out));
        return out;
    }

    /* Extracts a single row as a standalone one-row solution (Rust `extract`). */
    Solution extract(std::size_t row) const {
        LmSol *out = nullptr;
        detail::check(lm_sol_extract(handle_, row, &out));
        return Solution(out);
    }

    /* Merges solutions sharing the same schema and sense (Rust `merge_many`).
     *
     * `model` is an optional raw handle; when non-null the merged solution is
     * re-evaluated with `tol`.
     */
    static Solution merge_many(const std::vector<const Solution *> &sols,
                               const LmModel *model = nullptr,
                               std::optional<double> tol = std::nullopt) {
        std::vector<const LmSol *> handles;
        handles.reserve(sols.size());
        for (const Solution *sol : sols) {
            handles.push_back(sol->get());
        }
        LmSol *out = nullptr;
        detail::check(
            lm_sol_merge_many(handles.data(), handles.size(), model, tol ? &*tol : nullptr, &out));
        return Solution(out);
    }

    std::vector<std::uint8_t> serialize() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_sol_serialize(handle_, &buf));
        std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
        lm_buffer_free(buf);
        return bytes;
    }

    static Solution deserialize(const std::uint8_t *data, std::size_t len) {
        LmSol *out = nullptr;
        detail::check(lm_sol_deserialize(data, len, &out));
        return Solution(out);
    }
    static Solution deserialize(const std::vector<std::uint8_t> &bytes) {
        return deserialize(bytes.data(), bytes.size());
    }

    LmSol *get() const noexcept { return handle_; }
    LmSol *release() noexcept {
        LmSol *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmSol *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_SOLUTION_HPP
