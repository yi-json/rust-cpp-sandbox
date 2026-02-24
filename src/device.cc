#include "src/main.cxx.h"
#include <iostream>

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
    DeviceAddressBytes my_address;
     my_address.bytes = {0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x6F};

    PeerId result = assign_peer_id(my_address);
    std::cout << "C++ received Peer ID: " << result.value << "\n";
}