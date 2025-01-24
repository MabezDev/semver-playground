

fn main() {
    let peripherals = hal::take(); // 2.0.0 peripherals
    hal_lib::lib_thing(peripherals);
    println!("Hello, world!");
}
