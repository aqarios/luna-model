#include <luna_model.hpp>

#include <map>
#include <string>
#include <vector>

#include "common.hpp"

/* Exercises the public API that dispatches into the C ABI: expression
 * evaluation and item iteration, sample-view materialization, solution
 * filtering, and model evaluate-sample / violated-constraints. */
int test_parity(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    lm::Env env;
    lm::Vref x = env.insert("x", LmVtype_Binary);
    lm::Vref y = env.insert("y", LmVtype_Binary);
    lm::Vref z = env.insert("z", LmVtype_Binary);

    // Objective 2x + y.
    lm::Expression obj = (x * 2.0) + y;

    // --- expression evaluation (dispatches to lm_expr_evaluate_sample) ---
    CHECK(obj.evaluate_sample({{"x", 1.0}, {"y", 0.0}}) == 2.0);
    CHECK(obj.evaluate_sample({{"x", 1.0}, {"y", 1.0}}) == 3.0);

    // --- expression introspection ---
    CHECK(obj.contains(x));
    CHECK(!obj.contains(z));
    CHECK(obj.linear(x.id()) == 2.0);
    CHECK(obj.linear_items().size() == 2u);
    CHECK(obj.items().size() == 2u); // two linear terms, zero offset

    // --- vref renames match rs (evaluate / to_variable) ---
    CHECK(x.evaluate(0.0));
    CHECK(x.to_variable().name() == "x");
    CHECK(env.get_variable(x.id()).name() == "x");

    // --- constraint collection: iter + add_many ---
    lm::Constraint cap(x + y, 1.0, LmComparator_Le, std::string("cap"));
    lm::ConstraintCollection cc;
    cc.add_constraint(cap);
    lm::Constraint floor(x + y, 0.0, LmComparator_Ge, std::string("floor"));
    std::vector<std::pair<const lm::Constraint *, std::optional<std::string>>> more = {
        {&floor, std::nullopt}};
    std::vector<std::string> added = cc.add_many_constraints(more);
    CHECK(added.size() == 1u);
    CHECK(added[0] == "floor");
    CHECK(cc.iter().size() == 2u);

    // --- model over the environment ---
    lm::Model model = lm::Model::with_env(env.clone(), std::string("m"));
    model.set_objective(obj);
    model.set_constraints(cc);

    // A one-row solution with x = y = 1, built via push so per-row counts are
    // populated (extract, used by model.evaluate_sample, requires them).
    lm::Solution sol;
    std::vector<LmAssignment> row = {LmAssignment{LmVtype_Binary, 1.0},
                                     LmAssignment{LmVtype_Binary, 1.0}};
    sol.push({"x", "y"}, row);

    // --- expression evaluate over a whole solution (lm_expr_evaluate) ---
    std::vector<double> series = obj.evaluate(sol);
    CHECK(series.size() == 1u);
    CHECK(series[0] == 3.0); // 2*1 + 1

    // --- sample-view materialization ---
    lm::SampleView sv = sol.sample_view(0);
    CHECK(sv.to_vec().size() == 2u);
    CHECK(sv.get("x").value == 1.0);
    CHECK(sv.iter().size() == 2u);
    CHECK(sv.iter_named().size() == 2u);
    CHECK(sv.as_map().at("y") == 1.0);
    CHECK(sv.try_get(std::size_t{0}).value == 1.0);

    // --- model evaluate_sample (lm_model_evaluate_sample) ---
    lm::ResultView rv = model.evaluate_sample(sv);
    CHECK(rv.obj_value().has_value());
    CHECK(rv.obj_value().value() == 3.0);
    CHECK(rv.feasible().has_value());

    // --- model violated_constraints (lm_model_violated_constraints) ---
    lm::ConstraintCollection violated = model.violated_constraints(sv);
    CHECK(violated.contains("cap"));    // x + y = 2 > 1
    CHECK(!violated.contains("floor")); // satisfied

    // --- solution: sample() optional + filter(predicate) ---
    CHECK(sol.sample(0).has_value());
    CHECK(!sol.sample(999).has_value());
    lm::Solution kept = sol.filter([](const lm::ResultView &) { return true; });
    CHECK(kept.len() == sol.len());
    lm::Solution dropped = sol.filter([](const lm::ResultView &) { return false; });
    CHECK(dropped.len() == 0u);

    return 0;
}
