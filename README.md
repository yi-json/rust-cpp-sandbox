# Rust + C++ Interop Sandbox

This sandbox demonstrates how to safely bridge Rust and C++ using the [`cxx`](https://cxx.rs/) crate and the Bazel build system. It covers the complete spectrum of cross-language memory management and function calling.

## Project Structure
* `src/main.rs`: The entry point and the `#[cxx::bridge]` macro definition.
* `src/rust_device.rs`: Pure, safe Rust business logic (isolated from FFI).
* `src/device.cc`: The C++ implementation side.
* `src/device.h`: C++ header file to declare functions for the generated bridge.
* `src/counter.cc` & `src/counter.h`: The implementation of the C++ Opaque Type (`Counter` class).
* `src/BUILD.bazel`: The Bazel build graph linking the C++ compiler, Rust compiler, and CXX code generator.


## How to Run
To compile and execute the complete FFI pipeline, first navigate to `src/device.cc` and comment/uncomment the bridge phase(s) you'd like to execute.

then run:
```bash
bazel run //src:main
```

## Interop Phases Covered
### 1. C++ Opaque Types
Concept: Rust holds a blind pointer to a C++ heap-allocated object.

Mechanism: Declared as type Counter; inside extern "C++". Rust cannot inspect the fields, but can pass the pointer back to C++ to call methods.

### 2. Shared Structs (Stack Memory)
Concept: Passing exact byte-for-byte data structures by value.

Mechanism: Structs defined inside the #[cxx::bridge] macro (e.g., DeviceAddressBytes). Both compilers agree on the memory layout, allowing safe, zero-overhead passing.

### 3. Dynamic Strings (Heap Memory)
Concept: Safely passing strings without triggering double-frees or segmentation faults from mismatched allocators.

C++ to Rust: C++ std::string is received in Rust as &cxx::CxxString.

Rust to C++: Rust String is received in C++ as rust::String, which handles safe destruction by calling back into Rust's allocator.

### 4. Dynamic Vectors
Concept: Passing dynamic arrays of data (e.g., byte payloads).

C++ to Rust: C++ std::vector<uint8_t> is received as &cxx::CxxVector<u8>.

Rust to C++: Rust Vec<u8> is received as rust::Vec<uint8_t>. It fully supports standard C++ range-based for loops and iterators.

### 5. Error Handling & Exceptions
Concept: Translating Rust's enum-based errors into standard C++ exceptions.

Mechanism: Rust returns a Result<T, E>. If it returns Err, the CXX bridge automatically intercepts it and throws a rust::Error in C++, which can be caught using a standard try-catch block.

### 6. Rust Opaque Types
Concept: C++ holds a smart pointer to a stateful Rust object and calls its methods.

Mechanism: Declared as type MyRustDevice; inside extern "Rust". C++ receives a rust::Box<MyRustDevice> and can call methods on it using the -> operator, executing safe Rust code under the hood.