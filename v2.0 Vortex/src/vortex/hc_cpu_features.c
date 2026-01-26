#include "vortex/internal/hc_cpu_features.h"

#if defined(__x86_64__) || defined(_M_X64) || defined(__i386) ||               \
    defined(_M_IX86)

#if defined(_MSC_VER)
#include <intrin.h>
static void hc_cpuidex(int out[4], int leaf, int subleaf) {
  __cpuidex(out, leaf, subleaf);
}
static unsigned long long hc_xgetbv(unsigned int idx) { return _xgetbv(idx); }
#else
#include <cpuid.h>
static void hc_cpuidex(int out[4], int leaf, int subleaf) {
  unsigned int a, b, c, d;
  __cpuid_count((unsigned int)leaf, (unsigned int)subleaf, a, b, c, d);
  out[0] = (int)a;
  out[1] = (int)b;
  out[2] = (int)c;
  out[3] = (int)d;
}
static unsigned long long hc_xgetbv(unsigned int idx) {
  unsigned int eax, edx;
  __asm__ volatile(".byte 0x0f, 0x01, 0xd0" : "=a"(eax), "=d"(edx) : "c"(idx));
  return ((unsigned long long)edx << 32) | eax;
}
#endif

#include <stdatomic.h>

static atomic_bool g_inited = false;
static bool g_avx512f = false;
static bool g_avx512dq = false;
static bool g_avx512ifma = false;

static bool hc_os_supports_zmm_state(void) {
  // OS must enable XSAVE and preserve ZMM/YMM state (XCR0 bits 1,2,5,6,7)
  // Bits: XMM=1, YMM=2, Opmask=5, ZMM_hi256=6, Hi16_ZMM=7
  unsigned long long xcr0 = hc_xgetbv(0);
  const unsigned long long mask =
      (1ULL << 1) | (1ULL << 2) | (1ULL << 5) | (1ULL << 6) | (1ULL << 7);
  return (xcr0 & mask) == mask;
}

void hc_cpu_features_init(void) {
  bool expected = false;
  if (!atomic_compare_exchange_strong(&g_inited, &expected, true)) {
    return; // already inited
  }

  int info[4] = {0};
  hc_cpuidex(info, 0, 0);
  int max_leaf = info[0];

  if (max_leaf < 7) {
    g_avx512f = g_avx512dq = g_avx512ifma = false;
    return;
  }

  // Leaf 1: check OSXSAVE and AVX presence
  hc_cpuidex(info, 1, 0);
  const bool osxsave = (info[2] & (1 << 27)) != 0;
  const bool avx = (info[2] & (1 << 28)) != 0;

  if (!(osxsave && avx && hc_os_supports_zmm_state())) {
    g_avx512f = g_avx512dq = g_avx512ifma = false;
    return;
  }

  // Leaf 7 subleaf 0: extended features
  hc_cpuidex(info, 7, 0);
  g_avx512f = (info[1] & (1 << 16)) != 0;    // EBX bit 16
  g_avx512dq = (info[1] & (1 << 17)) != 0;   // EBX bit 17
  g_avx512ifma = (info[1] & (1 << 21)) != 0; // EBX bit 21
}

bool hc_cpu_has_avx512f(void) {
  hc_cpu_features_init();
  return g_avx512f;
}
bool hc_cpu_has_avx512dq(void) {
  hc_cpu_features_init();
  return g_avx512dq;
}
bool hc_cpu_has_avx512ifma(void) {
  hc_cpu_features_init();
  return g_avx512ifma;
}

#else

void hc_cpu_features_init(void) {}
bool hc_cpu_has_avx512f(void) { return false; }
bool hc_cpu_has_avx512dq(void) { return false; }
bool hc_cpu_has_avx512ifma(void) { return false; }

#endif
