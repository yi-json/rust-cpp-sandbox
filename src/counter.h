#pragma once

class Counter {
public:
  Counter() : value_(0) {}
  void increment() { value_++; }

private:
  int value_;
};
