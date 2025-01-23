## semver playground

This repo serves as a playground to try out different combinations of crates with `links` and `extern` global symbols.

* pac - contains `__EXTERNAL_INTERRUPTS` in the API (see the multiple pac test readme for why this isn't a global symbol)
    * versions: 0.1, 0.2
* hal - contains global `PERIPHERALS_TAKEN` and uses the `__EXTERNAL_INTERRUPTS` from the pac
    * versions: 1.0.0, 1.1.0
* hal-lib - a project that depends on the hal, with some public functions to do some good things, and some bad things for testing purposes

### Tests

Each test project in the tests folder has a readme discussing the results and how to play with the test scenario a bit to try and answer https://github.com/esp-rs/esp-hal/issues/2589.