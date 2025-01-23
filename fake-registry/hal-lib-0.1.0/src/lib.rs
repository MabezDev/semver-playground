

use hal as _;


// this should be fine
pub fn lib_thing() {
    assert!(true);
}

/// Calling this in a lib should fail to link
pub fn bad_lib_thing() {
    assert!(hal::take());
}
