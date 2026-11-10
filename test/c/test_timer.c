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

    LmSubTimer *sub = NULL;
    CHECK(LmStatus_Ok == lm_timer_record(timer, "preprocessing", &sub));
    CHECK(NULL != sub);
    double elapsed = 0.0;
    CHECK(LmStatus_Ok == lm_subtimer_stop(sub, &elapsed));
    CHECK(elapsed >= 0.0);

    LmTiming *timing = NULL;
    LmStatus status = lm_timer_stop(timer, &timing);
    CHECK(LmStatus_Ok == status);

    double total = -1.0;
    CHECK(LmStatus_Ok == lm_timing_total_seconds(timing, &total));
    CHECK(total >= 0.0);

    LmF64Buffer recorded = {NULL, 0, 0};
    CHECK(LmStatus_Ok == lm_timing_get(timing, "preprocessing", &recorded));
    CHECK(1 == recorded.len);
    lm_f64_buffer_free(recorded);

    double recorded_total = -1.0;
    CHECK(LmStatus_Ok == lm_timing_total_for(timing, "preprocessing", &recorded_total));
    CHECK(recorded_total >= 0.0);

    CHECK(LmStatus_Absent == lm_timing_total_for(timing, "missing", &recorded_total));

    CHECK(LmStatus_Ok == lm_timing_set(timing, "manual", 1.5));
    CHECK(LmStatus_Ok == lm_timing_add(timing, "manual", 2.5));
    double manual_total = -1.0;
    CHECK(LmStatus_Ok == lm_timing_total_for(timing, "manual", &manual_total));
    CHECK(manual_total > 3.9 && manual_total < 4.1);

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
