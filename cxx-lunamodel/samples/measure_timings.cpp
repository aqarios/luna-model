#include "luna_model/subtimer.hpp"
#include "luna_model/timer.hpp"
#include "luna_model/timing.hpp"
#include <iostream>

#include <luna_model.hpp>
#include <thread>

int main() {
    std::cout << "LunaModel " << lm::version() << "\n";

    lm::Timer timer = lm::Timer::start();

    lm::SubTimer sub = timer.record("phase_a");
    std::this_thread::sleep_for(std::chrono::nanoseconds(1000));
    sub.stop();
    lm::SubTimer sub2 = timer.record("phase_b");
    std::this_thread::sleep_for(std::chrono::nanoseconds(5000));
    sub2.stop();

    lm::Timing timing = timer.stop();

    std::cout << "Timer total: " << timing.total() << std::endl;
    std::cout << "Timer 'phase_a': " << timing.total_for("phase_a").value_or(-1) << std::endl;
    std::cout << "Timer 'phase_b': " << timing.total_for("phase_b").value_or(-1) << std::endl;
}
