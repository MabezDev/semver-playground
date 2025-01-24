## semver playground

This repo serves as a playground to try out different combinations of crates with `extern` global symbols.

* pac - contains `__EXTERNAL_INTERRUPTS` in the API (see the multiple pac test readme for why this shouldn't be a global symbol)
    * versions: 0.1.0, 0.2.0
* hal - contains global `PERIPHERALS_TAKEN` and uses the `__EXTERNAL_INTERRUPTS` from the pac
    * versions: 1.0.0, 1.1.0, 1.3.0 (which rexports 2.0.0 public items), 2.0.0
* hal-lib - a project that depends on the hal, with some public functions to do some good things, and some bad things for testing purposes
    * versions: 0.1.0

### Tests

Each test project in the tests folder has a readme discussing the results and how to play with the test scenario a bit to try and answer https://github.com/esp-rs/esp-hal/issues/2589.

### Adding more tests

If you want to play around with this, to add new packages to `fake-registry` you can:

1) Run `cargo package --allow-dirty` in the package folder
2) Move the generated package into `fake-registry`
3) Once inside `fake-registry`, cargo expects a check sum file called `.cargo-checksum.json` with the following content: `{"files":{}}`, if you're on a unix system you can run `echo '{"files":{}}' > .cargo-checksum.json`

also note, any modifictions to the registry packages (to play around with MSRV etc) will require a clean build, cargo doesn't expected "released" packages to change.