#[cxx::bridge]
mod ffi {
    struct DeviceAddressBytes {
        bytes: [u8; 6],
    }

    struct PeerId {
        value: u64,
    }

    unsafe extern "C++" {
        include!("src/device.h");
        fn simulate_discovery();
    }

    extern "Rust" {
        // phase 2 - shared types
        fn assign_peer_id(address: DeviceAddressBytes) -> PeerId;

        // phase 3 - sharing dynamic structures
        fn verify_device_name(name: &CxxString) -> String;
        fn process_payload(payload: &CxxVector<u8>) -> Vec<u8>;

        // phase 4 - error handling
        fn connect_to_device(id: u64) -> Result<()>;
    }
}

fn assign_peer_id(address: ffi::DeviceAddressBytes) -> ffi::PeerId {
    println!("{:?}", address.bytes);
    ffi::PeerId { value: 42 }
}

fn verify_device_name(name: &cxx::CxxString) -> String {
    let safe_rust_str: &str = name.to_str().unwrap();

    // we allocate a brand new string on the Rust heap
    let mut final_name = String::from(safe_rust_str);

    // mutate the string by appending text
    final_name.push_str(" (Verified)");

    println!("We allocated a new string: {}", final_name);

    final_name
}

fn process_payload(payload: &cxx::CxxVector<u8>) -> Vec<u8> {
    println!("Rust received a C++ payload of {} bytes", payload.len());

    // allocate a brand new vec on Rust heap
    let mut response_packet = Vec::new();

    // we can iterate over the C++ vector exactly like a Rust slice
    for byte in payload.iter() {
        response_packet.push(*byte);
    }

    // append a new byte (our mock checksum)
    response_packet.push(0xFF);

    response_packet
}

fn connect_to_device(id: u64) -> Result<(), String> {
    println!("Rust is attempting to connect to device ID: {}", id);

    if id == 42 {
        println!("Rust: Connection Successful!");
        Ok(())
    } else {
        Err(format!("Device {} refused connection. Invalid credentials.", id))
    }
}


fn main() {
    ffi::simulate_discovery();
}