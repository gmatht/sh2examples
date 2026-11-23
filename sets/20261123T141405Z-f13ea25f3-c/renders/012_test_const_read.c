#include <stdio.h>
#include <limits.h>
#include <stdint.h>
#include <string.h>
#include <gmp.h>
#define _SH_MPZ_INIT(name) static mpz_t name; static int name##_i = 0; if (!name##_i) { mpz_init(name); name##_i = 1; }
mpz_t _bigint_10;
static inline void _bigint_init(void) {
  mpz_init_set_str(_bigint_10, "10", 10);
}

#include <gmp.h>
/* fast long long -> decimal: byte-for-byte what snprintf("%lld")
 * writes for every long long (incl. LLONG_MIN) — no format parse,
 * no locale machinery (SH2_NO_ITOA=1 restores snprintf). */
/* The per-digit `u / 10` is this helper's whole cost under tcc: `%`
 * and `/` each lower to a 64-bit `div` — two of them per digit, in
 * the hottest function of the MIMEcroft render.  One `mulq` by
 * 0xCCCC...CD >> 3 does both jobs (the remainder follows from
 * `u - q*10`). SH2_NO_ITOA_FASTDIV=1 restores the plain spelling. */
#ifndef _SH_DIV10
#if defined(__x86_64__) && (defined(__GNUC__) || defined(__clang__) || defined(__TINYC__))
#define _SH_DIV10(u, q) \
  do { unsigned long long _d10_u = (unsigned long long)(u), \
       _d10_m = 14757395258967641293ull, _d10_lo, _d10_hi; \
       __asm__("mulq %3" : "=a"(_d10_lo), "=d"(_d10_hi) : "a"(_d10_u), "r"(_d10_m) : "cc"); \
       (q) = _d10_hi >> 3; } while (0)
#else
#define _SH_DIV10(u, q) do { (q) = (unsigned long long)(u) / 10; } while (0)
#endif
#endif
static inline void _sh_ll2s(char *d, long long v) {
  char t[24]; int i = 24; int neg = (v < 0);
  unsigned long long u = neg ? (unsigned long long)(-(v + 1)) + 1 : (unsigned long long)v;
  do { unsigned long long _d10_q; _SH_DIV10(u, _d10_q);
       t[--i] = (char)('0' + (u - _d10_q * 10)); u = _d10_q; } while (u);
  if (neg) t[--i] = '-';
  { size_t n = (size_t)(24 - i); memcpy(d, t + i, n); d[n] = 0; }
}


char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
  _bigint_init();
    uint8_t i = 0;
    {
        i = 0;
        while ((i < 10)) {
            static char _s1[32];
            _sh_ll2s(_s1, (long long)(i));
            fputs((char*)(_s1), stdout);
            i++;
        }
    }
    printf("%s\n", (char*)(""));
      mpz_clear(_bigint_10);
    return 0;
}