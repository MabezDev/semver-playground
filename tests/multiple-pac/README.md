## multiple pac versions

### Results

Before we discuss the results we must first clear up some confusion around the pacs `__EXTERNAL_INTERRUPTS` symbol. It was brought up here by Bjoern: https://github.com/esp-rs/esp-hal/issues/2589#issuecomment-2586999434, which at first glance does seem correct. I did wonder how I missed this the first time though, and it's because on Xtensa we _don't_: https://github.com/esp-rs/esp-pacs/blob/d28d5656821ab8ba5d937a4f53ae552a975390ac/esp32s3/src/lib.rs#L120. In my opinion, Xtensa is doing this correctly. A public static doesn't need to be no_mangle, because the dependent crate, the hal, can directly reference that static! I'll file an issue about this svd2rust codegen shortly. On to the results.

By depending directly on the static from the pac, we are able to mix pac versions cleanly without the links field.

```md
multiple-pac v0.1.0 (/home/mabez/development/rust/cargo-semver-tests/tests/multiple-pac)
├── hal v1.0.0
│   └── pac v0.1.0
└── pac v0.2.0
```

At this point the pac becomes a truly private dependency. If we add a links field, this test would fail.