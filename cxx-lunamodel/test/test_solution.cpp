#include <cstdint>
#include <string>
#include <vector>

#include <luna_model.hpp>

#include "common.hpp"

/* Entry point dispatched by the CTest driver (name must match the file stem). */
int test_solution(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    // Creation.
    lm::Solution empty;
    CHECK(empty.is_empty());
    CHECK(empty.len() == 0u);
    CHECK(empty.sense() == LmSense_Min);

    lm::Solution maximize(LmSense_Max);
    CHECK(maximize.sense() == LmSense_Max);

    // Build a two-variable, two-row solution.
    lm::Solution sol;
    sol.add_binary("x", {0.0, 1.0});
    sol.add_real("y", {0.5, 1.5});

    CHECK(!sol.is_empty());
    CHECK(sol.len() == 2u);
    CHECK(sol.sample_len() == 2u);

    // Variable names preserve insertion (column) order.
    std::vector<std::string> names = sol.variable_names();
    CHECK(names.size() == 2u);
    CHECK(names[0] == "x");
    CHECK(names[1] == "y");

    // Raw stored values.
    CHECK(sol.value(0, "x").value() == 0.0);
    CHECK(sol.value(1, "x").value() == 1.0);
    CHECK(sol.value(0, "y").value() == 0.5);
    // An out-of-range sample yields std::nullopt.
    CHECK(!sol.value(999, "x").has_value());

    // Typed assignments, by name and by column index.
    LmAssignment ax = sol.assignment(0, "x");
    CHECK(ax.vtype == LmVtype_Binary);
    CHECK(ax.value == 0.0);
    CHECK(sol.assignment_idx(1, 0).value == 1.0);

    // Column handle mirrors the stored column.
    lm::Column cx = sol.column("x");
    CHECK(cx.len() == 2u);
    CHECK(!cx.is_empty());
    CHECK(cx.assignment(1).value == 1.0);

    // Standalone column operations stay in the host implementation.
    lm::Column owned = lm::Column::binary({0.0, 1.0});
    owned.try_push(1.0);
    CHECK(owned.vtype() == LmVtype_Binary);
    CHECK(owned.values().size() == 3u);
    lm::Column filtered = owned.filter_by_mask({false, true, true});
    CHECK(filtered.values() == std::vector<double>({1.0, 1.0}));
    lm::Column owned_clone = owned.clone();
    CHECK(owned_clone == owned);
    owned_clone.remove(0);
    CHECK(owned_clone != owned);

    // Sample map entries carry index, name, and an owned column.
    std::vector<lm::SampleColumn> cols = sol.samples();
    CHECK(cols.size() == 2u);
    CHECK(cols[0].index == 0u);
    CHECK(cols[0].key == "x");
    CHECK(cols[0].column.len() == 2u);
    CHECK(cols[1].index == 1u);
    CHECK(cols[1].key == "y");

    // Metadata is unevaluated on a freshly built solution.
    CHECK(!sol.raw_energies().has_value());
    CHECK(!sol.obj_values().has_value());
    CHECK(!sol.feasible().has_value());
    CHECK(!sol.timing().has_value());
    CHECK(sol.constraints().empty());
    CHECK(sol.variable_bounds().empty());

    // Sample/result views retain their own host snapshot.
    std::optional<lm::SampleView> sample = sol.sample(1);
    CHECK(sample.has_value());
    CHECK(sample->index() == 1u);
    CHECK(sample->assignment("x").value == 1.0);
    CHECK(sol.sample_views().size() == 2u);
    lm::Solution view_solution;
    view_solution.push({{"x", LmAssignment{LmVtype_Binary, 1.0}}}, 3);
    std::optional<lm::ResultView> result = view_solution.result(0);
    CHECK(result.has_value());
    CHECK(result->counts() == 3u);
    CHECK(result->sample().assignment("x").value == 1.0);

    // Serialize / deserialize round-trip preserves contents.
    std::vector<std::uint8_t> bytes = sol.serialize();
    CHECK(!bytes.empty());
    lm::Solution restored = lm::Solution::deserialize(bytes);
    CHECK(restored.equal_contents(sol));

    // Clone is independent but content-equal.
    lm::Solution cloned = sol.clone();
    CHECK(cloned.get() != sol.get());
    CHECK(cloned.equal_contents(sol));
    CHECK(cloned == sol);

    // Mutation on the clone does not affect the original.
    cloned.remove_col("x");
    CHECK(cloned.sample_len() == 1u);
    CHECK(sol.sample_len() == 2u);

    // Error paths surface as lm::Error with a non-Ok status.
    bool threw = false;
    try {
        std::vector<std::uint8_t> garbage = {1, 2, 3, 4, 5};
        lm::Solution::deserialize(garbage);
    } catch (const lm::Error &e) {
        threw = true;
        CHECK(e.status() != LmStatus_Ok);
    }
    CHECK(threw);

    threw = false;
    try {
        sol.column("does-not-exist");
    } catch (const lm::Error &e) {
        threw = true;
        CHECK(e.status() != LmStatus_Ok);
    }
    CHECK(threw);

    threw = false;
    try {
        sol.assignment(0, "does-not-exist");
    } catch (const lm::Error &e) {
        threw = true;
        CHECK(e.status() != LmStatus_Ok);
    }
    CHECK(threw);

    // Convenience statistics require evaluated objective values.
    threw = false;
    try {
        sol.expectation_value();
    } catch (const lm::Error &e) {
        threw = true;
        CHECK(e.status() != LmStatus_Ok);
    }
    CHECK(threw);

    return 0;
}
