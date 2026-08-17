#include "luna_model/timer.hpp"
#include <luna_model.hpp>

#include <vector>

#include "common.hpp"

int test_timer(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    lm::Timer timer = lm::Timer::start();
    lm::Timing timing = timer.stop();

    // Serialization round-trips a timing record without going through a Solution. The wire
    // format (SerTiming V0) stores timestamps as seconds-since-epoch `f64`, which loses
    // sub-microsecond precision at typical Unix-epoch magnitudes — a pre-existing property of
    // the shared serializer, not specific to this accessor — so compare within a tolerance
    // rather than exact nanoseconds.
    std::vector<std::uint8_t> bytes = timing.serialize();
    CHECK(!bytes.empty());
    lm::Timing restored = lm::Timing::deserialize(bytes);
    auto close_ns = [](std::uint64_t a, std::uint64_t b) {
        std::uint64_t diff = a > b ? a - b : b - a;
        return diff < 1000; // within 1 microsecond
    };
    CHECK(close_ns(restored.start_ns(), timing.start_ns()));
    CHECK(close_ns(restored.end_ns(), timing.end_ns()));
    CHECK(close_ns(restored.total_ns(), timing.total_ns()));

    return 0;
}
