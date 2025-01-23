

fn main() {
    assert!(hal::take());
    hal_lib::lib_thing();
    hal_lib::bad_lib_thing();
    println!("Hello, world!");
}
