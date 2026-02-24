#include "src/main.cxx.h"
#include "rust/cxx.h"
#include <iostream>
#include <vector>

/*
Recall that main.cxx.h looks like:

struct DeviceAddressBytes final {
  ::std::array<::std::uint8_t, 6> bytes;
};

struct PeerId final {
  ::std::uint64_t value;
};

*/

void simulate_discovery() {
    // == phase 2: sharing c++ opaque types ==
    // DeviceAddressBytes my_address;
    //  my_address.bytes = {0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x6F};

    // PeerId result = assign_peer_id(my_address);
    // std::cout << "C++ received Peer ID: " << result.value << "\n";

    // == phase 3: sharing dynamic structures ==
    // std::string text = "Sensor-Alpha";
    // rust::String result = verify_device_name(text);
    // std::cout << "C++ received new string: " << result.c_str() << std::endl;
    // std::cout << "Size of string: " << result.size() << std::endl;

    // std::vector<uint8_t> payload = {1, 2, 3, 4};
    // rust::Vec<uint8_t> packet = process_payload(payload);
    // std::cout << "C++ received new vector: ";
    // for (uint8_t x : packet) {
    //   std::cout << static_cast<int>(x) << " "; 
    // }
    // std::cout << std::endl;

    // == phase 4: error handling == 
    // try {
    //   connect_to_device(42); // succeed
    //   connect_to_device(99); // error

    //   std::cout << "C++: Both devices connected successfully.\n";
    // } catch (const rust::Error& e) {
    //   std::cerr << "C++ Caught a Rust Error " << e.what() << std::endl;
    // }

    // == phase 5: rust opaque types ==
    rust::Box<MyRustDevice> device = create_rust_device(200);

    device->tick();
    device->tick();

    uint32_t current_state = device->get_state();
    std::cout << "C++: Fetched final state from Rust: " << current_state << std::endl;

}