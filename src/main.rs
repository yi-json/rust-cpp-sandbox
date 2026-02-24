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
    }
}

fn assign_peer_id(address: ffi::DeviceAddressBytes) -> ffi::PeerId {
    println!("{:?}", address.bytes);
    ffi::PeerId { value: 42 }
}


fn main() {
    ffi::simulate_discovery();
}