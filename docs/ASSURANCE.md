# Compatibility and numerical assurance

Rust CI tests Linux x86-64/ARM64, Windows and macOS, plus MSRV 1.86 and
wasm/i686/aarch64 compile checks. A compile check is not hardware execution or a
performance result. The manually dispatched/monthly hardware evidence workflow
records the runner's actual CPU/backend and retains raw distributions and readable
reports as artifacts. Hosted runner labels do not guarantee Intel versus AMD;
dedicated machines and repeatable ARM tuning remain work to do.

`python tools/check_api.py` compares all three packages against the immutable
v0.9.0 source, both with all features and without default features. The explicit
minor policy permits additions and rejects removals/signature incompatibilities
independently of the next 0.x version. Its JSON report records baseline/current
revisions, tool version, commands, exit status and complete diagnostics. The
underlying tool currently has no stable structured findings output, so diagnostics
are retained as text rather than parsed into guessed API changes. Static checking
cannot establish mathematical or performance compatibility.

`python tools/check_advisories.py` audits both lockfiles and retains unfiltered
RustSec JSON. The library lockfile has no vulnerability exceptions. Both dependency
trees currently include the unmaintained `paste` warning RUSTSEC-2024-0436 through
numerical dependencies; this warning is visible, not a clean-maintenance claim.

Candle 0.11 requires zip 8.6. The comparator harness constructs synthetic tensors
and does not load or extract archives. A narrow, expiring development-only exception
covers RUSTSEC-2026-0367/0368/0369 for exactly zip 8.6.0, until 2026-11-22. Full
findings remain in the audit artifact. Unknown advisories, another package/version,
runtime occurrences and expiry fail the gate. Track the [upstream release issue](https://github.com/huggingface/candle/issues/4043);
remove exceptions when a compatible release uses patched zip. Do not alter a
comparator's numerical source or silently substitute a framework version.

AddressSanitizer exercises native checked wrappers, decomposition regressions, pivoted minimum-norm solves and
condition/refinement diagnostics, contiguous contractions, explicit parallel execution and
owned gradient accumulation on nightly. The scheduled/manual fuzz workflow
uses two bounded libFuzzer targets: shape/layout round trips and QR/Cholesky
reconstruction, orthogonality and refinement monotonicity. Checked-in seeds are
separate from the evolving cached corpus; failures retain reproducing inputs.
Fuzzing is an additional source of counterexamples, not proof of correctness.

```sh
cargo +nightly fuzz run layout -- -max_total_time=60 -max_len=512
cargo +nightly fuzz run factorizations -- -max_total_time=60 -max_len=512
```

The coverage job pins Rust 1.99.0 / cargo-llvm-cov 0.9.1 and requires at least
85% library source line coverage in the all-features Linux GNU suite, excluding
integration-test and example source. The measured initial baseline is 85.44%
(7,263 / 8,501 lines); the floor rounds this down and prevents material regressions.
This development-tool version does not raise the library's Rust 1.86 MSRV.
The job cleans coverage artifacts before measurement so obsolete source maps do
not inflate the denominator after edits. Raw per-file summaries remain artifacts. Lines, generic instantiations, branches,
assembly and numerical-domain coverage are different quantities: this is not a
branch/MC/DC gate or proof that every mathematical case is tested. Error formatting,
configuration, fixed-rank forwarding and neural module wrappers have remaining gaps.
A broad sanitizer/target matrix and a stable structured API findings format remain
follow-ups. A passing workflow must not imply those exist.
