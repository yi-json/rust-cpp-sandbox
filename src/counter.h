#pragma once

class Counter {
public:
  Counter() : value_(0) {}
  void increment() { value_++; }
  int get() const { return value_; }

private:
  int value_;
};
