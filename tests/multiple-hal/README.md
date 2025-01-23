## mutliple hal versions

### Results

From my testing on a MSRV aware cargo version, it is seemingly impossible to have two different 1.0 hal crates in the same tree. Cargo will _always_ resolve it to the latest possible version, which is evident from the `cargo tree` output:

```md
multiple-hal v0.1.0 (/home/mabez/development/rust/cargo-semver-tests/tests/multiple-hal)
├── hal v1.1.0
│   └── pac v0.2.0
└── hal-lib v0.1.0
    └── hal v1.1.0 (*)
```

If a library that depends on esp-hal does something really naughty, like `version = "=1.0.0"` in it's cargo.toml, cargo will outright reject it without the even having the links field in place (you can try this for yourself by modifying fake-registry/hal-lib.0.1.0's cargo toml).

The fact that only one esp-hal version will ever exist in means that even if the hal-lib crate tries to interact with runtime things, we won't get a linker error, just a runtime panic in the case of `take`ing peripherals.

Based on these facts, I don't believe `esp-hal` _needs_ a links field. It would also cause complications if we ever release esp-hal@2.0. However, we may want to think about reintroducing a runtime feature so that library crates can't call `esp_hal::init` for example.