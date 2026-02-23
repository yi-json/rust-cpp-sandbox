#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // Tell Rust to look at our C++ header
        include!("src/counter.h");

        // Map the C++ Counter class to an opaque Rust type
        type Counter;
        
        // Define the methods we want to call from Rust
        fn new_counter() -> UniquePtr<Counter>;
        fn inc(self: Pin<&mut Counter>);
        fn get(self: &Counter) -> i32;
    }
}

fn main() {
    // 1. Instantiate the C++ class
    let mut counter = ffi::new_counter();
    println!("Initial value from C++: {}", counter.get());

    // 2. Mutate the C++ class (Notice we have to pin it in memory!)
    counter.pin_mut().inc();
    println!("Value after inc() in Rust: {}", counter.get());
}