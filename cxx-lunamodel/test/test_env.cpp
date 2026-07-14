#include <luna_model.hpp>

#include <vector>

#include "common.hpp"

/* Entry point dispatched by the CTest driver (name must match the file stem). */
int test_env(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    /* Empty environment. */
    lm::Env env;
    CHECK(env.is_empty());
    CHECK(env.len() == 0);

    /* Insert + scalar accessors. */
    lm::Vref x = env.insert("x", LmVtype_Binary);
    lm::Vref y = env.insert("y", LmVtype_Spin);
    CHECK(!env.is_empty());
    CHECK(env.len() == 2);
    CHECK(env.contains("x"));
    CHECK(!env.contains("nope"));
    CHECK(env.vtype_of("y") == LmVtype_Spin);
    CHECK(env.next_idx() >= 2);

    /* Vref-returning accessors. */
    CHECK(env.vars().size() == 2);
    CHECK(env.lookup("x").name() == "x");
    CHECK(env.get(x.id()).name() == "x");

    /* Canonical sort of a name list. */
    std::vector<std::string> sorted = env.sort({"y", "x"});
    CHECK(sorted.size() == 2);
    CHECK(sorted[0] == "x");
    CHECK(sorted[1] == "y");

    /* clone() shares identity; deep_clone() is independent but equal. */
    lm::Env shared = env.clone();
    CHECK(shared.id().hi == env.id().hi);
    CHECK(shared.id().lo == env.id().lo);
    lm::Env deep = env.deep_clone();
    CHECK(deep.equal_contents(env));

    /* Serialize round-trip. */
    std::vector<std::uint8_t> bytes = env.serialize();
    CHECK(!bytes.empty());
    lm::Env restored = lm::Env::deserialize(bytes.data(), bytes.size());
    CHECK(restored.equal_contents(env));

    /* Inverted companion of a binary variable grows the environment. */
    lm::Vref x_inv = env.insert_inverted(x);
    CHECK(x_inv.check_living());
    CHECK(env.len() == 3);

    /* Remove a variable. */
    env.remove(y);
    CHECK(!env.contains("y"));

    /* Explicit construction from existing variables (Env::create). */
    lm::Env src;
    lm::Vref a = src.insert("a", LmVtype_Binary);
    lm::Vref b = src.insert("b", LmVtype_Integer);
    lm::Variable va = a.to_var();
    lm::Variable vb = b.to_var();
    std::uint32_t ai = a.id();
    std::uint32_t bi = b.id();
    lm::Env built = lm::Env::create({{ai, &va}, {bi, &vb}}, {{"a", ai}, {"b", bi}}, src.next_idx());
    CHECK(built.len() == 2);
    CHECK(built.contains("a"));
    CHECK(built.lookup("b").name() == "b");

    return 0;
}
