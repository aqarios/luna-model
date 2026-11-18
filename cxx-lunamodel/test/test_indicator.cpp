#include <luna_model.hpp>

#include <optional>
#include <vector>

#include "common.hpp"

/* Entry point dispatched by the CTest driver (name must match the file stem). */
int test_indicator(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    lm::Env env;
    lm::Vref z = env.insert("z", LmVtype_Binary);
    lm::Vref x = env.insert("x", LmVtype_Integer);

    /* insert_before puts the new variable in front of its anchor. */
    lm::Vref y = env.insert_before(x, "y", LmVtype_Integer);
    std::vector<lm::Vref> vars = env.vars();
    CHECK(vars.size() == 3);
    CHECK(vars[1].name() == "y");
    CHECK(vars[2].name() == "x");

    /* d/dz (z x + 3 z + y) = x + 3. */
    lm::Expression expr = z * x + z * 3.0 + y;
    lm::Expression derivative = expr.derivative(z);
    CHECK(derivative.linear(x.id()) == 1.0);
    CHECK(derivative.linear(y.id()) == 0.0);
    CHECK(derivative.offset() == 3.0);

    /* A constraint without a common binary factor is no indicator. */
    lm::Constraint cap(x * 1.0, 3.0, LmComparator_Le, "cap");
    CHECK(!cap.indicator().has_value());

    /* when() and indicator() are inverses, for both values of the binary. */
    for (bool value : {true, false}) {
        lm::Constraint conditional = cap.when(z, value);
        CHECK(conditional.rhs() == 0.0);
        CHECK(conditional.name() == "cap");

        std::optional<lm::Indicator> indicator = conditional.indicator();
        CHECK(indicator.has_value());
        CHECK(indicator->variable.id() == z.id());
        CHECK(indicator->value == value);
        CHECK(indicator->constraint.equal_contents(cap));
    }

    /* An inverted binary stands for the other value of its base. */
    lm::Vref not_z = !z;
    std::optional<lm::Indicator> inverted = cap.when(not_z).indicator();
    CHECK(inverted.has_value());
    CHECK(inverted->variable.id() == z.id());
    CHECK(!inverted->value);

    /* Only binaries can be conditions. */
    bool threw = false;
    try {
        cap.when(x, true);
    } catch (const lm::Error &) {
        threw = true;
    }
    CHECK(threw);

    return 0;
}
