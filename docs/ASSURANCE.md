# Compatibility and numerical assurance

Rust CI tests Linux x86-64/ARM64, Windows and macOS, plus MSRV 1.86 and
wasm/i686/aarch64 compile checks. A compile check is not hardware execution or a
performance result. The manually dispatched/monthly hardware evidence workflow
records the runner's actual CPU/backend and retains raw distributions and readable
reports as artifacts. Hosted runner labels do not guarantee Intel versus AMD;
dedicated machines and repeatable ARM tuning remain work to do.

`python tools/check_api.py` compares all three packages against the immutable
v0.7.3 source, both with all features and without default features. The explicit
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

AddressSanitizer exercises native checked wrappers, decomposition regressions and
condition/refinement diagnostics on nightly. The scheduled/manual fuzz workflow
uses two bounded libFuzzer targets: shape/layout round trips and QR/Cholesky
reconstruction, orthogonality and refinement monotonicity. Checked-in seeds are
separate from the evolving cached corpus; failures retain reproducing inputs.
Fuzzing is an additional source of counterexamples, not proof of correctness.

```sh
cargo +nightly fuzz run layout -- -max_total_time=60 -max_len=512
cargo +nightly fuzz run factorizations -- -max_total_time=60 -max_len=512
```

Coverage thresholds, a broad sanitizer/target matrix and a stable structured API
findings format remain follow-ups. A passing workflow must not imply those exist.
