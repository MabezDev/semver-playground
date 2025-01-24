## multiple hal semver trick

This test is an expansion of the multiple-hal test, please read the results in the README there for context here.

### Results

As touched on in multiple-hal, it's possible for two major versions of esp-hal to coexist, and infact, it's even possible for their exported types to be compatible, as this repo demonstrates.

```md
multiple-hal-sermver-trick v0.1.0 (/home/mabez/development/rust/cargo-semver-tests/tests/multiple-hal-semver-trick)
├── hal v2.0.0
│   └── pac v0.2.0
└── hal-lib v0.1.0
    └── hal v1.3.0
        ├── hal v2.0.0 (*)
        └── pac v0.2.0
```

hal@1.3.0 exports the `Peripherals` type from hal@2.0.0, meaning hal-lib@0.1.0 is can use `Peripherals` from esp-hal@1.x _or_ esp-hal@2.x. We have a lot of options here, and provided we're careful now, and clever in future we'll never need to break our ecosystem with huge major changes.

Now onto the issues. If you tried to run or build this test, you'll get a linker error like this (the fact we get to linkage means the our trick above worked, and cargo/rust deemed them semver compatible):

```
"/home/mabez/development/rust/cargo-semver-tests/tests/multiple-hal-semver-trick/target/debug/deps/multiple_hal_sermver_trick-a906676d8b898399" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: rust-lld: error: duplicate symbol: PERIPHERALS_TAKEN
          >>> defined at lib.rs:5 (/home/mabez/development/rust/cargo-semver-tests/tests/multiple-hal-semver-trick/../../fake-registry/hal-1.3.0/src/lib.rs:5)
          >>>            hal-246c1e17bf7e69a3.hal.805581ab71e5e738-cgu.0.rcgu.o:(PERIPHERALS_TAKEN) in archive /home/mabez/development/rust/cargo-semver-tests/tests/multiple-hal-semver-trick/target/debug/deps/libhal-246c1e17bf7e69a3.rlib
          >>> defined at lib.rs:5 (/home/mabez/development/rust/cargo-semver-tests/tests/multiple-hal-semver-trick/../../fake-registry/hal-2.0.0/src/lib.rs:5)
          >>>            hal-9e85f62e8fee2da3.hal.e2564bc72b17dd59-cgu.0.rcgu.o:(.bss.PERIPHERALS_TAKEN+0x0) in archive /home/mabez/development/rust/cargo-semver-tests/tests/multiple-hal-semver-trick/target/debug/deps/libhal-9e85f62e8fee2da3.rlib
          collect2: error: ld returned 1 exit status
          

error: could not compile `multiple-hal-sermver-trick` (bin "multiple-hal-sermver-trick") due to 1 previous error
```

Which is why I believe we need to introduce a `rt` feature to esp-hal to seperate the "library" parts from the "end user" parts. There is precidence for this all over the ecosystem, namely tokio: https://github.com/tokio-rs/tokio/blob/ee19b0ed7371b069112b9c9ef9280b81f3438d26/tokio/Cargo.toml#L73.