## semver playground

This repo serves as a playground to try out different combinations of crates with `links` and `extern` global symbols.

* pac - contains `__EXTERNAL_INTERRUPTS` in the API (see the multiple pac test readme for why this isn't a global symbol)
* hal - contains global `PERIPHERALS_TAKEN` and uses the `__EXTERNAL_INTERRUPTS` from the pac