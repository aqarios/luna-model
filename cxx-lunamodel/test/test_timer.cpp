#include "luna_model/timer.hpp"
#include <luna_model.hpp>

int test_timer(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    lm::Timer timer = lm::Timer::start();
    lm::Timing timing = timer.stop();

    return 0;
}
