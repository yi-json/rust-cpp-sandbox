pub struct MyRustDevice {
    state: u32,
}

impl MyRustDevice {
    pub fn new(initial_state: u32) -> Box<MyRustDevice> {
        println!("Rust: Allocating MyRustDevice on the Rust heap...");
        Box::new(MyRustDevice { state: initial_state })
    }

    pub fn tick(&mut self) {
        self.state += 1;
        println!("Rust: Device ticked. Internal state mutated to: {}", self.state);
    }

    pub fn get_state(&self) -> u32 {
        self.state
    }
}