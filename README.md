## Semver playground

This repo serves as a playground to try out different combinations of crates with `links` and `extern` global symbols.

### extern packages

* pac - contains `__EXTERNAL_INTERRUPTS` in the API
* hal - contains global `PERIPHERALS_TAKEN` and uses the `__EXTERNAL_INTERRUPTS` from the pac