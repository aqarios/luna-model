#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include <luna_model.hpp>

#include "common.hpp"

/* Entry point dispatched by the CTest driver (name must match the file stem). */
int test_model(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    CHECK(!lm::version().empty());

    lm::Model model;
    std::vector<std::uint8_t> bytes = model.serialize();
    CHECK(!bytes.empty());

    lm::Model restored = lm::Model::deserialize(bytes);
    CHECK(restored.serialize().size() == bytes.size());

    lm::Model cloned = model.clone();
    CHECK(cloned.get() != nullptr);
    CHECK(cloned.get() != model.get());

    // Consumer-facing model operations all route through C handles.
    lm::Vref x = model.add_var("x", LmVtype_Binary);
    lm::Expression objective = x + 1.0;
    model.set_objective(objective);
    CHECK(model.objective().offset() == 1.0);
    CHECK(model.vars().size() == 1u);
    CHECK(model.var("x").id() == x.id());

    lm::Expression lhs = x + 0.0;
    lm::Constraint constraint(std::move(lhs), 0.5, LmComparator_Le,
                              std::optional<std::string>("limit"));
    CHECK(constraint.evaluate_sample_quick({0.0}));
    CHECK(!constraint.evaluate_sample_quick({1.0}));
    lm::ConstraintCollection constraints;
    CHECK(constraints.add_constraint(constraint) == "limit");
    CHECK(constraints.contains("limit"));
    CHECK(!constraints.evaluate_sample_quick({1.0}).at("limit"));
    model.set_constraints(constraints);

    lm::Solution candidate;
    candidate.add_binary("x", {1.0});
    lm::SampleView sample = std::move(candidate.sample(0).value());
    CHECK(model.violated_constraints(sample).len() == 1u);

    lm::Specs specs = model.specs();
    CHECK(model.satisfies(specs));
    CHECK(specs.satisfies(specs));
    CHECK(specs.diff(specs).empty());

    // Garbage input throws lm::Error with a non-Ok status.
    bool threw = false;
    try {
        std::vector<std::uint8_t> garbage = {1, 2, 3, 4, 5};
        lm::Model::deserialize(garbage);
    } catch (const lm::Error &e) {
        threw = true;
        CHECK(e.status() != LmStatus_Ok);
    }
    CHECK(threw);

    return 0;
}
