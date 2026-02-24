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
        fn assign_peer_id(address: DeviceAddressBytes) -> PeerId;
        fn verify_device_name(name: &CxxString) -> String;
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


fn main() {
    ffi::simulate_discovery();
}