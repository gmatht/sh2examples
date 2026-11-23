#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <stdint.h>
#include <string.h>
/* checked strdup (P1 OOM policy): fail-stop, never NULL-on-OOM */
static char *_sh_xstrdup(const char *s) {
  char *p = strdup(s);
  if (s && !p) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
  return p;
}

/* in-place store (SH2_NO_SSET=1 keeps the legacy _old temp-swap):
 * the legacy store freed the old buffer, so nothing may legally alias
 * it — copying into it when the new value fits skips a malloc/free
 * pair; memmove covers src aliasing the buffer (self-ref RHS).
 * (sh2-perf: the fit test is strnlen-capped at n-1 — the verdict
 * `strlen(old)+1 >= n` needs only that the first n-1 bytes hold no
 * NUL, so the scan stops at n-1 = strlen(src) instead of walking
 * the whole old buffer: the demo's #1 strlen-byte site.)
 * SH2_NO_SCANFIX=1 restores the strlen(old) form.)
 */
static inline void _sh_sset(char **slot, const char *src) {
  char *old = *slot;
  if (old && src) { size_t n = strlen(src) + 1; if (strnlen(old, n - 1) == n - 1) { memmove(old, src, n); return; } }
  char *p = _sh_xstrdup((char*)(src ? src : ""));
  free(old);
  *slot = p;
}

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
static size_t _alen_tsv;

uint8_t i = 0;

int main(void) {
    char* tsv = NULL;
    _sh_sset(&tsv, (char*)(""));
    _alen_tsv = tsv ? (size_t)strlen(tsv) : 0;
    static const long long _for_i_0[] = {1, 2, 3, 4, 5};
    for (size_t _i_i = 0; _i_i < 5; _i_i++) {
        i = _for_i_0[_i_i];
        static char _s1[32];
        _sh_ll2s(_s1, (long long)(i));
        { const char *_a0_0 = (_s1); size_t _l0_0 = strlen(_a0_0); size_t _o0 = _alen_tsv; size_t _n0 = _l0_0 + 1; char *_np0 = (char*)realloc((tsv), (_o0 + _n0 + 1) * 2); if (!_np0) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); } (tsv) = _np0; char *_w0 = (tsv) + _o0; memcpy(_w0, _a0_0, _l0_0); _w0 += _l0_0; memcpy(_w0, ",", 1); _w0 += 1; _w0[0] = 0; _alen_tsv = _o0 + _n0; }
    }
    printf("%s\n", (char*)((tsv ? tsv : "")));
    free(tsv);
    return 0;
}