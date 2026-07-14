#include <luna_model.hpp>

#include <cstdint>
#include <string>
#include <vector>

#include "common.hpp"

/* Exercises the newly wrapped surface: model objective/constraints/variables,
 * constraint collections, specs, lazy bounds, variables, sample views, and the
 * added solution/expression/term operations. */
int test_full_api(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    // --- version / abi ---
    CHECK(!lm::version().empty());
    CHECK(lm::abi_version() > 0u);

    // --- bound / bounds / lazy bounds ---
    CHECK(lm::Bound::bounded(3.0).is_bounded());
    CHECK(!lm::Bound::unbounded().is_bounded());

    // Integer bounds accept explicit integral endpoints.
    lm::LazyBounds lb(lm::Bound::bounded(0.0), lm::Bound::bounded(5.0));
    lm::Bounds concrete = lb.concretize(LmVtype_Integer);
    CHECK(concrete.lower() == lm::Bound::bounded(0.0));
    CHECK(concrete.upper() == lm::Bound::bounded(5.0));
    lm::LazyBounds dflt = lm::LazyBounds::default_bounds();
    lm::LazyBounds lb2 = dflt.clone();
    (void)lb2.lower();
    (void)lb2.upper();

    // --- variable ---
    lm::Variable z("z", LmVtype_Integer);
    CHECK(z.name() == "z");
    CHECK(z.vtype() == LmVtype_Integer);
    lm::Variable z2 = z.clone();
    CHECK(z2.name() == "z");

    // --- model over an environment ---
    lm::Env env;
    lm::Model model = lm::Model::with_env(env, std::string("m"), LmSense_Min);
    CHECK(model.name() == "m");
    CHECK(model.sense() == LmSense_Min);

    lm::Vref x = model.add_var("x", LmVtype_Binary);
    lm::Vref y = model.add_var("y", LmVtype_Binary);
    // add_var registers variables in the environment.
    CHECK(model.env().len() == 2u);
    CHECK(model.var("x").name() == "x");

    // Objective: 2*y + x.
    lm::Expression obj = ((y * 2.0) + x);
    model.set_objective(obj);
    CHECK(model.objective().num_vars() == 2u);
    // num_variables()/vars() count variables used by the objective + constraints.
    CHECK(model.num_variables() == 2u);
    CHECK(model.vars().size() == 2u);

    // Constraint collection: x + y <= 1.
    lm::Constraint c(x + y, 1.0, LmComparator_Le, std::string("cap"));
    CHECK(!c.auto_name());
    lm::ConstraintCollection cc;
    CHECK(cc.is_empty());
    std::string added = cc.add_constraint(c);
    CHECK(added == "cap");
    CHECK(cc.len() == 1u);
    CHECK(cc.contains("cap"));
    CHECK(cc.get("cap").name() == "cap");
    CHECK(cc.names().size() == 1u);
    CHECK(cc.vars().size() == 2u);
    model.set_constraints(cc);
    CHECK(model.constraints().len() == 1u);

    // Removing from a copy leaves the model's collection intact.
    lm::ConstraintCollection cc2 = cc.clone();
    cc2.remove_constraint("cap");
    CHECK(cc2.is_empty());
    CHECK(cc.len() == 1u);

    // --- specs ---
    lm::Specs specs = model.specs();
    (void)specs.max_degree();
    (void)specs.max_constraint_degree();
    (void)specs.sense();
    CHECK(specs.satisfies(model.specs()));
    lm::Specs specs2 = specs.clone();
    (void)specs2.vtypes();

    // --- expression from a dense quadratic matrix ---
    // Real variables keep their quadratic terms (binary would fold x*x -> x).
    lm::Env qenv;
    qenv.insert("a", LmVtype_Real);
    qenv.insert("b", LmVtype_Real);
    lm::Expression q = lm::Expression::from_dense_quadratic({1.0, 1.0, 1.0, 1.0}, 2, qenv);
    CHECK(q.has_quadratic());

    // --- scalar/vref reverse-subtraction operators ---
    lm::Expression rs_bias = 5.0 - obj; // bias - expr
    CHECK(rs_bias.num_vars() == 2u);
    lm::Expression rs_vref = x - obj; // vref - expr
    CHECK(rs_vref.num_vars() >= 1u);

    // --- quadratic term-store mutation ---
    lm::Quadratic quad;
    quad.push_back_empty(0);
    quad.add_entry(0, 1, 2.0);
    CHECK(quad(0, 1) == 2.0);
    quad.set_bias(0, 1, 0.0);
    quad.clean();
    CHECK(quad.is_zero());

    lm::HigherOrder ho;
    ho.add_entry({0, 1, 2}, 1.0);
    ho.set_bias({0, 1, 2}, 0.0);
    ho.clean();
    CHECK(ho.is_zero());

    // --- solution: set_sense, sample views, push, filter_by_mask, column vtype ---
    lm::Solution sol;
    sol.add_binary("x", {0.0, 1.0});
    sol.add_binary("y", {1.0, 0.0});
    CHECK(sol.len() == 2u);

    sol.set_sense(LmSense_Max);
    CHECK(sol.sense() == LmSense_Max);

    CHECK(sol.column("x").vtype() == LmVtype_Binary);

    lm::SampleView sv = sol.sample_view(0);
    CHECK(sv.index() == 0u);
    CHECK(sv.variable_names().size() == 2u);
    CHECK(sv.assignment("x").value == 0.0);
    CHECK(sv.assignment_idx(0).value == 0.0);
    CHECK(sol.sample_views().size() == 2u);

    // Append a third row via typed assignments.
    std::vector<LmAssignment> row = {LmAssignment{LmVtype_Binary, 1.0},
                                     LmAssignment{LmVtype_Binary, 1.0}};
    sol.push({"x", "y"}, row);
    CHECK(sol.len() == 3u);

    // Keep only the first and third rows.
    lm::Solution filtered = sol.filter_by_mask({true, false, true});
    CHECK(filtered.len() == 2u);

    return 0;
}
