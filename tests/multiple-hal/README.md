## multiple hal versions

### Results

From my testing on a MSRV aware cargo version, it is seemingly impossible to have two different 1.0 hal crates in the same tree. Cargo will _always_ resolve it to the latest possible version, which is evident from the `cargo tree` output:

```md
multiple-hal v0.1.0 (/home/mabez/development/rust/cargo-semver-tests/tests/multiple-hal)
├── hal v1.1.0
│   └── pac v0.2.0
└── hal-lib v0.1.0
    └── hal v1.1.0 (*)
```

If a library that depends on esp-hal does something really naughty, like `version = "=1.0.0"` in it's cargo.toml, cargo will outright reject it without the even having the links field in place (you can try this for yourself by modifying fake-registry/hal-lib.0.1.0's cargo toml, and doing a clean build (cargo doesn't expect registry crates to change :D)).

The fact that only one _semver compatible version_ of esp-hal version will ever exist (to be really explicit here: only a single 1.x version or 2.x version will exist in the tree at one time.) it means that even if the hal-lib crate tries to interact with runtime things two things can happen:

* Just 1.x version: we won't get a linker error, just a runtime panic in the case of `take`ing peripherals. (you can see this by running the test)
* 1.x and 2.x version: We'll get a linker error because of duplicate `PERIPHERALS_TAKEN` symbols (see multiple-hal-semver-trick)

Based on these facts, I don't believe `esp-hal` _needs_ a links field. It would also cause complications if we ever release esp-hal@2.0.0 because now users _have_ to upgrade all their deps to 2.0.0 even though they might not need to, as most libraries should only be depending on esp-hal types and APIs and _not_ runtime internals, we can likely release 2.0.0 with [semver trick](https://github.com/dtolnay/semver-trick) to allow esp-hal@1.0.0 and esp-hal@2.0.0 to coexist. We should probably reintroduce a runtime feature so that library crates can't call `esp_hal::init` etc. Essentially split away the runtime specifics from libraries such that only the end user calls `esp_hal::init`. I explore this in multiple-hal-semver-trick test.