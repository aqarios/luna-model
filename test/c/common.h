#ifndef LUNA_MODEL_TEST_COMMON_H
#define LUNA_MODEL_TEST_COMMON_H

#include <stdio.h>

/* Fails the current test (returns 1) when cond is false. */
#define CHECK(cond)                                                                                \
    do {                                                                                           \
        if (!(cond)) {                                                                             \
            fprintf(stderr, "FAILED: %s (%s:%d)\n", #cond, __FILE__, __LINE__);                    \
            return 1;                                                                              \
        }                                                                                          \
    } while (0)

#endif /* LUNA_MODEL_TEST_COMMON_H */
