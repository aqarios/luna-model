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

    /* Serialization round-trips a timing record without going through a Solution. */
    LmBuffer buf = {NULL, 0, 0};
    CHECK(LmStatus_Ok == lm_timing_serialize(timing, &buf));
    CHECK(buf.len > 0);

    LmTiming *restored = NULL;
    CHECK(LmStatus_Ok == lm_timing_deserialize(buf.data, buf.len, &restored));
    CHECK(NULL != restored);
    lm_buffer_free(buf);
    lm_timing_free(restored);

    lm_timer_free(timer);
    lm_timing_free(timing);

    return 0;
}
