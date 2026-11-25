#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>
/* fast atoll: same text -> same value as strtoll (leading
 * whitespace, optional sign, digits, stop at the first other
 * byte, 0 when there are none); 19+ digits go to atoll so the
 * overflow result is libc's too. SH2_NO_ATOI_FAST=1 restores
 * plain atoll. */
static inline long long _sh_atoi(const char *s) {
  const unsigned char *p = (const unsigned char *)(s ? s : "");
  int neg = 0, nd = 0;
  unsigned long long v = 0;
  while (*p == ' ' || (*p >= 9 && *p <= 13)) p++;
  if (*p == '+' || *p == '-') { neg = (*p == '-'); p++; }
  if (*p < '0' || *p > '9') return 0;
  do {
    if (++nd > 18) return atoll(s);
    v = v * 10 + (unsigned long long)(*p - '0');
  } while (*++p >= '0' && *p <= '9');
  return neg ? -(long long)v : (long long)v;
}
#define atoll _sh_atoi

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
char* var = NULL;

int main(void) {
    static char _s0[32];
    _sh_ll2s(_s0, (long long)((atoll((var ? var : "")) + 1)));
    printf("%s\n", (char*)(_s0));
    free(var);
    return 0;
}