#include <luna_model.hpp>

#include "common.hpp"

/* Entry point dispatched by the CTest driver (name must match the file stem). */
int test_error(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    lm::Env env;
    env.insert("x", LmVtype_Binary);

    /* Inserting a duplicate name raises a genuine business-logic error (LunaModelError's
     * `VariableExists` variant) — this is exactly the case the typed ErrorKind exists for: a C++
     * caller distinguishing that from, say, `InvalidBounds` or `IndexOutOfBounds`, rather than
     * seeing only the coarse `LmStatus_Errored`. */
    bool caught = false;
    try {
        env.insert("x", LmVtype_Spin);
    } catch (const lm::Error &e) {
        caught = true;
        CHECK(e.status() == LmStatus_Errored);
        CHECK(e.kind_tag() == "variable_exists");
        CHECK(e.kind() == lm::ErrorKind::VariableExists);
        CHECK(e.kind() != lm::ErrorKind::InvalidBounds);
        CHECK(e.kind() != lm::ErrorKind::Other);
    }
    CHECK(caught);

    /* An unrecognized/plugin-style tag falls into `Other` rather than being misclassified. */
    CHECK(lm::detail::error_kind_from_tag("some_plugin::custom_kind") == lm::ErrorKind::Other);

    return 0;
}
