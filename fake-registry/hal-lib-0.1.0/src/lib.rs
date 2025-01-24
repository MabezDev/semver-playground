

// this should be fine
pub fn lib_thing(p: hal::Peripherals) {
    assert!(true, "{:?}", p)
}

/// Calling this in a lib should fail to link _if_ there are two versions of esp-hal in the tree, else it will result in a runtime error
/// This is obviously bad practice, but should be explored regardless.
pub fn bad_lib_thing() {
    _ = hal::take();
}
