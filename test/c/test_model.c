#include <stdint.h>
#include <string.h>

#include <luna_model.h>

#include "common.h"

/* Entry point dispatched by the CTest driver (name must match the file stem).
 */
int test_model(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    const char *version = lm_version();
    CHECK(version != NULL);
    CHECK(strlen(version) > 0);
    CHECK(lm_abi_version() == LUNA_MODEL_CAPI_ABI_VERSION);

    /* Round-trip a fresh model through serialize -> deserialize. */
    LmModel *model = lm_model_default();
    CHECK(model != NULL);

    LmBuffer buf = {NULL, 0, 0};
    CHECK(lm_model_serialize(model, &buf) == LmStatus_Ok);
    CHECK(buf.len > 0);

    LmModel *restored = NULL;
    CHECK(lm_model_deserialize(buf.data, buf.len, &restored) == LmStatus_Ok);
    CHECK(restored != NULL);

    lm_buffer_free(buf);
    lm_model_free(restored);
    lm_model_free(model);

    /* Garbage input fails cleanly with a structured error, no crash. */
    uint8_t garbage[] = {1, 2, 3, 4, 5};
    LmModel *bad = NULL;
    CHECK(lm_model_deserialize(garbage, sizeof(garbage), &bad) != LmStatus_Ok);
    CHECK(bad == NULL);

    LmError *err = lm_last_error();
    CHECK(err != NULL);
    LmBuffer msg = {NULL, 0, 0};
    CHECK(lm_error_message(err, &msg) == LmStatus_Ok);
    CHECK(msg.len > 0);
    lm_buffer_free(msg);
    lm_error_free(err);

    return 0;
}
