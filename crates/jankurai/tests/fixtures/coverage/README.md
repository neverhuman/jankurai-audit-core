# Coverage Audit Fixtures

These fixtures are small deterministic producer outputs for `jankurai coverage audit`.
They are hand-written parser fixtures, not generated audit artifacts.

`cargo_llvm_cov_0_8_7.info` is the exception: real `cargo llvm-cov 0.8.7
--lcov` output for a small crate, with only its `SF:` path rewritten to the
fixture repo. LLVM derives `LF`/`LH` from its line summary, so it declares
`LF:32`/`LH:31` over 31 `DA` records (30 covered).
