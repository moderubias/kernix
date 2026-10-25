/* Optional SDK bridge. Enum values, BLAS ABI and availability come from Apple headers. */
#define ACCELERATE_NEW_LAPACK
#include <Accelerate/Accelerate.h>
#include <stdint.h>

int kernix_accelerate_0_12_available(void) {
    if (__builtin_available(macOS 15.0, *)) {
        const __typeof__(BLASGetThreading()) previous = BLASGetThreading();
        if (BLASSetThreading(BLAS_THREADING_SINGLE_THREADED) != 0) return -1;
        return BLASSetThreading(previous);
    }
    return -1;
}

#define KERNIX_GEMM(NAME, TYPE, BLAS) \
int NAME(int m, int n, int k, const TYPE *a, const TYPE *b, TYPE *out) { \
    if (__builtin_available(macOS 15.0, *)) { \
        const __typeof__(BLASGetThreading()) previous = BLASGetThreading(); \
        if (BLASSetThreading(BLAS_THREADING_SINGLE_THREADED) != 0) return -1; \
        if (BLASGetThreading() != BLAS_THREADING_SINGLE_THREADED) { \
            (void)BLASSetThreading(previous); return -2; \
        } \
        BLAS(CblasRowMajor, CblasNoTrans, CblasNoTrans, m, n, k, \
             (TYPE)1, a, k, b, n, (TYPE)0, out, n); \
        return BLASSetThreading(previous); \
    } \
    return -1; \
}
KERNIX_GEMM(kernix_accelerate_0_12_f32, float, cblas_sgemm)
KERNIX_GEMM(kernix_accelerate_0_12_f64, double, cblas_dgemm)

int kernix_accelerate_0_12_threading(void) {
    if (__builtin_available(macOS 15.0, *)) return (int)BLASGetThreading();
    return -1;
}
