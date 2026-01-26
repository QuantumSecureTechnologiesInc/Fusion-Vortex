# Optimisation Verification Benchmark

This benchmark validates the performance benefits of the quaternion‑level optimisation strategies implemented in the HyperCycle math boost layer.  It compares naive sequential rotation with a pre‑composed rotation and reports the speed‑up.  To run the benchmark, compile `hc_opt_benchmark.c` and execute the resulting binary.

## Building the benchmark

```
gcc -O3 hc_opt_benchmark.c -I../include -o hc_bench
```

On systems without `gcc` you can use any C compiler that supports C11.  The benchmark depends only on `hc_math_boost.h` and standard C headers.

## Running

```
./hc_bench
```

The program prints the time required to rotate a vector five million times using two different methods:

1. **Sequential rotations:** apply two quaternion rotations one after the other for every iteration.
2. **Combined rotations:** precompute a single quaternion representing the composition of the two rotations and apply it once per iteration.

By comparing the elapsed times, the benchmark computes a speed‑up factor.  A speed‑up greater than 1.2× indicates that the rotation composition optimisation is effective.

## Source code

The benchmark source is provided below for reference:

```c
#include <stdio.h>
#include <time.h>
#include "hc_math_boost.h"

double get_time_sec() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec + ts.tv_nsec * 1e-9;
}

int main() {
    const int ITERATIONS = 5000000;
    hc_quat_t rot1 = {HC_SCALE, 100, 200, 300};
    hc_quat_t rot2 = {HC_SCALE, 50, -50, 50};
    hc_quat_t vec  = {0, 1000, 0, 0};
    hc_quat_t result;

    /* Sequential rotations */
    double start_seq = get_time_sec();
    for (int i = 0; i < ITERATIONS; i++) {
        hc_quat_t temp;
        hc_quat_rotate(&rot1, &vec, &temp);
        hc_quat_rotate(&rot2, &temp, &result);
        vec.w ^= result.w & 1; // inhibit optimisation
    }
    double time_seq = get_time_sec() - start_seq;

    /* Combined rotation */
    double start_comb = get_time_sec();
    hc_quat_t combined;
    hc_quat_compose_rotations(&rot2, &rot1, &combined);
    for (int i = 0; i < ITERATIONS; i++) {
        hc_quat_rotate(&combined, &vec, &result);
        vec.w ^= result.w & 1;
    }
    double time_comb = get_time_sec() - start_comb;

    printf("Sequential Time: %.6f s\n", time_seq);
    printf("Combined Time:   %.6f s\n", time_comb);
    printf("Speedup Factor:  %.2fx\n", time_seq / time_comb);
    return 0;
}
```

The program uses high‑resolution timers to measure execution time and prints a summary at the end.

