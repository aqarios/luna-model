#ifndef LUNA_MODEL_CPP_TEST_COMMON_HPP
#define LUNA_MODEL_CPP_TEST_COMMON_HPP

#include <iostream>

/* Fails the current test (returns 1) when cond is false. */
#define CHECK(cond)                                                                                \
    do {                                                                                           \
        if (!(cond)) {                                                                             \
            std::cerr << "FAILED: " #cond " (" << __FILE__ << ":" << __LINE__ << ")\n";            \
            return 1;                                                                              \
        }                                                                                          \
    } while (0)

#endif // LUNA_MODEL_CPP_TEST_COMMON_HPP
