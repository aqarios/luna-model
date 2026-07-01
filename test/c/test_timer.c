#include <luna_model.h>
#include <stdint.h>

#include "common.h"
#include "luna_model/funcs.h"
#include "luna_model/types.h"

int test_timer(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    LmTimer *timer = lm_timer_start();
    CHECK(NULL != timer);

    LmTiming *timing = NULL;
    LmStatus status = lm_timer_stop(timer, &timing);
    CHECK(LmStatus_Ok == status);

    lm_timer_free(timer);
    lm_timing_free(timing);

    return 0;
}
