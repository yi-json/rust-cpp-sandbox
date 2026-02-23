#pragma once
#include <memory>

class Counter {
public:
  Counter();
  void inc();
  int get() const;
private:
  int val;
};

// Factory function for Rust to call
std::unique_ptr<Counter> new_counter();