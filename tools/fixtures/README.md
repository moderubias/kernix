# Bounded Miri wrapper checks

`python tools/check_miri.py` runs this standalone downstream consumer with Miri;
`--native` executes the identical fixture with the ordinary test harness.

The independent f32/f64 oracles cover dense, padded, negative and zero strides,
nonzero origins, aliased read-only inputs, empty contractions, malformed spans,
incorrect output lengths, overwritten output and surrounding canaries. Padding
contains NaNs so accidental reads are observable. Inputs remain unchanged.

Miri disables architecture-specific SIMD selection. Repeating large matrix
multiplications in the interpreter does not execute those SIMD kernels: the
previous full native suite took over three hours. Keep interpreter cases small;
the complete native shapes, dispatch boundaries, scale/error checks and large
convolution fixtures continue to execute unchanged in four-platform CI and ASan.
Kernix-core storage/layout tests still run in full under Miri.
