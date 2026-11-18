#include "luna_model/timer.hpp"
#include <luna_model.hpp>

#include <vector>

#include "common.hpp"

int test_timer(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    lm::Timer timer = lm::Timer::start();

    lm::SubTimer sub = timer.record("preprocessing");
    double elapsed = sub.stop();
    CHECK(elapsed >= 0.0);

    lm::Timing timing = timer.stop();
    CHECK(timing.total() >= 0.0);

    std::vector<double> recorded = timing.get("preprocessing");
    CHECK(recorded.size() == 1);

    std::optional<double> recorded_total = timing.total_for("preprocessing");
    CHECK(recorded_total.has_value());
    CHECK(!timing.total_for("missing").has_value());

    timing.set("manual", 1.5);
    timing.add("manual", 2.5);
    std::optional<double> manual_total = timing.total_for("manual");
    CHECK(manual_total.has_value() && *manual_total > 3.9 && *manual_total < 4.1);

    // Serialization round-trips a timing record without going through a Solution. The wire
    // format stores timestamps as seconds-since-epoch `f64`, which loses sub-microsecond
    // precision at typical Unix-epoch magnitudes — a pre-existing property of the shared
    // serializer, not specific to this accessor — so compare within a tolerance rather than
    // exact nanoseconds.
    std::vector<std::uint8_t> bytes = timing.serialize();
    CHECK(!bytes.empty());
    lm::Timing restored = lm::Timing::deserialize(bytes);
    auto close_ns = [](std::uint64_t a, std::uint64_t b) {
        std::uint64_t diff = a > b ? a - b : b - a;
        return diff < 1000; // within 1 microsecond
    };
    CHECK(restored.start_ns().has_value() && timing.start_ns().has_value());
    CHECK(close_ns(*restored.start_ns(), *timing.start_ns()));
    CHECK(restored.end_ns().has_value() && timing.end_ns().has_value());
    CHECK(close_ns(*restored.end_ns(), *timing.end_ns()));

    return 0;
}
