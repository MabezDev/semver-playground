

fn main() {
    let peripherals = hal::take();
    hal_lib::lib_thing(peripherals);
    hal_lib::bad_lib_thing();
    println!("Hello, world!");
}
