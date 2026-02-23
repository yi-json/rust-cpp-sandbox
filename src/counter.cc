#include "counter.h"

Counter::Counter(): val(0) {}
void Counter::inc() { val++; }
int Counter::get() const { return val; }

std::unique_ptr<Counter> new_counter() {
    return std::make_unique<Counter>();
}