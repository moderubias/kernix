/* Benchmark-only shim: resolve SDK constants without guessing their ABI values. */
#include <Accelerate/Accelerate.h>
int kernix_bench_single_thread(void) {
    if (BLASSetThreading(BLAS_THREADING_SINGLE_THREADED) != 0) return -1;
    return BLASGetThreading() == BLAS_THREADING_SINGLE_THREADED ? 0 : -2;
}
