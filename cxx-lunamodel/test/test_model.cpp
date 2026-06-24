#include <cstdint>
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
