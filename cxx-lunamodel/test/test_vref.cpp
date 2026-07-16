#include <luna_model.hpp>

#include "common.hpp"

/* Entry point dispatched by the CTest driver (name must match the file stem). */
int test_vref(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    lm::Env env;
    lm::Vref x = env.insert("x", LmVtype_Binary);
    lm::Vref y = env.insert("y", LmVtype_Binary);
    lm::Vref s = env.insert("s", LmVtype_Spin);

    /* Access. */
    CHECK(x.name() == "x");
    CHECK(x.vtype() == LmVtype_Binary);
    CHECK(s.vtype() == LmVtype_Spin);
    CHECK(x.check_living());
    CHECK(x == x);
    CHECK(x != y);
    CHECK(x.hash() == x.hash());
    CHECK(x.hash() != y.hash());
    CHECK(x.to_variable().name() == "x");

    /* Default bounds for a binary variable are [0, 1]. */
    lm::Bounds bx = x.bounds();
    CHECK(bx.lower() == lm::Bound::bounded(0.0));
    CHECK(bx.upper() == lm::Bound::bounded(1.0));

    /* Inverted companion: absent until `!x` creates it. */
    CHECK(!x.inverted().has_value());
    lm::Vref nx = !x;
    CHECK(nx.vtype() == LmVtype_InvertedBinary);
    CHECK(x.inverted().has_value());
    CHECK(*x.inverted() == nx);

    /* vref * vref promotes to a quadratic expression. */
    lm::Expression q = x * y;
    CHECK(q.degree() == 2);
    CHECK(q.num_vars() == 2);
    CHECK(q.has_quadratic());
    CHECK(!q.has_higher_order());
    CHECK(q.quadratic_terms().has_value());
    CHECK(q.quadratic(x.id(), y.id()) == 1.0);
    CHECK(!q.higher_order_terms().has_value());
    CHECK(q.vars().size() == 2);
    CHECK(q.vtypes().size() == 1); /* both binary */

    /* Domain identities enforced by the core:
     * binary * same binary collapses to linear, ... */
    lm::Expression xx = x * x;
    CHECK(xx.degree() == 1);
    CHECK(xx.linear(x.id()) == 1.0);
    /* ... spin * same spin collapses to the constant 1, ... */
    lm::Expression ss = s * s;
    CHECK(ss.is_constant());
    CHECK(ss.offset() == 1.0);
    /* ... and binary * its inverted companion collapses to zero. */
    lm::Expression zero = x * nx;
    CHECK(zero.is_constant());
    CHECK(zero.offset() == 0.0);

    /* Scalar ops, both operand orders. */
    CHECK((2.0 * x).linear(x.id()) == 2.0);
    CHECK((x * 2.0).linear(x.id()) == 2.0);
    CHECK((x + 1.5).offset() == 1.5);
    CHECK((1.5 + x).offset() == 1.5);
    CHECK((x - 1.0).offset() == -1.0);
    CHECK((1.0 - x).offset() == 1.0);
    CHECK((1.0 - x).linear(x.id()) == -1.0);

    /* Addition, subtraction, negation, pow. */
    CHECK((x + y).degree() == 1);
    CHECK((x + y).num_vars() == 2);
    CHECK((x - y).linear(y.id()) == -1.0);
    CHECK((-x).linear(x.id()) == -1.0);
    CHECK(x.pow(2).degree() == 1); /* binary^2 == binary */
    CHECK(s.pow(2).is_constant()); /* spin^2 == 1 */

    /* Compound assignment mutates in place. */
    lm::Expression e = x + y;
    e += s;
    CHECK(e.num_vars() == 3);

    /* Expression clone / equality / serialization round-trip. */
    lm::Expression c = e.clone();
    CHECK(c == e);
    CHECK(c.equal_contents(e));
    lm::Expression restored = lm::Expression::deserialize(e.serialize(), e.env());
    CHECK(restored.equal_contents(e));

    /* Bounds evaluation. */
    CHECK(x.evaluate(0.0));
    CHECK(!x.evaluate(5.0));

    /* Mixing environments is an error, not UB. */
    lm::Env env2;
    lm::Vref x2 = env2.insert("x", LmVtype_Binary);
    bool threw = false;
    try {
        lm::Expression bad = x * x2;
        (void)bad;
    } catch (const lm::Error &err) {
        threw = true;
        CHECK(err.status() != LmStatus_Ok);
    }
    CHECK(threw);

    return 0;
}
