#![no_std]

#[unsafe(no_mangle)]
#[used]
static mut PERIPHERALS_TAKEN: bool = false;

pub use hal::Peripherals;

/// Takes the peripherals (equivalent to calling esp_hal::init)
pub fn take() -> Peripherals {
    unsafe {
        // use a the global from the pac
        pac::__EXTERNAL_INTERRUTPTS[0] = 0xAA;

        if PERIPHERALS_TAKEN {
            panic!("Taken twice!");
        }
        PERIPHERALS_TAKEN = true;

        assert_eq!(pac::__EXTERNAL_INTERRUTPTS[0], 0xAA);
    }

    Peripherals
}
