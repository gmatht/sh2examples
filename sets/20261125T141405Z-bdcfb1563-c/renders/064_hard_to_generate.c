#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <stdint.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <ctype.h>
#include <sys/stat.h>
#include <gmp.h>
#define _SH_MPZ_INIT(name) static mpz_t name; static int name##_i = 0; if (!name##_i) { mpz_init(name); name##_i = 1; }
mpz_t _bigint_1000, _bigint_2;
static inline void _bigint_init(void) {
  mpz_init_set_str(_bigint_1000, "1000", 10);
  mpz_init_set_str(_bigint_2, "2", 10);
}

#include <sys/stat.h>
#include <fnmatch.h>
#include <regex.h>
/* [[ s =~ re ]] — POSIX ERE match */
static int _sh_regex_match(const char *s, const char *re) {
  regex_t rx;
  if (regcomp(&rx, re, REG_EXTENDED | REG_NOSUB) != 0) return 0;
  int rc = regexec(&rx, s, 0, 0, 0);
  regfree(&rx);
  return rc == 0;
}

#include <assert.h>
#include <gmp.h>
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
typedef struct { char *p; size_t cap; } _sh_vbuf;
static inline char *_sh_vgrow(_sh_vbuf *b, size_t need) {
  if (need <= b->cap && b->p) return b->p;
  size_t ncap = b->cap ? b->cap : 256;
  while (ncap < need) ncap *= 2;
  char *np = (char*)realloc(b->p, ncap);
  if (!np) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
  b->p = np; b->cap = ncap;
  return np;
}
/* sh2 runtime: exit-status register + shell-out helpers */
static int _sh_rc;
static int _sh_argc;
static char **_sh_argv = 0;
#define SH2_ARENA_CAP (256 * 1024)
typedef struct { char buf[SH2_ARENA_CAP]; size_t used; } _sh_arena;
static char *_sh_adup(_sh_arena *a, const char *s) {
  size_t n = strlen(s) + 1;
  if (a->used + n > SH2_ARENA_CAP) return _sh_xstrdup(s);
  char *r = a->buf + a->used; memcpy(r, s, n); a->used += n; return r;
}
static inline void _sh_arena_reset(_sh_arena *a) { a->used = 0; }
typedef struct { char *p; } _sh_mstr;
static inline void _sh_mstr_set(_sh_mstr *v, const char *val) {
  free(v->p);
  v->p = val ? _sh_xstrdup(val) : NULL;
}
static const char *_sh_mstr_get(const _sh_mstr *v) {
  return (v->p) ? v->p : "";
}
static char *_sh_cmd = 0; static size_t _sh_cap = 0;
static char *_sh_wb = 0; static size_t _sh_wcap = 0;
static char *_sh_wrap = 0; static size_t _sh_wrapcap = 0;
static inline void _sh_grow(char **b, size_t *cap, size_t need) {
  if (need <= *cap) return;
  *cap = need * 2;
  char *np = (char*)realloc(*b, *cap);
  if (!np) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
  *b = np;
}
static inline void _sh_add(const char *s) {
  size_t l = _sh_cmd ? strlen(_sh_cmd) : 0, n = strlen(s);
  _sh_grow(&_sh_cmd, &_sh_cap, l + n + 1);
  if (_sh_cmd) memcpy(_sh_cmd + l, s, n + 1);
}
static inline void _sh_addc(char c) {
  size_t l = _sh_cmd ? strlen(_sh_cmd) : 0;
  _sh_grow(&_sh_cmd, &_sh_cap, l + 2);
  _sh_cmd[l] = c; _sh_cmd[l + 1] = 0;
}
static inline void _sh_reset(void) { _sh_grow(&_sh_cmd, &_sh_cap, 1); _sh_cmd[0] = 0; }
static inline void _sh_wb_add(const char *s) {
  size_t l = _sh_wb ? strlen(_sh_wb) : 0, n = strlen(s);
  _sh_grow(&_sh_wb, &_sh_wcap, l + n + 1);
  if (_sh_wb) memcpy(_sh_wb + l, s, n + 1);
}
static inline void _sh_wb_reset(void) { _sh_grow(&_sh_wb, &_sh_wcap, 1); _sh_wb[0] = 0; }
typedef struct { const char *p; size_t idx; } _sh_sline;
static int _sh_numkey_cmp(const char *a, const char *b) {
  while (*a == ' ' || *a == '\t') a++;
  while (*b == ' ' || *b == '\t') b++;
  int sa = 1, sb = 1;
  if (*a == '-') { sa = -1; a++; }
  if (*b == '-') { sb = -1; b++; }
  while (*a == '0') a++;
  while (*b == '0') b++;
  const char *ia = a; while (*a >= '0' && *a <= '9') a++;
  const char *ib = b; while (*b >= '0' && *b <= '9') b++;
  size_t la = (size_t)(a - ia), lb = (size_t)(b - ib);
  const char *fa = 0, *fb = 0; size_t lfa = 0, lfb = 0;
  if (*a == '.') { a++; fa = a; while (*a >= '0' && *a <= '9') a++; lfa = (size_t)(a - fa); }
  if (*b == '.') { b++; fb = b; while (*b >= '0' && *b <= '9') b++; lfb = (size_t)(b - fb); }
  int za = 1, zb = 1, k;
  for (k = 0; (size_t)k < la; k++) if (ia[k] != '0') za = 0;
  for (k = 0; (size_t)k < lb; k++) if (ib[k] != '0') zb = 0;
  for (k = 0; (size_t)k < lfa; k++) if (fa[k] != '0') za = 0;
  for (k = 0; (size_t)k < lfb; k++) if (fb[k] != '0') zb = 0;
  if (za && zb) return 0;
  if (za) return -sb; if (zb) return sa;
  if (sa != sb) return sa > sb ? 1 : -1;
  int m = 0;
  if (la != lb) m = la > lb ? 1 : -1;
  else if ((m = memcmp(ia, ib, la)) != 0) m = m > 0 ? 1 : -1;
  else {
    size_t fl = lfa > lfb ? lfa : lfb, f;
    for (f = 0; f < fl; f++) {
      int da = f < lfa ? fa[f] - '0' : 0;
      int db = f < lfb ? fb[f] - '0' : 0;
      if (da != db) { m = da > db ? 1 : -1; break; }
    }
  }
  return sa > 0 ? m : -m;
}
static long long *_sh_sj_keys = 0;
static size_t _sh_sj_keycap;
static int _sh_sj_fast;
static int _sh_sline_cmp(const void *pa, const void *pb) {
  const _sh_sline *a = (const _sh_sline *)pa, *b = (const _sh_sline *)pb;
  int m;
  if (_sh_sj_fast) {
    long long x = _sh_sj_keys[a->idx], y = _sh_sj_keys[b->idx];
    m = x < y ? -1 : x > y ? 1 : 0;
  } else m = _sh_numkey_cmp(a->p, b->p);
  if (m != 0) return m;
  return a->idx < b->idx ? -1 : a->idx > b->idx ? 1 : 0;
}
/* canonical plain integer fitting i64? (blanks, one '-',
   digits, blanks — anything else, including '+', '.', or
   overflow, fails so the general path handles it) */
static int _sh_parse_i64(const char *s, long long *out) {
  while (*s == ' ' || *s == '\t') s++;
  int neg = 0;
  if (*s == '-') { neg = 1; s++; }
  if (*s < '0' || *s > '9') return 0;
  unsigned long long mag = 0;
  while (*s >= '0' && *s <= '9') {
    unsigned d = (unsigned)(*s - '0');
    if (mag > (ULLONG_MAX - d) / 10) return 0;
    mag = mag * 10 + d; s++;
  }
  while (*s == ' ' || *s == '\t') s++;
  if (*s != 0) return 0;
  if (!neg) {
    if (mag > (unsigned long long)LLONG_MAX) return 0;
    *out = (long long)mag;
  } else {
    if (mag > (unsigned long long)LLONG_MAX + 1ULL) return 0;
    *out = (mag == (unsigned long long)LLONG_MAX + 1ULL) ? LLONG_MIN : -(long long)mag;
  }
  return 1;
}
static char *_sh_sorted_join(_sh_vbuf *d, char **a, size_t n) {
  static _sh_vbuf _sj = {0, 0};
  static _sh_sline *_sj_ln = 0; static size_t _sj_lncap = 0;
  size_t i, ln = 0;
  char *dp = _sh_vgrow(d, 1); dp[0] = 0;
  _sh_vgrow(&_sj, 1); _sj.p[0] = 0;
  size_t tot = 1, pos = 0;
  for (i = 0; i < n; i++) tot += (a[i] ? strlen(a[i]) : 0) + 1;
  _sh_vgrow(&_sj, tot);
  for (i = 0; i < n; i++) {
    const char *s = a[i] ? a[i] : "";
    size_t l = strlen(s);
    memcpy(_sj.p + pos, s, l); pos += l;
    _sj.p[pos++] = '\n';
  }
  _sj.p[pos] = 0;
  /* split on '\n', NUL-terminating the scratch (the array is

     never mutated); then drop the post-terminator artifact

     piece so [a] and [a, ""] come out as [a] and [a, ""]:
     printf '%s\n' "a" emits "a\n" (one line), while "a\n"\n
     (element with trailing newline) emits lines "a" and "". */
  i = 0;
  while (i <= pos) {
    size_t j = i;
    while (j < pos && _sj.p[j] != '\n') j++;
    _sj.p[j] = 0;
    if (ln + 1 >= _sj_lncap) {
      size_t nc = _sj_lncap ? _sj_lncap * 2 : 64;
      while (nc < ln + 1) nc *= 2;
      _sh_sline *np = (_sh_sline *)realloc(_sj_ln, nc * sizeof *_sj_ln);
      if (!np) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
      _sj_ln = np; _sj_lncap = nc;
    }
    _sj_ln[ln].p = _sj.p + i; _sj_ln[ln].idx = ln;
    ln++;
    i = j + 1;
  }
  if (ln > 0) ln--; /* post-terminator artifact */
  /* tier: every line a canonical plain i64 int? Then
     parse-once keys + trivial compare (no GMP, no per-
     comparison parsing). Otherwise the general string
     path above (mpq can't parse sort keys anyway: it
     rejects every decimal point). */
  _sh_sj_fast = 1;
  size_t kk;
  for (kk = 0; kk < ln; kk++) {
    if (_sh_sj_keycap <= kk) {
      size_t nc = _sh_sj_keycap ? _sh_sj_keycap * 2 : 64;
      while (nc <= kk) nc *= 2;
      long long *np = (long long *)realloc(_sh_sj_keys, nc * sizeof *_sh_sj_keys);
      if (!np) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
      _sh_sj_keys = np; _sh_sj_keycap = nc;
    }
    if (!_sh_parse_i64(_sj_ln[kk].p, &_sh_sj_keys[kk])) { _sh_sj_fast = 0; break; }
  }
  if (ln > 0) qsort(_sj_ln, ln, sizeof *_sj_ln, _sh_sline_cmp);
  size_t dn = 0, k;
  _sh_vgrow(d, 1);
  for (k = 0; k < ln; k++) {
    int same;
    if (k == 0) same = 0;
    else if (_sh_sj_fast) same = (_sh_sj_keys[_sj_ln[k].idx] == _sh_sj_keys[_sj_ln[k-1].idx]);
    else same = (_sh_numkey_cmp(_sj_ln[k].p, _sj_ln[k-1].p) == 0);
    if (k > 0 && same) continue;
    size_t l = strlen(_sj_ln[k].p);
    _sh_vgrow(d, dn + (dn ? 2 : 0) + l + 1);
    if (dn) { d->p[dn++] = ','; d->p[dn++] = ' '; }
    memcpy(d->p + dn, _sj_ln[k].p, l); dn += l;
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
  return d->p;
}
static char *_sh_mpz_str(_sh_vbuf *d, const mpz_t v) {
  char *s = mpz_get_str(0, 10, v);
  if (!s) { char *dp = _sh_vgrow(d, 1); dp[0] = 0; return dp; }
  size_t n = strlen(s);
  _sh_vgrow(d, n + 1);
  memcpy(d->p, s, n + 1);
  free(s);
  return d->p;
}

/* append s as ONE shell word (single-quoted, so no re-expansion) */
static inline void _sh_word(const char *s) {
  _sh_add(" '");
  for (const char *p = s; *p; p++) {
    if (*p == '\'') _sh_add("'\"'\"'"); else _sh_addc(*p);
  }
  _sh_addc('\'');
}
/* append raw text (no quoting) - for already-shell-safe pieces */
static inline void _sh_addraw(const char *s) { _sh_add(" "); _sh_add(s); }
/* buffer-parameterized variants (capture sites' private buffers) */
static inline void _sh_badd(char **b, size_t *cap, const char *s) {
  size_t l = *b ? strlen(*b) : 0, n = strlen(s);
  _sh_grow(b, cap, l + n + 1);
  if (*b) memcpy(*b + l, s, n + 1);
}
static inline void _sh_baddc(char **b, size_t *cap, char c) {
  size_t l = *b ? strlen(*b) : 0;
  _sh_grow(b, cap, l + 2);
  if (*b) { (*b)[l] = c; (*b)[l + 1] = 0; }
}
static inline void _sh_bres(char **b, size_t *cap) { _sh_grow(b, cap, 1); if (*b) (*b)[0] = 0; }
static inline void _sh_bword(char **b, size_t *cap, const char *s) {
  _sh_badd(b, cap, " '");
  for (const char *p = s; *p; p++) {
    if (*p == '\'') _sh_badd(b, cap, "'\"'\"'"); else _sh_baddc(b, cap, *p);
  }
  _sh_baddc(b, cap, '\'');
}
static inline void _sh_idx_init(char **b, size_t *cap, const char *name, char **a, size_t n) {
  _sh_badd(b, cap, " "); _sh_badd(b, cap, name); _sh_badd(b, cap, "=(");
  for (size_t i = 0; i < n; i++) if (a[i]) _sh_bword(b, cap, a[i]);
  _sh_badd(b, cap, ")");
}
static inline void _sh_idx_init_ll(char **b, size_t *cap, const char *name, const long long *a, const unsigned char *pres, size_t n) {
  _sh_badd(b, cap, " "); _sh_badd(b, cap, name); _sh_badd(b, cap, "=(");
  for (size_t i = 0; i < n; i++) {
    if (pres && !pres[i]) continue;
    char ent[40]; snprintf(ent, sizeof ent, "[%zu]=%lld ", i, a[i]); _sh_badd(b, cap, ent);
  }
  _sh_badd(b, cap, ")");
}
/* single-quote append WITHOUT the leading space (assoc [k]=v pairs) */
static inline void _sh_baddq(char **b, size_t *cap, const char *s) {
  _sh_badd(b, cap, "'");
  for (const char *p = s; *p; p++) {
    if (*p == '\'') _sh_badd(b, cap, "'\"'\"'"); else _sh_baddc(b, cap, *p);
  }
  _sh_baddc(b, cap, '\'');
}
static inline void _sh_baddqn(char **b, size_t *cap, const char *s) {
  if (!s) return;
  if (!strchr(s, '\n')) { _sh_badd(b, cap, s); return; }
  _sh_badd(b, cap, "'");
  for (const char *p = s; *p; p++) {
    if (*p == '\'') _sh_badd(b, cap, "'\"'\"'"); else _sh_baddc(b, cap, *p);
  }
  _sh_baddc(b, cap, '\'');
}
static inline void _sh_addqn(const char *s) {
  if (!s) return;
  if (!strchr(s, '\n')) { _sh_add(s); return; }
  _sh_add("'");
  for (const char *p = s; *p; p++) {
    if (*p == '\'') _sh_add("'\"'\"'"); else _sh_addc(*p);
  }
  _sh_addc('\'');
}
static inline void _sh_assoc_init(char **b, size_t *cap, const char *name, char **k, char **v, size_t n) {
  _sh_badd(b, cap, " "); _sh_badd(b, cap, name); _sh_badd(b, cap, "=(");
  for (size_t i = 0; i < n; i++) {
    if (!k[i]) continue;
    if (i > 0) _sh_badd(b, cap, " ");
    _sh_badd(b, cap, "["); _sh_baddq(b, cap, k[i]); _sh_badd(b, cap, "]=");
    _sh_baddq(b, cap, v[i] ? v[i] : "");
  }
  _sh_badd(b, cap, ")");
}
static inline void _sh_arr_set(char **a, size_t *len, size_t cap, long long i, const char *v) {
  if (i < 0 || !v) return;
  if (i >= (long long)cap) { fprintf(stderr, "sh2c: capacity exceeded\n"); exit(127); }
  { char *_nv = (char*)_sh_xstrdup(v); free(a[i]); a[i] = _nv; }
  if ((size_t)(i + 1) > *len) *len = (size_t)(i + 1);
}
static char *_sh_trash[64];
static int _sh_trashn;
static inline void _sh_defer_free(char *p) {
  if (!p) return;
  if (_sh_trashn < 64) { _sh_trash[_sh_trashn++] = p; return; }
  free(p);
}
static inline void _sh_flush_trash(void) {
  while (_sh_trashn > 0) free(_sh_trash[--_sh_trashn]);
}
static inline void _sh_arr_take(char **a, size_t *len, size_t cap, long long i, const char *v) {
  if (i < 0 || !v) return;
  if (i >= (long long)cap) { fprintf(stderr, "sh2c: capacity exceeded\n"); exit(127); }
  { char *_nv = (char*)_sh_xstrdup(v); _sh_defer_free(a[i]); a[i] = _nv; }
  if ((size_t)(i + 1) > *len) *len = (size_t)(i + 1);
}
static inline void _sh_assoc_set(char **k, char **v, size_t *n, size_t cap, const char *key, const char *val) {
  if (!key) return;
  for (size_t i = 0; i < *n; i++)
    if (k[i] && strcmp(k[i], key) == 0) { char *_nv = (char*)_sh_xstrdup(val ? val : ""); free(v[i]); v[i] = _nv; return; }
  if (*n < cap) { k[*n] = (char*)_sh_xstrdup(key); v[*n] = (char*)_sh_xstrdup(val ? val : ""); (*n)++; }
  else { fprintf(stderr, "sh2c: capacity exceeded\n"); exit(127); }
}
static inline void _sh_assoc_take(char **k, char **v, size_t *n, size_t cap, const char *key, const char *val) {
  if (!key) return;
  for (size_t i = 0; i < *n; i++)
    if (k[i] && strcmp(k[i], key) == 0) { char *_nv = (char*)_sh_xstrdup(val ? val : ""); _sh_defer_free(v[i]); v[i] = _nv; return; }
  if (*n < cap) { k[*n] = (char*)_sh_xstrdup(key); v[*n] = (char*)_sh_xstrdup(val ? val : ""); (*n)++; }
  else { fprintf(stderr, "sh2c: capacity exceeded\n"); exit(127); }
}
static inline void _sh_join_arr(_sh_vbuf *d, char **a, size_t n) {
  size_t dn = 0; int _f = 1;
  for (size_t i = 0; i < n; i++) {
    if (!a[i]) continue;
    if (_f) { _f = 0; } else { _sh_vgrow(d, dn + 2); d->p[dn++] = ' '; }
    for (const char *s = a[i]; *s; s++) { _sh_vgrow(d, dn + 2); d->p[dn++] = *s; }
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
static inline void _sh_join_arr_sep(_sh_vbuf *d, char **a, size_t n, char sep) {
  size_t dn = 0; int _f = 1;
  for (size_t i = 0; i < n; i++) {
    if (!a[i]) continue;
    if (_f) { _f = 0; } else if (sep) { _sh_vgrow(d, dn + 2); d->p[dn++] = sep; }
    for (const char *s = a[i]; *s; s++) { _sh_vgrow(d, dn + 2); d->p[dn++] = *s; }
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
/* ${arr[@]:off:len} — an ELEMENT slice of the store */
static inline void _sh_join_arr_rng(_sh_vbuf *d, char **a, size_t n, long long off, long long len, char sep) {
  /* bash (MEASURED 2026-11-05, (off,len) grid over four sparse shapes):
     the OFFSET is a SLOT INDEX (negative counts slots back from
     last_index+1), but the LENGTH counts PRESENT ELEMENTS -- the
     window starts at that slot and scans forward, skipping holes
     WITHOUT spending the length. a=(a b c); a[5]=z gives
     ${a[*]:1:3}=b c z and :0:2=a b; a[3]=x; a[7]=y gives :0:2=x y.
     Out-of-range offset -> EMPTY, never a clamped slice;
     array len<0 -> expansion error, no value */
  long long b = off < 0 ? ((off < -(long long)n) ? -1 : (long long)n + off) : off;
  if (b < 0 || b >= (long long)n) { _sh_vgrow(d, 1); d->p[0] = 0; return; }
  if (len < 0) { fprintf(stderr, "%lld: substring expression < 0\n", len); exit(1); }
  size_t dn = 0; int _f = 1; long long taken = 0;
  for (size_t i = (size_t)b; i < n; i++) {
    if (!a[i]) continue;
    if (taken >= len) break;
    if (_f) { _f = 0; } else if (sep) { _sh_vgrow(d, dn + 2); d->p[dn++] = sep; }
    for (const char *s = a[i]; *s; s++) { _sh_vgrow(d, dn + 2); d->p[dn++] = *s; }
    taken++;
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
static inline void _sh_join_keys(_sh_vbuf *d, char **k, size_t n, char sep) {
  size_t dn = 0; int _f = 1;
  for (size_t i = 0; i < n; i++) {
    if (!k[i]) continue;
    if (_f) { _f = 0; } else if (sep) { _sh_vgrow(d, dn + 2); d->p[dn++] = sep; }
    for (const char *s = k[i]; *s; s++) { _sh_vgrow(d, dn + 2); d->p[dn++] = *s; }
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
static inline void _sh_join_ikind(_sh_vbuf *d, char **a, size_t n, char sep) {
  size_t dn = 0; int _f = 1;
  for (size_t i = 0; i < n; i++) {
    if (!a[i]) continue;
    if (_f) { _f = 0; } else if (sep) { _sh_vgrow(d, dn + 2); d->p[dn++] = sep; }
    char _b[24]; int _l = snprintf(_b, sizeof _b, "%zu", i);
    for (int _j = 0; _j < _l; _j++) { _sh_vgrow(d, dn + 2); d->p[dn++] = _b[_j]; }
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
static inline void _sh_join_idx(_sh_vbuf *d, const unsigned char *s, size_t n, char sep) {
  size_t dn = 0; int _f = 1;
  for (size_t i = 0; i < n; i++) {
    if (s && !s[i]) continue;
    if (_f) { _f = 0; } else if (sep) { _sh_vgrow(d, dn + 2); d->p[dn++] = sep; }
    char _b[24]; int _l = snprintf(_b, sizeof _b, "%zu", i);
    for (int _j = 0; _j < _l; _j++) { _sh_vgrow(d, dn + 2); d->p[dn++] = _b[_j]; }
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
static long long *_sh_sij_keys = 0; static size_t _sh_sij_keycap = 0;
static int _sh_sij_cmp(const void *pa, const void *pb) {
  size_t a = *(const size_t *)pa, b = *(const size_t *)pb;
  long long x = _sh_sij_keys[a], y = _sh_sij_keys[b];
  if (x != y) return x < y ? -1 : 1;
  return a < b ? -1 : a > b;
}
static inline void _sh_sorted_int_join(_sh_vbuf *d, char **a, size_t n) {
  static size_t *_sh_sij_ix = 0; static size_t _sh_sij_ixcap = 0;
  size_t i;
  char *dp = _sh_vgrow(d, 1); dp[0] = 0;
  if (_sh_sij_keycap < n) {
    size_t nc = _sh_sij_keycap ? _sh_sij_keycap : 64;
    while (nc < n) nc *= 2;
    long long *np = (long long *)realloc(_sh_sij_keys, nc * sizeof *_sh_sij_keys);
    if (!np) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
    _sh_sij_keys = np; _sh_sij_keycap = nc;
  }
  if (_sh_sij_ixcap < n) {
    size_t nc = _sh_sij_ixcap ? _sh_sij_ixcap : 64;
    while (nc < n) nc *= 2;
    size_t *np = (size_t *)realloc(_sh_sij_ix, nc * sizeof *_sh_sij_ix);
    if (!np) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
    _sh_sij_ix = np; _sh_sij_ixcap = nc;
  }
  for (i = 0; i < n; i++) { _sh_sij_keys[i] = atoll(a[i] ? a[i] : "0"); _sh_sij_ix[i] = i; }
  if (n > 0) qsort(_sh_sij_ix, n, sizeof *_sh_sij_ix, _sh_sij_cmp);
  size_t dn = 0, k;
  for (k = 0; k < n; k++) {
    size_t j = _sh_sij_ix[k];
    if (k > 0 && _sh_sij_keys[j] == _sh_sij_keys[_sh_sij_ix[k-1]]) continue;
    const char *s = a[j] ? a[j] : "";
    size_t l = strlen(s);
    _sh_vgrow(d, dn + (dn ? 2 : 0) + l + 1);
    if (dn) { d->p[dn++] = ','; d->p[dn++] = ' '; }
    memcpy(d->p + dn, s, l); dn += l;
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
static long long *_sh_sij_i64_v = 0;
static int _sh_sij_i64_cmp(const void *pa, const void *pb) {
  size_t a = *(const size_t *)pa, b = *(const size_t *)pb;
  long long x = _sh_sij_i64_v[a], y = _sh_sij_i64_v[b];
  if (x != y) return x < y ? -1 : 1;
  return a < b ? -1 : a > b;
}
static inline void _sh_sorted_inti64_join(_sh_vbuf *d, long long *a, size_t n) {
  static size_t *_sh_sij_ix = 0; static size_t _sh_sij_ixcap = 0;
  size_t i;
  char *dp = _sh_vgrow(d, 1); dp[0] = 0;
  if (_sh_sij_ixcap < n) {
    size_t nc = _sh_sij_ixcap ? _sh_sij_ixcap : 64;
    while (nc < n) nc *= 2;
    size_t *np = (size_t *)realloc(_sh_sij_ix, nc * sizeof *_sh_sij_ix);
    if (!np) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
    _sh_sij_ix = np; _sh_sij_ixcap = nc;
  }
  _sh_sij_i64_v = a;
  for (i = 0; i < n; i++) _sh_sij_ix[i] = i;
  if (n > 0) qsort(_sh_sij_ix, n, sizeof *_sh_sij_ix, _sh_sij_i64_cmp);
  size_t dn = 0, k;
  for (k = 0; k < n; k++) {
    size_t j = _sh_sij_ix[k];
    if (k > 0 && a[j] == a[_sh_sij_ix[k-1]]) continue;
    char _s[32];
    snprintf(_s, sizeof _s, "%lld", (long long)a[j]);
    size_t l = strlen(_s);
    _sh_vgrow(d, dn + (dn ? 2 : 0) + l + 1);
    if (dn) { d->p[dn++] = ','; d->p[dn++] = ' '; }
    memcpy(d->p + dn, _s, l); dn += l;
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
static char *_sh_argv_join(_sh_vbuf *d, char sep) {
  char *dp = _sh_vgrow(d, 1); dp[0] = 0; size_t n = 0;
  for (int i = 1; i < _sh_argc; i++) {
    if (i > 1 && sep) { _sh_vgrow(d, n + 2); d->p[n++] = sep; }
    const char *s = _sh_argv[i];
    while (s && *s) { _sh_vgrow(d, n + 2); d->p[n++] = *s++; }
  }
  _sh_vgrow(d, n + 1); d->p[n] = 0; return d->p;
}
/* ${@:off:len} — positional-param slice (off is 1-based; off 0 includes $0) */
static char *_sh_argv_slice(_sh_vbuf *d, long long off, long long len, char sep) {
  long long total = (long long)_sh_argc;
  long long b = off < 0 ? ((off < -total) ? -1 : total + off) : off;
  /* bash (MEASURED 2026-11-01): offset out of range -> EMPTY; positional
     len<0 errors for EVERY in-range b (0..total inclusive) */
  if (b < 0 || b > total) { char *q = _sh_vgrow(d, 1); q[0] = 0; return d->p; }
  if (len < 0) { fprintf(stderr, "%lld: substring expression < 0\n", len); exit(1); }
  long long e = len > total - b ? total : b + len; /* b<=total: no signed overflow */
  size_t dn = 0; long long i;
  char *dp = _sh_vgrow(d, 1); dp[0] = 0;
  for (i = b; i < e; i++) {
    if (i > b && sep) { _sh_vgrow(d, dn + 2); d->p[dn++] = sep; }
    const char *s = _sh_argv[i] ? _sh_argv[i] : "";
    while (*s) { _sh_vgrow(d, dn + 2); d->p[dn++] = *s++; }
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0; return d->p;
}
/* wrap the built command as `bash -c '<cmd>'` (single-quote escaped) */
static inline void _sh_wrap_cmd(const char *cmd) {
  size_t n = strlen(cmd), need = n * 2 + 16;
  _sh_grow(&_sh_wrap, &_sh_wrapcap, need);
  char *p = _sh_wrap; strcpy(p, "bash -c '"); p += 9;
  for (const char *c = cmd; *c; c++) {
    if (*c == '\'') { memcpy(p, "'\"'\"'", 5); p += 5; }
    else *p++ = *c;
  }
  *p++ = '\''; *p = 0;
}
/* `echo`/`:`, or `xxd -r -p IN`, with one `>`/`>>` redirect, run in
 * process (see the emit site). Returns 0 with _sh_rc set when the
 * command was a recognised shape, -1 otherwise — the caller then runs
 * system() unchanged. */
/* nibble table for the `xxd -r -p` decode: 0-15 nibble, 0xFF
 * separator whitespace, 0xFE any other byte (resets a half
 * pair, like xxd). */
static const unsigned char _sh_hexv[256] = {
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xff, 0xff, 0xfe, 0xfe, 0xff, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xff, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
};
static void _sh_fa(char **b, size_t *cap, size_t *len, const char *s, size_t n) {
  _sh_grow(b, cap, *len + n + 1);
  memcpy(*b + *len, s, n);
  *len += n;
  (*b)[*len] = 0;
}
static int _sh_echo_fast(const char *cmd) {
  char *w = 0, *a = 0, *t = 0;
  size_t wcap = 0, acap = 0, tcap = 0, wl = 0, al = 0, tl = 0;
  int nredir = 0, append = 0, nwords = 0, tgt = 0, bad = 0, seg = 0;
  const char *p = cmd;
  size_t vl;
  int is_echo, is_colon;
  FILE *f;
  if (!cmd || !*cmd) return -1;
  while (!bad) {
    while (*p == ' ' || *p == '\t') p++;
    if (!*p) break;
    /* a raw newline is a COMMAND SEPARATOR to the shell, never more
     * whitespace — falling back here keeps two commands from merging */
    if (*p == '\n') { bad = 1; break; }
    if (*p == '>') {
      /* `word>file` IS a plain redirect — bash splits the word at the `>`
       * too. The one glued form that is not is an fd number: `echo hi
       * 2>/dev/null` would otherwise take the `2` as an echo argument and
       * redirect the payload to /dev/null instead of discarding stderr.
       * So a glued `>` is refused only when the word it ends is all
       * digits. This backend's own word emitter separates the two with a
       * space (`_sh_addraw` prepends one), so the rule never fires on its
       * output — it exists for a raw concat (`_sh_add(">")`) and for any
       * other producer of the text. */
      if (p > cmd && p[-1] != ' ' && p[-1] != '\t' && wl > 0 && w) {
        size_t di = 0;
        while (di < wl && w[di] >= '0' && w[di] <= '9') di++;
        if (di == wl) { bad = 1; break; }
      }
      if (p[1] == '>') { p += 2; append = 1; } else { p += 1; append = 0; }
      if (*p == '>' || *p == '&' || *p == '<') { bad = 1; break; }
      if (++nredir > 1) { bad = 1; break; }
      tgt = 1;
      continue;
    }
    if (*p == '<' || *p == '&' || *p == '|' || *p == ';' || *p == '`' ||
        *p == '(' || *p == ')') { bad = 1; break; }
    /* one word = one or more ADJACENT segments (`'/a/' "$b" -c`) */
    wl = 0; seg = 0;
    if (w) w[0] = 0;
    for (;;) {
      if (*p == '\'') {
        const char *q = strchr(++p, '\'');
        if (!q) { bad = 1; break; }
        _sh_fa(&w, &wcap, &wl, p, (size_t)(q - p));
        p = q + 1; seg = 1;
      } else if (*p == '"') {
        int closed = 0;
        p++;
        while (*p && !bad) {
          if (*p == '"') { closed = 1; p++; break; }
          if (*p == '\\') {
            char c = p[1];
            if (!c || c == '\n') { bad = 1; break; }
            if (c == '$' || c == '`' || c == '"' || c == '\\') {
              _sh_fa(&w, &wcap, &wl, &c, 1); p += 2;
            } else { _sh_fa(&w, &wcap, &wl, "\\", 1); p++; }
            continue;
          }
          if (*p == '`') { bad = 1; break; }
          if (*p == '$') {
            const char *q = p + 1, *ns;
            char nm[256];
            size_t nlen;
            int br = 0;
            if (*q == '{') { br = 1; q++; }
            if (!(*q == '_' || (*q >= 'A' && *q <= 'Z') || (*q >= 'a' && *q <= 'z'))) { bad = 1; break; }
            q++;
            while (*q == '_' || (*q >= '0' && *q <= '9') || (*q >= 'A' && *q <= 'Z') || (*q >= 'a' && *q <= 'z')) q++;
            if (br) { if (*q != '}') { bad = 1; break; } }
            ns = p + 1 + (br ? 1 : 0);
            nlen = (size_t)(q - ns);
            if (nlen == 0 || nlen >= sizeof nm) { bad = 1; break; }
            memcpy(nm, ns, nlen); nm[nlen] = 0;
            { const char *val = getenv(nm); if (val) _sh_fa(&w, &wcap, &wl, val, strlen(val)); }
            p = q + (br ? 1 : 0);
            seg = 1;
            continue;
          }
          _sh_fa(&w, &wcap, &wl, p, 1); p++;
        }
        if (bad) break;
        if (!closed) { bad = 1; break; }
        seg = 1;
      } else if (!*p || *p == ' ' || *p == '\t' || *p == '\n' || *p == '>' ||
                 *p == '<' || *p == '&' || *p == '|' || *p == ';' || *p == '`' ||
                 *p == '(' || *p == ')') {
        /* a newline ends the word here; the next loop turn then bails */
        break;
      } else if (*p == '\\') {
        if (!p[1] || p[1] == '\n') { bad = 1; break; }
        _sh_fa(&w, &wcap, &wl, p + 1, 1); p += 2; seg = 1;
      } else if (*p == '$' || *p == '*' || *p == '?' || *p == '[' || *p == '~' ||
                 *p == '!' || *p == '{' || *p == '}') {
        bad = 1; break;
      } else {
        _sh_fa(&w, &wcap, &wl, p, 1); p++; seg = 1;
      }
    }
    if (bad || !seg) { bad = 1; break; }
    if (tgt) {
      if (tl) { bad = 1; break; }
      _sh_fa(&t, &tcap, &tl, w, wl);
      tgt = 0;
    } else {
      if (nwords) _sh_fa(&a, &acap, &al, " ", 1);
      _sh_fa(&a, &acap, &al, w, wl);
      nwords++;
    }
  }
  if (!bad && tgt == 0 && nredir == 1 && tl > 0 && nwords >= 1 && a) {
    /* `xxd -r -p IN` — the reverse-plain decode, in process: the
     * frame-present path runs it once per frame (~200KB of hex), and
     * forking bash + xxd for that was 4.7% of a first-frame run.
     * Semantics copied from xxd: ' ' TAB LF CR skip without disturbing
     * a pending nibble, any other non-hex byte discards it, a trailing
     * odd nibble is dropped. A file that does not open (xxd would exit
     * 2 with its own message) falls through to system() untouched. */
    if (nwords == 4 && al >= 10 && memcmp(a, "xxd -r -p ", 10) == 0) {
      FILE *in = fopen(a + 10, "rb");
      if (in) {
        f = fopen(t, append ? "ab" : "wb");
        if (f) {
          /* 64 KiB out: an fwrite this size skips stdio's buffer and is
           * ONE write(2), so a 300 KB frame leaves in ~5 syscalls, not
           * 75. A watcher polling the destination then finds a whole
           * frame (or a clean truncation) instead of a stream of
           * quarter-written ones, and the kernel stops paying a syscall
           * per 4 KB of every hex-decoded file. */
          unsigned char hbuf[65536], obuf[65536];
          size_t hgot, hi_i, on = 0;
          int hlo = -1, hd;
          while ((hgot = fread(hbuf, 1, sizeof hbuf, in)) > 0) {
            for (hi_i = 0; hi_i < hgot; hi_i++) {
              /* ONE table lookup per byte instead of a class chain: the
               * input is random hex, so the chain mispredicted on nearly
               * every byte. Measured on a 320x240 frame's hex (691KB):
               * 7.2 ms -> 1.6 ms per decode. Values < 16 are nibbles, 0xFF
               * is separator whitespace, 0xFE is any other byte. */
              unsigned hv = _sh_hexv[hbuf[hi_i]];
              if (hv < 16u) hd = (int)hv;
              else if (hv == 0xFEu) { hlo = -1; continue; }
              else continue;
              if (hlo < 0) hlo = hd;
              else {
                /* batched: one fwrite per 4KB, not per byte — a 1-byte
                 * fwrite costs a stdio call per pixel otherwise */
                obuf[on++] = (unsigned char)((hlo << 4) | hd);
                hlo = -1;
                if (on == sizeof obuf) { fwrite(obuf, 1, on, f); on = 0; }
              }
            }
          }
          if (on) fwrite(obuf, 1, on, f);
          fclose(f);
          fclose(in);
          _sh_rc = 0;
          free(w); free(a); free(t);
          return 0;
        }
        fclose(in);
      }
    }
    /* `cat SRC > DEST` and `tail -n +2 SRC > DEST` — file-to-file copies,
     * and between them the standard way a script pops the first line off a
     * queue file (`x=$(head -n 1 F)`, then `tail -n +2 F > T; cat T > F`).
     * Both forked a coreutils process per call; MIMEcroft pays them once
     * per key consumed, which is why its input phase measured 13 ms/frame
     * (60% of the loop) while a key was held. Only a plain single-operand
     * copy is taken: the source must be a regular file, the output is
     * truncated first exactly as the shell's redirect truncates, and an
     * unreadable source falls back to system() so bash's stderr text and
     * exit status are unchanged. */
    {
      const char *src = 0;
      int skip_first = 0;
      struct stat cst;
      if (nwords == 2 && al > 4 && memcmp(a, "cat ", 4) == 0) {
        src = a + 4;
      } else if (nwords == 4 && al > 11 && memcmp(a, "tail -n +2 ", 11) == 0) {
        src = a + 11;
        skip_first = 1;
      }
      if (src && *src && stat(src, &cst) == 0 && S_ISREG(cst.st_mode)) {
        FILE *in;
        f = fopen(t, append ? "ab" : "wb");
        if (f) {
          in = fopen(src, "rb");
          if (in) {
            char cbuf[8192];
            size_t cgot;
            int cerr = 0;
            if (skip_first) {
              int hit = 0;
              while (!hit && (cgot = fread(cbuf, 1, sizeof cbuf, in)) > 0) {
                size_t ci = 0;
                while (ci < cgot && cbuf[ci] != '\n') ci++;
                if (ci < cgot) {
                  hit = 1;
                  fwrite(cbuf + ci + 1, 1, cgot - ci - 1, f);
                }
              }
              cerr = ferror(in);
            }
            while (!cerr && (cgot = fread(cbuf, 1, sizeof cbuf, in)) > 0) {
              fwrite(cbuf, 1, cgot, f);
            }
            cerr = cerr || ferror(in);
            fclose(in);
            fclose(f);
            _sh_rc = cerr ? 1 : 0;
            free(w); free(a); free(t);
            return 0;
          }
          fclose(f);
        }
      }
    }
    /* `printf FORMAT WORD > FILE` — printf with ONE argument and a format
     * that is literal text plus `%s` plus the simple escapes (`\n`, `\t`,
     * `\r`, `\\`) has fully determined bytes, so it is a file WRITE and
     * not a shell command. MIMEcroft truncates its key queue with
     * `printf %s` once per key read, which forked bash per frame.
     *
     * Refused (system() path, byte-identical to before): a format with an
     * embedded space (the word join above cannot re-split it), any other
     * conversion or a width/precision flag, the `\0nnn`/`\xHH`/`\u`
     * escapes this does not reproduce, more than one argument, and any
     * argument count the join could have flattened. Every refusal keeps
     * bash's bytes, which is what `printf` would have printed. */
    if (nwords == 3 && al > 7 && memcmp(a, "printf ", 7) == 0) {
      const char *fmt = a + 7;
      const char *sp = strchr(fmt, ' ');
      if (sp && sp > fmt) {
        const char *arg = sp + 1;
        int ok = 1, ns = 0;
        for (const char *q = fmt; q < sp && ok; q++) {
          if (*q == '%') {
            if (q + 1 < sp && q[1] == 's') { ns++; q++; } else ok = 0;
          } else if (*q == '\\') {
            char e = q[1];
            if (e == 'n' || e == 't' || e == 'r' || e == '\\') q++;
            else ok = 0;
          }
        }
        if (ok && ns == 1) {
          f = fopen(t, append ? "ab" : "wb");
          if (f) {
            for (const char *q = fmt; q < sp; q++) {
              if (*q == '%') { fwrite(arg, 1, strlen(arg), f); q++; }
              else if (*q == '\\') {
                q++;
                fputc(*q == 'n' ? '\n' : *q == 't' ? '\t'
                    : *q == 'r' ? '\r' : '\\', f);
              } else fputc(*q, f);
            }
            fclose(f);
            _sh_rc = 0;
            free(w); free(a); free(t);
            return 0;
          }
        }
      }
    }
    vl = 0;
    while (a[vl] && a[vl] != ' ') vl++;
    is_echo = (vl == 4 && memcmp(a, "echo", 4) == 0);
    is_colon = (vl == 1 && a[0] == ':');
    /* only a first argument that cannot be an option: bash's echo would
     * honour -n/-e/-E there, and we always want its one trailing newline */
    if (is_colon || (is_echo && (nwords == 1 || a[vl + 1] != '-'))) {
      f = fopen(t, append ? "a" : "w");
      if (f) {
        if (is_echo) {
          if (nwords > 1) fwrite(a + vl + 1, 1, al - vl - 1, f);
          fputc('\n', f);
        }
        fclose(f);
        _sh_rc = 0;
        free(w); free(a); free(t);
        return 0;
      }
    }
  }
  free(w); free(a); free(t);
  return -1;
}
static int _sh_system_rc(void) {
  if (_sh_echo_fast(_sh_cmd ? _sh_cmd : "") >= 0) return _sh_rc;
  _sh_wrap_cmd(_sh_cmd ? _sh_cmd : "");
  int rc = system(_sh_wrap);
  _sh_rc = (rc == -1) ? 127 : (WIFEXITED(rc) ? WEXITSTATUS(rc) : 1);
  return _sh_rc;
}
static inline void _sh_run(void) { (void)_sh_system_rc(); }
/* run the built command, capture stdout, strip trailing newlines */
static inline void _sh_capture(_sh_vbuf *d, const char *cmd) {
  char *dp = _sh_vgrow(d, 1); dp[0] = 0;
  if (!cmd || !*cmd) { _sh_rc = 0; return; }
  if (strncmp(cmd, "head -n 1 \"$", 12) == 0) {
    const char *nm = cmd + 12; size_t nl = 0; int ok = 1;
    while (nm[nl] && nm[nl] != '\"') { if (!isalnum((unsigned char)nm[nl]) && nm[nl] != '_') ok = 0; nl++; }
    if (ok && nl > 0 && nl < 64 && nm[nl] == '\"' && nm[nl + 1] == 0) {
      char vn[80]; memcpy(vn, nm, nl); vn[nl] = 0;
      const char *path = getenv(vn);
      if (path) {
        FILE *hf = fopen(path, "r");
        if (hf) {
          size_t n = 0; int c;
          while ((c = fgetc(hf)) != EOF && c != '\n') { _sh_vgrow(d, n + 2); d->p[n++] = (char)c; }
          d->p[n] = 0;
          fclose(hf);
          while (n > 0 && d->p[n - 1] == '\r') d->p[--n] = 0;
          _sh_rc = 0; return;
        }
      }
    }
  }
  _sh_wrap_cmd(cmd);
  FILE *p = popen(_sh_wrap, "r");
  if (!p) { _sh_rc = 127; return; }
  size_t n = 0, r;
  do {
    _sh_vgrow(d, n + 8192);
    r = fread(d->p + n, 1, d->cap - n - 1, p);
    n += r;
  } while (r > 0);
  d->p[n] = 0;
  int rc = pclose(p);
  _sh_rc = (rc == -1) ? 127 : (WIFEXITED(rc) ? WEXITSTATUS(rc) : 1);
  while (n > 0 && (d->p[n - 1] == '\n' || d->p[n - 1] == '\r')) d->p[--n] = 0;
}
static char *_sh_capture_fn(void (*fn)(void), _sh_vbuf *vb) {
  _sh_vgrow(vb, 1); vb->p[0] = 0;
  FILE *t = tmpfile();
  if (!t) return vb->p;
  fflush(stdout);
  int saved = dup(1);
  dup2(fileno(t), 1);
  fn();
  fflush(stdout);
  dup2(saved, 1); close(saved);
  rewind(t);
  size_t n = 0, r;
  do {
    _sh_vgrow(vb, n + 8192);
    r = fread(vb->p + n, 1, vb->cap - n - 1, t);
    n += r;
  } while (r > 0);
  vb->p[n] = 0;
  fclose(t);
  while (n > 0 && (vb->p[n - 1] == '\n' || vb->p[n - 1] == '\r')) vb->p[--n] = 0;
  return vb->p;
}
static char *_sh_ret_val(_sh_vbuf *vb, const char *v) {
  size_t n = v ? strlen(v) : 0;
  char *p = _sh_vgrow(vb, n + 1);
  if (v) memcpy(p, v, n);
  p[n] = 0;
  while (n > 0 && (p[n - 1] == '\n' || p[n - 1] == '\r')) p[--n] = 0;
  return p;
}
static char *_sh_call_fn(void (*fn)(void), char **av, int n, _sh_vbuf *vb) {
  char **sv = _sh_argv; int sc = _sh_argc;
  char *scmd = _sh_cmd; size_t scap = _sh_cap; _sh_cmd = 0; _sh_cap = 0;
  _sh_argv = av; _sh_argc = n;
  _sh_capture_fn(fn, vb);
  _sh_argv = sv; _sh_argc = sc;
  _sh_cmd = scmd; _sh_cap = scap;
  return vb->p;
}
/* string-var ++/-- — function-call boundaries are sequence
   points, so two mutations in one expression stay ordered */
static long long _sh_postinc(char **v, int d) {
  long long r = atoll(*v ? *v : "");
  char b[32]; snprintf(b, sizeof b, "%lld", (long long)(r + d));
  free(*v); *v = _sh_xstrdup(b); return r;
}
static long long _sh_preinc(char **v, int d) {
  long long r = atoll(*v ? *v : "") + d;
  char b[32]; snprintf(b, sizeof b, "%lld", r);
  free(*v); *v = _sh_xstrdup(b); return r;
}
/* fixed-buffer forms: write in place (bound asserted elsewhere) */
static inline void _sh_import_env(const char *path) {
  FILE *p = fopen(path, "rb");
  if (!p) return;
  fseek(p, 0, SEEK_END); long sz = ftell(p); fseek(p, 0, SEEK_SET);
  if (sz <= 0) { fclose(p); return; }
  char *data = (char*)malloc((size_t)sz + 1);
  size_t rd = fread(data, 1, (size_t)sz, p); data[rd] = 0; fclose(p);
  char *q = data; char *end = data + rd;
  while (q < end) {
    size_t len = strnlen(q, (size_t)(end - q));
    char *eq = (char*)memchr(q, '=', len);
    if (eq && eq > q) {
      size_t nl = (size_t)(eq - q);
      int ok = isalpha((unsigned char)q[0]) || q[0] == '_';
      for (size_t k = 1; ok && k < nl; k++) ok = isalnum((unsigned char)q[k]) || q[k] == '_';
      if (ok) { char c = *eq; *eq = 0; setenv(q, eq + 1, 1); *eq = c; }
    }
    q += len + 1;
  }
  free(data);
}
/* eval / . : run the built command, then snapshot the child's
   post-command environment and import it so assignments persist */
static int _sh_state_run(void) {
  const char *suffix = " ; __shst_rc=$? ; env -0 > \"${_SH2ENVF}\" ; exit $__shst_rc";
  const char *pre = "set -a ; ";
  size_t l = _sh_cmd ? strlen(_sh_cmd) : 0, n = strlen(suffix), p0 = strlen(pre);
  _sh_grow(&_sh_cmd, &_sh_cap, l + n + p0 + 1);
  if (!_sh_cmd) return 127;
  memmove(_sh_cmd + p0, _sh_cmd, l + 1);
  memcpy(_sh_cmd, pre, p0);
  strcat(_sh_cmd, suffix);
  _sh_wrap_cmd(_sh_cmd);
  int rc = system(_sh_wrap);
  rc = (rc == -1) ? 127 : (WIFEXITED(rc) ? WEXITSTATUS(rc) : 1);
  const char *f = getenv("_SH2ENVF");
  if (f && *f) _sh_import_env(f);
  return rc;
}
/* field splitting (POSIX 2.5.6), IFS-DRIVEN — MEASURED 2026-11-04:
   a run of IFS WHITESPACE is ONE delimiter and is stripped at both
   ends; each IFS non-whitespace char delimits one field, so `a::b`
   has an empty middle field and `:a:` an empty first; an EMPTY IFS
   disables splitting. The pre-fix helper hard-coded space/tab/
   newline and never read IFS, so `IFS=:; for w in $v` gave 2 fields
   where bash gives 4 (`a::b: c` -> a, '', b, ' c').  */
static inline void _sh_wgrow(char ***w, size_t *cap, size_t need) {
  if (need <= *cap) return;
  size_t ncap = *cap ? *cap : 64;
  while (ncap < need) ncap *= 2;
  char **nw = (char**)realloc(*w, ncap * sizeof *nw);
  if (!nw) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
  *w = nw; *cap = ncap;
}
static int _sh_ifs_ws(char c) { return (c==' '||c=='\t'||c=='\n') && strchr(IFS, c) != 0; }
static int _sh_ifs_nw(char c) { return c && c!=' ' && c!='\t' && c!='\n' && strchr(IFS, c) != 0; }
static size_t _sh_split(char *buf, char ***w, size_t *wcap) {
  size_t n = 0; char *p = buf;
  if (!IFS[0]) { if (*p) { _sh_wgrow(w, wcap, n + 1); (*w)[n++] = p; } return n; }
  while (_sh_ifs_ws(*p)) p++;
  while (*p) {
    char *start = p; char d;
    while (*p && !_sh_ifs_ws(*p) && !_sh_ifs_nw(*p)) p++;
    d = *p; if (d) *p = 0;
    _sh_wgrow(w, wcap, n + 1); (*w)[n++] = start;
    if (!d) break;
    p++;
    if (_sh_ifs_ws(d)) {
      while (_sh_ifs_ws(*p)) p++;
      if (_sh_ifs_nw(*p)) p++;
      while (_sh_ifs_ws(*p)) p++;
    } else { while (_sh_ifs_ws(*p)) p++; }
  }
  return n;
}
/* export a var value so `$name` in a bash -c child sees it */
static inline void _sh_export(const char *name, const char *val) {
  struct _sh_xe { char *n; char *blk; size_t cap; };
  static struct _sh_xe _xe[64];
  static int _xn;
  const char *v = val ? val : "";
  size_t ln = strlen(name), lv = strlen(v);
  int k = -1;
  for (int i = 0; i < _xn; i++) if (strcmp(_xe[i].n, name) == 0) { k = i; break; }
  if (k < 0) {
    if (_xn >= 64) { setenv(name, v, 1); return; }
    k = _xn++; _xe[k].n = _sh_xstrdup(name); _xe[k].blk = 0; _xe[k].cap = 0;
  }
  if (lv + ln + 2 > _xe[k].cap) {
    _xe[k].cap = lv + ln + 2;
    _xe[k].blk = (char*)realloc(_xe[k].blk, _xe[k].cap);
    if (!_xe[k].blk) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
  }
  memcpy(_xe[k].blk, name, ln);
  _xe[k].blk[ln] = '=';
  memcpy(_xe[k].blk + ln + 1, v, lv + 1);
  unsetenv(name);
  putenv(_xe[k].blk);
}
static const char *_sh_env(const char *name) {
  if (name && name[0] == 'I' && strcmp(name, "IFS") == 0) return IFS;
  const char *v = getenv(name);
  return v ? v : "";
}
/* `read` builtin: one line from stdin into a static buffer */
static char *_sh_readline(void) {
  static _sh_vbuf _sh_rd = {0, 0};
  size_t n = 0;
  for (;;) {
    _sh_vgrow(&_sh_rd, n + 512);
    if (!fgets(_sh_rd.p + n, (int)(_sh_rd.cap - n), stdin)) break;
    n += strlen(_sh_rd.p + n);
    if (n > 0 && _sh_rd.p[n - 1] == '\n') break;
  }
  _sh_vgrow(&_sh_rd, n + 1); _sh_rd.p[n] = 0;
  while (n > 0 && (_sh_rd.p[n - 1] == '\n' || _sh_rd.p[n - 1] == '\r')) _sh_rd.p[--n] = 0;
  return _sh_rd.p;
}
static int _sh_fgets_line(FILE *f, _sh_vbuf *vb) {
  size_t n = 0;
  for (;;) {
    _sh_vgrow(vb, n + 512);
    if (!fgets(vb->p + n, (int)(vb->cap - n), f)) break;
    n += strlen(vb->p + n);
    if (n > 0 && vb->p[n - 1] == '\n') break;
  }
  _sh_vgrow(vb, n + 1); vb->p[n] = 0;
  return n > 0;
}
static int _sh_readline_from(const char *s, size_t *pos, _sh_vbuf *out) {
  if (!s) return 0;
  size_t n = strlen(s);
  if (*pos > n) return 0;
  if (*pos == n) {
    if (n == 0 || s[n - 1] == '\n') { _sh_vgrow(out, 1); out->p[0] = 0; *pos = n + 1; return 1; }
    return 0;
  }
  size_t i = *pos;
  while (i < n && s[i] != '\n') i++;
  size_t len = i - *pos;
  _sh_vgrow(out, len + 1);
  memcpy(out->p, s + *pos, len); out->p[len] = 0;
  *pos = (i < n) ? i + 1 : i;
  return 1;
}
/* ${x:off:len} substring (bash: off<0 counts from the end; len<0 counts
   from the STRING end and errors when it lands before the offset) */
/* ${arr[@]:off:len} — an ELEMENT slice of the space-joined string */
/* ${m[*]:off:len} / ${m[@]:off:len} — the ASSOC value slice: a ONE-BASED
   positive offset (0 and 1 both start at the first element, k>1 at
   k-1), out of range -> EMPTY, a negative offset clamped at 0, and a
   length that counts PRESENT elements (MEASURED 2026-11-05) */
static inline void _sh_assoc_rng(_sh_vbuf *d, char **a, size_t n, long long off, long long len, char sep) {
  if (off > (long long)n) { _sh_vgrow(d, 1); d->p[0] = 0; return; }
  long long b = off < 0 ? ((long long)n + off < 0 ? 0 : (long long)n + off)
                        : (off <= 1 ? 0 : off - 1);
  if (len < 0) { fprintf(stderr, "%lld: substring expression < 0\n", len); exit(1); }
  size_t dn = 0; int _f = 1; long long taken = 0;
  for (size_t i = (size_t)b; i < n; i++) {
    if (!a[i]) continue;
    if (taken >= len) break;
    if (_f) { _f = 0; } else if (sep) { _sh_vgrow(d, dn + 2); d->p[dn++] = sep; }
    for (const char *s = a[i]; *s; s++) { _sh_vgrow(d, dn + 2); d->p[dn++] = *s; }
    taken++;
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
static inline void _sh_arr_slice(_sh_vbuf *d, const char *s, long long off, long long len, char sep) {
  long long n = 0; const char *q;
  if (*s) { n = 1; for (q = s; *q; q++) if (*q == sep) n++; }
  long long b = off < 0 ? ((off < -(long long)n) ? -1 : (long long)n + off) : off;
  /* bash (MEASURED 2026-11-01): offset out of range -> EMPTY, never a
     clamped slice; array len<0 -> expansion error, no value */
  if (b < 0 || b >= (long long)n) { _sh_vgrow(d, 1); d->p[0] = 0; return; }
  if (len < 0) { fprintf(stderr, "%lld: substring expression < 0\n", len); exit(1); }
  long long e = len > (long long)n - b ? (long long)n : b + len; /* b<n: no signed overflow */
  size_t dn = 0; const char *p = s; long long i = 0;
  while (i < e) {
    const char *w = p;
    while (*p && *p != sep) p++;
    if (i >= b) {
      if (i > b) { _sh_vgrow(d, dn + 2); d->p[dn++] = sep; }
      size_t wl = (size_t)(p - w);
      _sh_vgrow(d, dn + wl + 1);
      memcpy(d->p + dn, w, wl); dn += wl;
    }
    i++;
    if (*p == sep) p++; else break;
  }
  _sh_vgrow(d, dn + 1); d->p[dn] = 0;
}
static char *_sh_substr(_sh_vbuf *d, const char *s, long long off, long long len) {
  size_t n = strlen(s);
  long long b = off < 0 ? ((off < -(long long)n) ? -1 : (long long)n + off) : off;
  /* bash (MEASURED 2026-11-01): offset out of range -> EMPTY; len<0
     counts from the STRING end (e = n+len, NOT b+n+len) and errors
     when the adjusted end falls before b */
  if (b < 0 || b > (long long)n) { _sh_vgrow(d, 1); d->p[0] = 0; return d->p; }
  long long e;
  if (len < 0) {
    if (len < -(long long)n) { fprintf(stderr, "%lld: substring expression < 0\n", len); exit(1); }
    e = (long long)n + len;
    if (e < b) { fprintf(stderr, "%lld: substring expression < 0\n", len); exit(1); }
  } else { e = len > (long long)n - b ? (long long)n : b + len; }
  if (e > (long long)n) e = (long long)n;
  size_t out = (size_t)(e - b);
  _sh_vgrow(d, out + 1);
  memcpy(d->p, s + b, out); d->p[out] = 0;
  return d->p;
}
/* ${s//pat/repl} - literal replace-all */
static char *_sh_replace(_sh_vbuf *d, const char *s, const char *pat, const char *repl) {
  size_t pn = strlen(pat), rn = strlen(repl), dn = 0;
  const char *p = s;
  _sh_vgrow(d, 1);
  if (!pn) { size_t n = strlen(s); _sh_vgrow(d, n + 1); memcpy(d->p, s, n + 1); return d->p; }
  while (*p) {
    const char *hit = strstr(p, pat);
    if (!hit) break;
    size_t pre = (size_t)(hit - p);
    _sh_vgrow(d, dn + pre + rn + 1);
    memcpy(d->p + dn, p, pre); dn += pre;
    memcpy(d->p + dn, repl, rn); dn += rn;
    p = hit + pn;
  }
  size_t rest = strlen(p);
  _sh_vgrow(d, dn + rest + 1);
  memcpy(d->p + dn, p, rest + 1);
  return d->p;
}
/* FIRST match only — bash ${var/pat/repl} */
static char *_sh_replace_first(_sh_vbuf *d, const char *s, const char *pat, const char *repl) {
  size_t pn = strlen(pat), rn = strlen(repl), dn = 0;
  const char *p = s;
  _sh_vgrow(d, 1);
  if (!pn) { size_t n = strlen(s); _sh_vgrow(d, n + 1); memcpy(d->p, s, n + 1); return d->p; }
  const char *hit = strstr(p, pat);
  if (!hit) { size_t n = strlen(s); _sh_vgrow(d, n + 1); memcpy(d->p, s, n + 1); return d->p; }
  size_t pre = (size_t)(hit - p);
  _sh_vgrow(d, pre + rn + strlen(hit + pn) + 1);
  memcpy(d->p, p, pre); dn = pre;
  memcpy(d->p + dn, repl, rn); dn += rn;
  p = hit + pn;
  strcpy(d->p + dn, p);
  return d->p;
}
/* $PIPESTATUS[i] — comma-joined stage statuses imported from the pipeline child */
static inline void _sh_mstr_set_from(char **slot, const char *val) {
  /* Heap ownership like the scalar COPY path: decls are
     NULL-init'd and every write here strdups, so freeing
     the old value is safe — and required, else every
     loop iteration leaks the previous line. Always
     strdup (never store a "" literal) so the slot never
     aliases static storage a later free would fault on. */
  free(*slot);
  *slot = _sh_xstrdup(val ? val : "");
}
/* ${s#pat}/${s##pat} prefix strip (glob-aware, greedy = longest) */
static char *_sh_strippre(_sh_vbuf *d, const char *s, const char *pat, int greedy) {
  static _sh_vbuf sc = {0, 0};
  size_t n = strlen(s);
  _sh_vgrow(&sc, n + 1); strcpy(sc.p, s);
  size_t best = 0;
  for (size_t i = 0; i <= n; i++) {
    char c = sc.p[i]; sc.p[i] = 0;
    if (fnmatch(pat, sc.p, 0) == 0) best = i;
    sc.p[i] = c;
    if (!greedy && best) break;
  }
  _sh_vgrow(d, n - best + 1);
  strncpy(d->p, sc.p + best, n - best); d->p[n - best] = 0;
  return d->p;
}
/* ${s%pat}/${s%%pat} suffix strip (the pattern matches a SUFFIX) */
static char *_sh_stripsuf(_sh_vbuf *d, const char *s, const char *pat, int greedy) {
  static _sh_vbuf sc = {0, 0};
  size_t n = strlen(s);
  _sh_vgrow(&sc, n + 1); strcpy(sc.p, s);
  size_t best = n;
  if (greedy) {
    for (size_t i = n; i > 0; i--) {
      if (fnmatch(pat, sc.p + (n - i), 0) == 0) { best = n - i; break; }
    }
  } else {
    for (size_t i = 1; i <= n; i++) {
      if (fnmatch(pat, sc.p + (n - i), 0) == 0) { best = n - i; break; }
    }
  }
  _sh_vgrow(d, best + 1);
  strncpy(d->p, sc.p, best); d->p[best] = 0;
  return d->p;
}

/* native pipeline: argv stages (exec'd) + builtin thunks (forked) */
typedef struct { char **argv; void (*fn)(void); } _sh_stage;
static char *_sh_pipeline(_sh_stage *st, int n, _sh_vbuf *vb) {
  int outfd[2]; pid_t pids[64]; int np = 0; int prev = -1;
  _sh_vgrow(vb, 1); vb->p[0] = 0;
  if (pipe(outfd) != 0) { _sh_rc = 127; return vb->p; }
  for (int i = 0; i < n; i++) {
    int pfd[2]; int last = (i == n - 1);
    if (!last && pipe(pfd) != 0) break;
    fflush(stdout); fflush(stderr);
    pid_t pid = fork();
    if (pid < 0) break;
    if (pid == 0) {
      if (last) { dup2(outfd[1], 1); }
      close(outfd[0]); close(outfd[1]);
      if (!last) { close(pfd[0]); dup2(pfd[1], 1); close(pfd[1]); }
      if (prev >= 0) { dup2(prev, 0); close(prev); }
      if (st[i].fn) { st[i].fn(); fflush(stdout); _exit(0); }
      execvp(st[i].argv[0], st[i].argv);
      _exit(127);
    }
    if (np < 64) pids[np++] = pid;
    if (prev >= 0) close(prev);
    if (!last) { close(pfd[1]); prev = pfd[0]; }
  }
  close(outfd[1]);
  if (prev >= 0) close(prev);
  size_t gn = 0;
  for (;;) {
    _sh_vgrow(vb, gn + 8192);
    ssize_t r2 = read(outfd[0], vb->p + gn, vb->cap - gn - 1);
    if (r2 < 0) { if (errno == EINTR) continue; break; }
    if (r2 == 0) break;
    gn += (size_t)r2;
  }
  vb->p[gn] = 0; close(outfd[0]);
  int status = 0;
  for (int i = 0; i < np; i++) waitpid(pids[i], &status, 0);
  _sh_rc = (np == 0) ? 127 : (WIFEXITED(status) ? WEXITSTATUS(status) : 1);
  while (gn > 0 && (vb->p[gn - 1] == '\n' || vb->p[gn - 1] == '\r')) vb->p[--gn] = 0;
  return vb->p;
}

static inline void _sh_pipeline_out(_sh_stage *st, int n) {
  pid_t pids[64]; int np = 0; int prev = -1;
  for (int i = 0; i < n; i++) {
    int pfd[2]; int last = (i == n - 1);
    if (!last && pipe(pfd) != 0) break;
    fflush(stdout); fflush(stderr);
    pid_t pid = fork();
    if (pid < 0) break;
    if (pid == 0) {
      if (!last) { close(pfd[0]); dup2(pfd[1], 1); close(pfd[1]); }
      if (prev >= 0) { dup2(prev, 0); close(prev); }
      if (st[i].fn) { st[i].fn(); fflush(stdout); _exit(_sh_rc); }
      execvp(st[i].argv[0], st[i].argv);
      _exit(127);
    }
    if (np < 64) pids[np++] = pid;
    if (prev >= 0) close(prev);
    if (!last) { close(pfd[1]); prev = pfd[0]; }
  }
  if (prev >= 0) close(prev);
  int status = 0;
  for (int i = 0; i < np; i++) waitpid(pids[i], &status, 0);
  _sh_rc = (np == 0) ? 127 : (WIFEXITED(status) ? WEXITSTATUS(status) : 1);
}

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */
static int _sh_is_f(const char *p) { struct stat st; return stat(p, &st) == 0 && S_ISREG(st.st_mode); }
static int _sh_is_d(const char *p) { struct stat st; return stat(p, &st) == 0 && S_ISDIR(st.st_mode); }
/* mkdir -p: create parents as needed (EEXIST is success) */
static int _sh_mkdir_p_at(char *p) {
  if (mkdir(p, 0777) == 0) return 0;
  if (errno == EEXIST) { struct stat st; return (stat(p, &st) == 0 && S_ISDIR(st.st_mode)) ? 0 : 1; }
  for (char *q = p + 1; *q; q++) {
    if (*q != '/') continue;
    *q = 0;
    if (mkdir(p, 0777) != 0 && errno != EEXIST) { *q = '/'; return 1; }
    *q = '/';
  }
  if (mkdir(p, 0777) == 0 || errno == EEXIST) return 0;
  return 1;
}
static int _sh_mkdir_p(const char *p0) {
  char *p = _sh_xstrdup(p0);
  int r = _sh_mkdir_p_at(p);
  free(p);
  return r;
}
static char *config_k[1024] = {0};
static char *config_v[1024] = {0};
static size_t config_n;
static char *first_half[1] = {0};
static size_t first_half_len;
static char *last_half[1] = {0};
static size_t last_half_len;
static char *middle[1] = {0};
static size_t middle_len;
static long long numbers[10] = {0};
static size_t numbers_len;
static unsigned char _sh_pres_numbers[10] = {0};

char DEBUG[2] = "";
char LOG_LEVEL[8] = "";
char* __ps_tmp0 = NULL;
char* __ps_tmp1 = NULL;
char* __ps_tmp2 = NULL;
char* __ps_tmp3 = NULL;
char* __ps_tmp4 = NULL;
char* __ps_tmp5 = NULL;
char* basename_ = NULL;
char* extension = NULL;
char* file = NULL;
const char filename[12] = "my_file.txt";
char* gid = NULL;
char* home = NULL;
uint8_t i = 0;
char* info = NULL;
char* input = NULL;
char* j = NULL;
char* message = NULL;
const char name[9] = "John Doe";
char* pass = NULL;
char* result = NULL;
char* sanitized = NULL;
char* shell = NULL;
char* uid = NULL;
char* uppercase = NULL;
char* user = NULL;

static char *_cap_0(void);
static char *_cap_2(void);
static char *_cap_3(void);
static char *_cap_5(void);
static char *_cap_8(void);
static char *_cap_10(void);
static char *_cap_13(void);
static char *_cap_12(void);
static char *_cap_19(void);
static int _sh_site_1(void);
static int _sh_site_4(void);
static int _sh_site_6(void);
static int _sh_site_7(void);
static int _sh_site_9(void);
static int _sh_site_11(void);
static int _sh_site_14(void);
static int _sh_site_15(void);
static int _sh_site_20(void);

static char *_cap_0(void) {
  static _sh_vbuf buf = {0, 0};
  static char *_c0_cmd = 0; static size_t _c0_cap = 0;
  static char *_c0_wb = 0; static size_t _c0_wcap = 0;
_sh_bres(&_c0_cmd, &_c0_cap);
_sh_bword(&_c0_cmd, &_c0_cap, "mktemp");
_sh_capture(&buf, _c0_cmd);
return buf.p;
}

static char *_cap_2(void) {
  static _sh_vbuf buf = {0, 0};
  static char *_c2_cmd = 0; static size_t _c2_cap = 0;
  static char *_c2_wb = 0; static size_t _c2_wcap = 0;
_sh_bres(&_c2_cmd, &_c2_cap);
_sh_badd(&_c2_cmd, &_c2_cap, "mktemp");
_sh_capture(&buf, _c2_cmd);
return buf.p;
}

static char *_cap_3(void) {
  static _sh_vbuf buf = {0, 0};
  static char *_c3_cmd = 0; static size_t _c3_cap = 0;
  static char *_c3_wb = 0; static size_t _c3_wcap = 0;
_sh_bres(&_c3_cmd, &_c3_cap);
_sh_bword(&_c3_cmd, &_c3_cap, "mktemp");
_sh_capture(&buf, _c3_cmd);
return buf.p;
}

static char *_cap_5(void) {
  static _sh_vbuf buf = {0, 0};
  static char *_c5_cmd = 0; static size_t _c5_cap = 0;
  static char *_c5_wb = 0; static size_t _c5_wcap = 0;
_sh_bres(&_c5_cmd, &_c5_cap);
_sh_badd(&_c5_cmd, &_c5_cap, "mktemp");
_sh_capture(&buf, _c5_cmd);
return buf.p;
}

static char *_cap_8(void) {
  static _sh_vbuf buf = {0, 0};
  static char *_c8_cmd = 0; static size_t _c8_cap = 0;
  static char *_c8_wb = 0; static size_t _c8_wcap = 0;
_sh_bres(&_c8_cmd, &_c8_cap);
_sh_bword(&_c8_cmd, &_c8_cap, "mktemp");
_sh_capture(&buf, _c8_cmd);
return buf.p;
}

static char *_cap_10(void) {
  static _sh_vbuf buf = {0, 0};
  static char *_c10_cmd = 0; static size_t _c10_cap = 0;
  static char *_c10_wb = 0; static size_t _c10_wcap = 0;
_sh_bres(&_c10_cmd, &_c10_cap);
_sh_bword(&_c10_cmd, &_c10_cap, "mktemp");
_sh_capture(&buf, _c10_cmd);
return buf.p;
}

static char *_cap_13(void) {
  static _sh_vbuf buf = {0, 0};
  static char *_c13_cmd = 0; static size_t _c13_cap = 0;
  static char *_c13_wb = 0; static size_t _c13_wcap = 0;
_sh_bres(&_c13_cmd, &_c13_cap);
_sh_badd(&_c13_cmd, &_c13_cap, "echo \"$(( $(wc -l < /etc/passwd) + $(wc -l < /etc/group) ))\"");
_sh_capture(&buf, _c13_cmd);
return buf.p;
}

static char *_cap_12(void) {
  static _sh_vbuf buf = {0, 0};
  static char *_c12_cmd = 0; static size_t _c12_cap = 0;
  static char *_c12_wb = 0; static size_t _c12_wcap = 0;
_sh_bres(&_c12_cmd, &_c12_cap);
_sh_bword(&_c12_cmd, &_c12_cap, "echo");
_sh_bword(&_c12_cmd, &_c12_cap, _cap_13());
_sh_capture(&buf, _c12_cmd);
return buf.p;
}

static char *_cap_19(void) {
  static _sh_vbuf buf = {0, 0};
  static char *_c19_cmd = 0; static size_t _c19_cap = 0;
  static char *_c19_wb = 0; static size_t _c19_wcap = 0;
_sh_bres(&_c19_cmd, &_c19_cap);
_sh_badd(&_c19_cmd, &_c19_cap, "printf '%s\\n'");
_sh_badd(&_c19_cmd, &_c19_cap, " *.txt");
_sh_badd(&_c19_cmd, &_c19_cap, " *.log");
_sh_badd(&_c19_cmd, &_c19_cap, " *.dat");
_sh_capture(&buf, _c19_cmd);
return buf.p;
}

static int _sh_site_1(void) {
_sh_reset();
_sh_addraw("{");
_sh_addraw("__ps_tmp0=");
_sh_word(_cap_2());
_sh_addraw(";");
_sh_addraw("{");
_sh_addraw("(");
_sh_word("grep");
_sh_word("-v");
_sh_word("^#");
_sh_word("/etc/passwd");
_sh_addraw("|");
_sh_word("cut");
_sh_word("-d");
_sh_word(":");
_sh_word("-f");
_sh_word("1");
_sh_addraw(")");
_sh_addraw(">");
_sh_word((__ps_tmp0 ? __ps_tmp0 : ""));
_sh_addraw(";");
_sh_addraw("}");
_sh_addraw(";");
_sh_word("sort");
_sh_word((__ps_tmp0 ? __ps_tmp0 : ""));
_sh_addraw(";");
_sh_addraw("{");
_sh_addraw("kill");
_sh_addraw("$(cat ${__ps_tmp0}.pid)");
_sh_addraw("2>/dev/null;");
_sh_addraw("rm");
_sh_addraw("-f");
_sh_addraw("${__ps_tmp0}");
_sh_addraw("${__ps_tmp0}.pid");
_sh_addraw(";");
_sh_addraw("}");
_sh_addraw("; }");
_sh_addraw(">");
_sh_word((__ps_tmp1 ? __ps_tmp1 : ""));
  return !_sh_system_rc();
}

static int _sh_site_4(void) {
_sh_reset();
_sh_addraw("{");
_sh_addraw("__ps_tmp2=");
_sh_word(_cap_5());
_sh_addraw(";");
_sh_addraw("{");
_sh_addraw("(");
_sh_word("grep");
_sh_word("-v");
_sh_word("^#");
_sh_word("/etc/group");
_sh_addraw("|");
_sh_word("cut");
_sh_word("-d");
_sh_word(":");
_sh_word("-f");
_sh_word("1");
_sh_addraw(")");
_sh_addraw(">");
_sh_word((__ps_tmp2 ? __ps_tmp2 : ""));
_sh_addraw(";");
_sh_addraw("}");
_sh_addraw(";");
_sh_word("sort");
_sh_word((__ps_tmp2 ? __ps_tmp2 : ""));
_sh_addraw(";");
_sh_addraw("{");
_sh_addraw("kill");
_sh_addraw("$(cat ${__ps_tmp2}.pid)");
_sh_addraw("2>/dev/null;");
_sh_addraw("rm");
_sh_addraw("-f");
_sh_addraw("${__ps_tmp2}");
_sh_addraw("${__ps_tmp2}.pid");
_sh_addraw(";");
_sh_addraw("}");
_sh_addraw("; }");
_sh_addraw(">");
_sh_word((__ps_tmp3 ? __ps_tmp3 : ""));
  return !_sh_system_rc();
}

static int _sh_site_6(void) {
_sh_reset();
_sh_word("diff");
_sh_word((__ps_tmp1 ? __ps_tmp1 : ""));
_sh_word((__ps_tmp3 ? __ps_tmp3 : ""));
  return !_sh_system_rc();
}

static int _sh_site_7(void) {
_sh_reset();
_sh_word("cat");
_sh_addraw(">");
_sh_word("config.txt");
_sh_addraw("<<'_SH2EOF_'\n");
_sh_add("User: $USER\nHost: ${HOSTNAME:-localhost}\nPath: $PWD\n");
{ size_t _hl = strlen("User: $USER\nHost: ${HOSTNAME:-localhost}\nPath: $PWD\n"); if (_hl == 0 || "User: $USER\nHost: ${HOSTNAME:-localhost}\nPath: $PWD\n"[_hl - 1] != '\n') _sh_add("\n"); }
_sh_add("_SH2EOF_");
  return !_sh_system_rc();
}

static int _sh_site_9(void) {
_sh_reset();
_sh_word("cut");
_sh_word("-d");
_sh_word(":");
_sh_word("-f");
_sh_word("1");
_sh_word("/etc/passwd");
_sh_add("\n");
_sh_addraw("|");
_sh_word("sort");
_sh_addraw(">");
_sh_word((__ps_tmp4 ? __ps_tmp4 : ""));
  return !_sh_system_rc();
}

static int _sh_site_11(void) {
_sh_reset();
_sh_word("cut");
_sh_word("-d");
_sh_word(":");
_sh_word("-f");
_sh_word("3");
_sh_word("/etc/passwd");
_sh_addraw("|");
_sh_word("sort");
_sh_word("-n");
_sh_addraw(">");
_sh_word((__ps_tmp5 ? __ps_tmp5 : ""));
  return !_sh_system_rc();
}

static int _sh_site_14(void) {
_sh_reset();
_sh_word("grep");
_sh_word("-v");
_sh_word("^#");
_sh_word("/etc/passwd");
_sh_addraw("|");
_sh_word("cut");
_sh_word("-d");
_sh_word(":");
_sh_word("-f");
_sh_word("1,3");
_sh_addraw("|");
_sh_word("sort");
_sh_word("-t");
_sh_word(":");
_sh_word("-k");
_sh_word("2");
_sh_word("-n");
_sh_addraw("|");
_sh_word("head");
_sh_word("-5");
_sh_addraw(">");
_sh_word("users.txt");
_sh_addraw("2>");
_sh_word("errors.log");
  return !_sh_system_rc();
}

static int _sh_site_15(void) {
    FILE *_pf15 = fopen("/etc/passwd", "r");
    if (_pf15) {
        static _sh_vbuf _rl15 = {0, 0};
        while (_sh_fgets_line(_pf15, &_rl15)) {
            size_t _rn15 = strlen(_rl15.p); while (_rn15 && (_rl15.p[_rn15-1]=='\n' || _rl15.p[_rn15-1]=='\r')) _rl15.p[--_rn15] = 0;
            { char *__fv[7]; char *_rp = _rl15.p; size_t _rk; for (_rk = 0; _rk < 7; _rk++) { char *_d = (_rk + 1 < 7) ? strchr(_rp, 58) : NULL; if (_d) *_d = 0; __fv[_rk] = _rp; if (!_d) break; else _rp = _d + 1; }
            _sh_mstr_set_from(&user, __fv[0]);
            _sh_mstr_set_from(&pass, __fv[1]);
            _sh_mstr_set_from(&uid, __fv[2]);
            _sh_mstr_set_from(&gid, __fv[3]);
            _sh_mstr_set_from(&info, __fv[4]);
            _sh_mstr_set_from(&home, __fv[5]);
            _sh_mstr_set_from(&shell, __fv[6]);
            }
            if (((atoll((uid ? uid : "")) > 1000) && (fnmatch("/bin/false", (shell ? shell : ""), FNM_CASEFOLD) != 0))) {
                if (_sh_regex_match((home ? home : ""), "^/home/")) {
                    static _sh_vbuf _v24 = {0, 0};
                    { const char *_a25_0 = ((user ? user : "")); const char *_a25_1 = ((uid ? uid : "")); const char *_a25_2 = ((home ? home : "")); size_t _l25_0 = strlen(_a25_0); size_t _l25_1 = strlen(_a25_1); size_t _l25_2 = strlen(_a25_2); _sh_vgrow(&_v24, 6 + _l25_0 + 7 + _l25_1 + 4 + _l25_2 + 1); char *_w25 = _v24.p; memcpy(_w25, "User: ", 6); _w25 += 6; memcpy(_w25, _a25_0, _l25_0); _w25 += _l25_0; memcpy(_w25, " (UID: ", 7); _w25 += 7; memcpy(_w25, _a25_1, _l25_1); _w25 += _l25_1; memcpy(_w25, ") - ", 4); _w25 += 4; memcpy(_w25, _a25_2, _l25_2); _w25 += _l25_2; _w25[0] = 0; }
                    printf("%s\n", (char*)(_v24.p));
                    _sh_rc = 0;
                } else { _sh_rc = 0; }
            } else { _sh_rc = 0; }
        }
    fclose(_pf15);
    }
return 0; /* loop rc: last body status tracked via _sh_rc */
}

static int _sh_site_20(void) {
_sh_reset();
_sh_word("pwd");
  { int _rc = _sh_system_rc(); if (_rc != 0) { _sh_wrap_cmd("echo \"Error on line $LINENO\"; exit 1"); system(_sh_wrap); } return !_rc; }
}

int main(int argc, char **argv) {
  _sh_argc = argc; _sh_argv = argv;
  _bigint_init();
  if (!getenv("HOSTNAME")) { extern int gethostname(char *, size_t); static char _hn[256]; if (gethostname(_hn, sizeof _hn) == 0) setenv("HOSTNAME", _hn, 0); }
  freopen("/dev/null", "w", stderr);
  setvbuf(stdout, 0, _IONBF, 0);
    assert(strlen(DEBUG) <= 1);
    assert(strlen(LOG_LEVEL) <= 7);

    {
        _sh_sset(&__ps_tmp1, (char*)(_cap_0()));
        _sh_site_1();
        _sh_sset(&__ps_tmp3, (char*)(_cap_3()));
        _sh_site_4();
        _sh_site_6();
        {
        FILE *_pc0f = NULL; char _pc0p[4096];
        if (__ps_tmp1) {
        snprintf(_pc0p, sizeof _pc0p, "%s.pid", __ps_tmp1);
        _pc0f = fopen(_pc0p, "r");
        if (_pc0f) {
        int _pc0n = 0;
        if (fscanf(_pc0f, "%d", &_pc0n) == 1 && _pc0n > 1 && _pc0n != getpid()) kill(_pc0n, 15);
        fclose(_pc0f);
        }
        unlink(__ps_tmp1);
        unlink(_pc0p);
        }
        }
        {
        FILE *_pc1f = NULL; char _pc1p[4096];
        if (__ps_tmp3) {
        snprintf(_pc1p, sizeof _pc1p, "%s.pid", __ps_tmp3);
        _pc1f = fopen(_pc1p, "r");
        if (_pc1f) {
        int _pc1n = 0;
        if (fscanf(_pc1f, "%d", &_pc1n) == 1 && _pc1n > 1 && _pc1n != getpid()) kill(_pc1n, 15);
        fclose(_pc1f);
        }
        unlink(__ps_tmp3);
        unlink(_pc1p);
        }
        }
    }
    printf("%s\n", (char*)("Files:  file_a_1..10.txt file_a_1..10.log file_a_1..10.dat file_a_20.txt file_a_20.log file_a_20.dat file_a_30..40.txt file_a_30..40.log file_a_30..40.dat file_b_1..10.txt file_b_1..10.log file_b_1..10.dat file_b_20.txt file_b_20.log file_b_20.dat file_b_30..40.txt file_b_30..40.log file_b_30..40.dat file_c_1..10.txt file_c_1..10.log file_c_1..10.dat file_c_20.txt file_c_20.log file_c_20.dat file_c_30..40.txt file_c_30..40.log file_c_30..40.dat file_d_1..10.txt file_d_1..10.log file_d_1..10.dat file_d_20.txt file_d_20.log file_d_20.dat file_d_30..40.txt file_d_30..40.log file_d_30..40.dat file_e_1..10.txt file_e_1..10.log file_e_1..10.dat file_e_20.txt file_e_20.log file_e_20.dat file_e_30..40.txt file_e_30..40.log file_e_30..40.dat file_f_1..10.txt file_f_1..10.log file_f_1..10.dat file_f_20.txt file_f_20.log file_f_20.dat file_f_30..40.txt file_f_30..40.log file_f_30..40.dat file_g_1..10.txt file_g_1..10.log file_g_1..10.dat file_g_20.txt file_g_20.log file_g_20.dat file_g_30..40.txt file_g_30..40.log file_g_30..40.dat file_h_1..10.txt file_h_1..10.log file_h_1..10.dat file_h_20.txt file_h_20.log file_h_20.dat file_h_30..40.txt file_h_30..40.log file_h_30..40.dat file_i_1..10.txt file_i_1..10.log file_i_1..10.dat file_i_20.txt file_i_20.log file_i_20.dat file_i_30..40.txt file_i_30..40.log file_i_30..40.dat file_j_1..10.txt file_j_1..10.log file_j_1..10.dat file_j_20.txt file_j_20.log file_j_20.dat file_j_30..40.txt file_j_30..40.log file_j_30..40.dat file_k_1..10.txt file_k_1..10.log file_k_1..10.dat file_k_20.txt file_k_20.log file_k_20.dat file_k_30..40.txt file_k_30..40.log file_k_30..40.dat file_l_1..10.txt file_l_1..10.log file_l_1..10.dat file_l_20.txt file_l_20.log file_l_20.dat file_l_30..40.txt file_l_30..40.log file_l_30..40.dat file_m_1..10.txt file_m_1..10.log file_m_1..10.dat file_m_20.txt file_m_20.log file_m_20.dat file_m_30..40.txt file_m_30..40.log file_m_30..40.dat file_n_1..10.txt file_n_1..10.log file_n_1..10.dat file_n_20.txt file_n_20.log file_n_20.dat file_n_30..40.txt file_n_30..40.log file_n_30..40.dat file_o_1..10.txt file_o_1..10.log file_o_1..10.dat file_o_20.txt file_o_20.log file_o_20.dat file_o_30..40.txt file_o_30..40.log file_o_30..40.dat file_p_1..10.txt file_p_1..10.log file_p_1..10.dat file_p_20.txt file_p_20.log file_p_20.dat file_p_30..40.txt file_p_30..40.log file_p_30..40.dat file_q_1..10.txt file_q_1..10.log file_q_1..10.dat file_q_20.txt file_q_20.log file_q_20.dat file_q_30..40.txt file_q_30..40.log file_q_30..40.dat file_r_1..10.txt file_r_1..10.log file_r_1..10.dat file_r_20.txt file_r_20.log file_r_20.dat file_r_30..40.txt file_r_30..40.log file_r_30..40.dat file_s_1..10.txt file_s_1..10.log file_s_1..10.dat file_s_20.txt file_s_20.log file_s_20.dat file_s_30..40.txt file_s_30..40.log file_s_30..40.dat file_t_1..10.txt file_t_1..10.log file_t_1..10.dat file_t_20.txt file_t_20.log file_t_20.dat file_t_30..40.txt file_t_30..40.log file_t_30..40.dat file_u_1..10.txt file_u_1..10.log file_u_1..10.dat file_u_20.txt file_u_20.log file_u_20.dat file_u_30..40.txt file_u_30..40.log file_u_30..40.dat file_v_1..10.txt file_v_1..10.log file_v_1..10.dat file_v_20.txt file_v_20.log file_v_20.dat file_v_30..40.txt file_v_30..40.log file_v_30..40.dat file_w_1..10.txt file_w_1..10.log file_w_1..10.dat file_w_20.txt file_w_20.log file_w_20.dat file_w_30..40.txt file_w_30..40.log file_w_30..40.dat file_x_1..10.txt file_x_1..10.log file_x_1..10.dat file_x_20.txt file_x_20.log file_x_20.dat file_x_30..40.txt file_x_30..40.log file_x_30..40.dat file_y_1..10.txt file_y_1..10.log file_y_1..10.dat file_y_20.txt file_y_20.log file_y_20.dat file_y_30..40.txt file_y_30..40.log file_y_30..40.dat file_z_1..10.txt file_z_1..10.log file_z_1..10.dat file_z_20.txt file_z_20.log file_z_20.dat file_z_30..40.txt file_z_30..40.log file_z_30..40.dat"));
    _sh_rc = 0;
    static _sh_vbuf _v2 = {0, 0};
    { const char *_a3_0 = ("John_Doe"); size_t _l3_0 = strlen(_a3_0); _sh_vgrow(&_v2, 6 + _l3_0 + 1); char *_w3 = _v2.p; memcpy(_w3, "Hello ", 6); _w3 += 6; memcpy(_w3, _a3_0, _l3_0); _w3 += _l3_0; _w3[0] = 0; }
    printf("%s\n", (char*)(_v2.p));
    _sh_rc = 0;
    static _sh_vbuf _v4 = {0, 0};
    { const char *_a5_0 = ("8"); size_t _l5_0 = strlen(_a5_0); _sh_vgrow(&_v4, 8 + _l5_0 + 1); char *_w5 = _v4.p; memcpy(_w5, "Length: ", 8); _w5 += 8; memcpy(_w5, _a5_0, _l5_0); _w5 += _l5_0; _w5[0] = 0; }
    printf("%s\n", (char*)(_v4.p));
    _sh_rc = 0;
    printf("First: %s\n", "John"), _sh_rc = 0;
    static _sh_vbuf _v6 = {0, 0};
    _sh_substr(&_v6, name, -3, (long long)1LL<<60);
    printf("Last: %s\n", ((char*)(_v6.p) ? (char*)(_v6.p) : "")), _sh_rc = 0;
    _sh_rc = 0;
    _sh_rc = 0;
    const char *_cd7 = (((1 < _sh_argc && _sh_argv[1]) ? _sh_argv[1] : ""));
    if (fnmatch("[a-z]*", _cd7, 0 | FNM_CASEFOLD) == 0) {
        fputs("Lowercase start\n", stdout), _sh_rc = 0;
    }
    else if (fnmatch("[A-Z]*", _cd7, 0 | FNM_CASEFOLD) == 0) {
        fputs("Uppercase start\n", stdout), _sh_rc = 0;
    }
    else if (fnmatch("[0-9]*", _cd7, 0 | FNM_CASEFOLD) == 0) {
        fputs("Number start\n", stdout), _sh_rc = 0;
    }
    else if (fnmatch("?", _cd7, 0 | FNM_CASEFOLD) == 0) {
        fputs("Single character\n", stdout), _sh_rc = 0;
    }
    else if (fnmatch("*", _cd7, 0 | FNM_CASEFOLD) == 0) {
        fputs("Something else\n", stdout), _sh_rc = 0;
    }
    else { _sh_rc = 0; }
    i = 2;
    long long _sq8 = i;
    i += 1;
    long long _sq9 = (i += 1);
    static char _s10[32];
    _sh_ll2s(_s10, (long long)((_sq8 + _sq9)));
    _sh_sset(&j, (char*)(_s10));
    static char _s11[32];
    _sh_ll2s(_s11, (long long)(i));
    static _sh_vbuf _v12 = {0, 0};
    { const char *_a13_0 = (_s11); const char *_a13_1 = ((j ? j : "")); size_t _l13_0 = strlen(_a13_0); size_t _l13_1 = strlen(_a13_1); _sh_vgrow(&_v12, 2 + _l13_0 + 4 + _l13_1 + 1); char *_w13 = _v12.p; memcpy(_w13, "i=", 2); _w13 += 2; memcpy(_w13, _a13_0, _l13_0); _w13 += _l13_0; memcpy(_w13, ", j=", 4); _w13 += 4; memcpy(_w13, _a13_1, _l13_1); _w13 += _l13_1; _w13[0] = 0; }
    printf("%s\n", (char*)(_v12.p));
    _sh_rc = 0;
    _sh_rc = 0;
    _sh_assoc_set(config_k, config_v, &config_n, 1024, "user", "admin");
    _sh_assoc_set(config_k, config_v, &config_n, 1024, "host", "localhost");
    _sh_assoc_set(config_k, config_v, &config_n, 1024, "port", "8080");
    _sh_site_7();
    {
        _sh_sset(&__ps_tmp4, (char*)(_cap_8()));
        _sh_site_9();
        _sh_sset(&__ps_tmp5, (char*)(_cap_10()));
        _sh_site_11();
        char *_a14_0[] = {"paste", (__ps_tmp4 ? __ps_tmp4 : ""), (__ps_tmp5 ? __ps_tmp5 : ""), 0};
        char *_a14_1[] = {"head", "-10", 0};
        _sh_stage _sp14[] = {{_a14_0, 0}, {_a14_1, 0}};
        _sh_pipeline_out(_sp14, 2);
        {
        FILE *_pc15f = NULL; char _pc15p[4096];
        if (__ps_tmp4) {
        snprintf(_pc15p, sizeof _pc15p, "%s.pid", __ps_tmp4);
        _pc15f = fopen(_pc15p, "r");
        if (_pc15f) {
        int _pc15n = 0;
        if (fscanf(_pc15f, "%d", &_pc15n) == 1 && _pc15n > 1 && _pc15n != getpid()) kill(_pc15n, 15);
        fclose(_pc15f);
        }
        unlink(__ps_tmp4);
        unlink(_pc15p);
        }
        }
        {
        FILE *_pc16f = NULL; char _pc16p[4096];
        if (__ps_tmp5) {
        snprintf(_pc16p, sizeof _pc16p, "%s.pid", __ps_tmp5);
        _pc16f = fopen(_pc16p, "r");
        if (_pc16f) {
        int _pc16n = 0;
        if (fscanf(_pc16f, "%d", &_pc16n) == 1 && _pc16n > 1 && _pc16n != getpid()) kill(_pc16n, 15);
        fclose(_pc16f);
        }
        unlink(__ps_tmp5);
        unlink(_pc16p);
        }
        }
    }
    static _sh_vbuf _v17 = {0, 0};
    _sh_vgrow(&_v17, 256);
    for (;;) { int _m18 = snprintf(_v17.p, _v17.cap, "%s%s", ((2 < _sh_argc && _sh_argv[2]) ? _sh_argv[2] : ""), "\"==\"test"); if (_m18 < 0) break; if ((size_t)_m18 < _v17.cap) break; _sh_vgrow(&_v17, (size_t)_m18 + 1); }
    static _sh_vbuf _v19 = {0, 0};
    _sh_vgrow(&_v19, 256);
    for (;;) { int _m20 = snprintf(_v19.p, _v19.cap, "%s%s", ((2 < _sh_argc && _sh_argv[2]) ? _sh_argv[2] : ""), "\"==\"debug"); if (_m20 < 0) break; if ((size_t)_m20 < _v19.cap) break; _sh_vgrow(&_v19, (size_t)_m20 + 1); }
    if ((_sh_regex_match(((1 < _sh_argc && _sh_argv[1]) ? _sh_argv[1] : ""), "^[0-9]+$") && (((_v17.p) && (_v17.p)[0]) || ((_v19.p) && (_v19.p)[0])))) {
        printf("%s\n", (char*)("Valid input"));
        _sh_rc = 0;
    } else { _sh_rc = 0; }
    ({ int _mr = 0; { _mr |= _sh_mkdir_p("project/src/main/java"); } { _mr |= _sh_mkdir_p("project/src/main/resources"); } { _mr |= _sh_mkdir_p("project/src/test/java"); } { _mr |= _sh_mkdir_p("project/src/test/resources"); } { _mr |= _sh_mkdir_p("project/docs/api"); } { _mr |= _sh_mkdir_p("project/docs/user"); } { _mr |= _sh_mkdir_p("project/build/classes"); } { _mr |= _sh_mkdir_p("project/build/lib"); } _sh_rc = (_mr == 0 ? 0 : 1); _mr == 0; });
    static _sh_vbuf _v21 = {0, 0};
    { const char *_t=(filename); size_t _n=_t?strlen(_t):0; const char *_d="."; size_t _dl=strlen(_d); size_t _i=_n; int _hit=0; if(!_dl){ _hit=1; _i=(0)?0:_n; } else if(_t){ const char *_p=strstr(_t,_d); if(_p){ _hit=1; _i=(size_t)(_p-_t); if(!(0)){ const char *_q=_p+_dl; for(;;){ const char *_r=strstr(_q,_d); if(!_r)break; _q=_r+_dl; _i=(size_t)(_r-_t); } } } } size_t _off=(!_hit)?0:((1)?0:_i+_dl); size_t _gl =(!_hit)?_n:((1)?_i:(_n>_i+_dl?_n-(_i+_dl):0)); _sh_vgrow(&_v21,_gl+1); if(_t&&_gl)memcpy(_v21.p,_t+_off,_gl); _v21.p[_gl]=0; }
    _sh_sset(&basename_, (char*)(_v21.p));
    static _sh_vbuf _v22 = {0, 0};
    { const char *_t=(filename); size_t _n=_t?strlen(_t):0; const char *_d="."; size_t _dl=strlen(_d); size_t _i=_n; int _hit=0; if(!_dl){ _hit=1; _i=(0)?0:_n; } else if(_t){ const char *_p=strstr(_t,_d); if(_p){ _hit=1; _i=(size_t)(_p-_t); if(!(0)){ const char *_q=_p+_dl; for(;;){ const char *_r=strstr(_q,_d); if(!_r)break; _q=_r+_dl; _i=(size_t)(_r-_t); } } } } size_t _off=(!_hit)?0:((0)?0:_i+_dl); size_t _gl =(!_hit)?_n:((0)?_i:(_n>_i+_dl?_n-(_i+_dl):0)); _sh_vgrow(&_v22,_gl+1); if(_t&&_gl)memcpy(_v22.p,_t+_off,_gl); _v22.p[_gl]=0; }
    _sh_sset(&extension, (char*)(_v22.p));
    _sh_sset(&uppercase, (char*)("MY_FILE.TXT"));
    _sh_sset(&result, (char*)(_cap_12()));
    _sh_site_14();
    _sh_site_15();
    size_t _ai26 = 0;
    numbers_len = 0;
    memset(_sh_pres_numbers, 0, sizeof _sh_pres_numbers);
    numbers[_ai26] = (long long)(1LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    numbers[_ai26] = (long long)(2LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    numbers[_ai26] = (long long)(3LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    numbers[_ai26] = (long long)(4LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    numbers[_ai26] = (long long)(5LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    numbers[_ai26] = (long long)(6LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    numbers[_ai26] = (long long)(7LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    numbers[_ai26] = (long long)(8LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    numbers[_ai26] = (long long)(9LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    numbers[_ai26] = (long long)(10LL);
    _sh_pres_numbers[_ai26] = 1;
    numbers_len = ++_ai26;
    size_t _ai27 = 0;
    for (size_t _fi = 0; _fi < middle_len; _fi++) { free(middle[_fi]); middle[_fi] = 0; }
    middle_len = 0;
    static _sh_vbuf _v28 = {0, 0};
    { size_t _jdn16 = 0; int _jdf16 = 1; char _jdb16[24];
      for (size_t _jdi16 = 0; _jdi16 < numbers_len; _jdi16++) {
        if (!_sh_pres_numbers[_jdi16]) continue;
        if (_jdf16) { _jdf16 = 0; } else { char _jsep = ' '; if (_jsep) { _sh_vgrow(&_v28, _jdn16 + 2); _v28.p[_jdn16++] = _jsep; } }
        _sh_ll2s(_jdb16, (long long)(numbers[_jdi16]));
        for (const char *_jds16 = _jdb16; *_jds16; _jds16++) { _sh_vgrow(&_v28, _jdn16 + 2); _v28.p[_jdn16++] = *_jds16; }
      }
      _sh_vgrow(&_v28, _jdn16 + 1); _v28.p[_jdn16] = 0; }
    static _sh_vbuf _v29 = {0, 0};
    _sh_arr_slice(&_v29, _v28.p, 3, 4, ' ');
    middle[_ai27] = _sh_xstrdup((char*)(_v29.p));
    middle_len = ++_ai27;
    size_t _ai30 = 0;
    for (size_t _fi = 0; _fi < first_half_len; _fi++) { free(first_half[_fi]); first_half[_fi] = 0; }
    first_half_len = 0;
    static _sh_vbuf _v31 = {0, 0};
    { size_t _jdn17 = 0; int _jdf17 = 1; char _jdb17[24];
      for (size_t _jdi17 = 0; _jdi17 < numbers_len; _jdi17++) {
        if (!_sh_pres_numbers[_jdi17]) continue;
        if (_jdf17) { _jdf17 = 0; } else { char _jsep = ' '; if (_jsep) { _sh_vgrow(&_v31, _jdn17 + 2); _v31.p[_jdn17++] = _jsep; } }
        _sh_ll2s(_jdb17, (long long)(numbers[_jdi17]));
        for (const char *_jds17 = _jdb17; *_jds17; _jds17++) { _sh_vgrow(&_v31, _jdn17 + 2); _v31.p[_jdn17++] = *_jds17; }
      }
      _sh_vgrow(&_v31, _jdn17 + 1); _v31.p[_jdn17] = 0; }
    static _sh_vbuf _v32 = {0, 0};
    _sh_arr_slice(&_v32, _v31.p, 0, 5, ' ');
    first_half[_ai30] = _sh_xstrdup((char*)(_v32.p));
    first_half_len = ++_ai30;
    size_t _ai33 = 0;
    for (size_t _fi = 0; _fi < last_half_len; _fi++) { free(last_half[_fi]); last_half[_fi] = 0; }
    last_half_len = 0;
    static _sh_vbuf _v34 = {0, 0};
    { size_t _jdn18 = 0; int _jdf18 = 1; char _jdb18[24];
      for (size_t _jdi18 = 0; _jdi18 < numbers_len; _jdi18++) {
        if (!_sh_pres_numbers[_jdi18]) continue;
        if (_jdf18) { _jdf18 = 0; } else { char _jsep = ' '; if (_jsep) { _sh_vgrow(&_v34, _jdn18 + 2); _v34.p[_jdn18++] = _jsep; } }
        _sh_ll2s(_jdb18, (long long)(numbers[_jdi18]));
        for (const char *_jds18 = _jdb18; *_jds18; _jds18++) { _sh_vgrow(&_v34, _jdn18 + 2); _v34.p[_jdn18++] = *_jds18; }
      }
      _sh_vgrow(&_v34, _jdn18 + 1); _v34.p[_jdn18] = 0; }
    static _sh_vbuf _v35 = {0, 0};
    _sh_arr_slice(&_v35, _v34.p, 5, (long long)1LL<<60, ' ');
    last_half[_ai33] = _sh_xstrdup((char*)(_v35.p));
    last_half_len = ++_ai33;
    char *_wn_36 = _cap_19(); static char **_ws_37 = 0; static size_t _ws_37_cap = 0; size_t _wc__wn_36 = _sh_split(_wn_36, &_ws_37, &_ws_37_cap);
    for (size_t _wi__wn_36 = 0; _wi__wn_36 < _wc__wn_36; _wi__wn_36++) {
        file = _ws_37[_wi__wn_36];
        const char *_cd38 = ((file ? file : ""));
        if (fnmatch("*.txt", _cd38, 0 | FNM_CASEFOLD) == 0) {
            printf("Text file: %s\n", ((char*)(file) ? (char*)(file) : "")), _sh_rc = 0;
        }
        else if (fnmatch("*.log", _cd38, 0 | FNM_CASEFOLD) == 0) {
            printf("Text file: %s\n", ((char*)(file) ? (char*)(file) : "")), _sh_rc = 0;
        }
        else if (fnmatch("*.dat", _cd38, 0 | FNM_CASEFOLD) == 0) {
            printf("Data file: %s\n", ((char*)(file) ? (char*)(file) : "")), _sh_rc = 0;
        }
        else if (fnmatch("*", _cd38, 0 | FNM_CASEFOLD) == 0) {
            printf("Other file: %s\n", ((char*)(file) ? (char*)(file) : "")), _sh_rc = 0;
        }
        else { _sh_rc = 0; }
    }
    {
        char _sv_DEBUG[2];
        strcpy(_sv_DEBUG, DEBUG);
        char _sv_LOG_LEVEL[8];
        strcpy(_sv_LOG_LEVEL, LOG_LEVEL);

        strncpy(DEBUG, (char*)("1"), 2);
        _sh_rc = 0;
        strncpy(LOG_LEVEL, (char*)("verbose"), 8);
        _sh_rc = 0;
        {
            char _sv_DEBUG[2];
            strcpy(_sv_DEBUG, DEBUG);

            DEBUG[0] = '\0';
            _sh_rc = 0;
            printf("Inner: LOG_LEVEL=%s, DEBUG=%s\n", ((char*)(LOG_LEVEL) ? (char*)(LOG_LEVEL) : ""), ((char*)((((DEBUG) && (DEBUG)[0]) ? (DEBUG) : ("unset"))) ? (char*)((((DEBUG) && (DEBUG)[0]) ? (DEBUG) : ("unset"))) : "")), _sh_rc = 0;

            strcpy(DEBUG, _sv_DEBUG);
        }
        printf("Outer: LOG_LEVEL=%s, DEBUG=%s\n", ((char*)(LOG_LEVEL) ? (char*)(LOG_LEVEL) : ""), ((char*)(DEBUG) ? (char*)(DEBUG) : "")), _sh_rc = 0;

        strcpy(DEBUG, _sv_DEBUG);
        strcpy(LOG_LEVEL, _sv_LOG_LEVEL);
    }
    const char *_d39 = (_sh_env("USER"));
    const char *_d40 = (_sh_env("HOSTNAME"));
    static _sh_vbuf _v41 = {0, 0};
    { const char *_a42_0 = ((((_d39) && (_d39)[0]) ? (_d39) : ("guest"))); const char *_a42_1 = ((((_d40) && (_d40)[0]) ? (_d40) : ("localhost"))); size_t _l42_0 = strlen(_a42_0); size_t _l42_1 = strlen(_a42_1); _sh_vgrow(&_v41, 6 + _l42_0 + 6 + _l42_1 + 1); char *_w42 = _v41.p; memcpy(_w42, "Hello ", 6); _w42 += 6; memcpy(_w42, _a42_0, _l42_0); _w42 += _l42_0; memcpy(_w42, " from ", 6); _w42 += 6; memcpy(_w42, _a42_1, _l42_1); _w42 += _l42_1; _w42[0] = 0; }
    _sh_sset(&message, (char*)(_v41.p));
    printf("%s\n", (char*)((message ? message : "")));
    _sh_rc = 0;
    _sh_rc = 0;
    _sh_rc = 0;
    _sh_sset(&input, (char*)((((((1 < _sh_argc && _sh_argv[1]) ? _sh_argv[1] : "")) && (((1 < _sh_argc && _sh_argv[1]) ? _sh_argv[1] : ""))[0]) ? (((1 < _sh_argc && _sh_argv[1]) ? _sh_argv[1] : "")) : ("default_value"))));
    static _sh_vbuf _v43 = {0, 0};
    _sh_replace(&_v43, (input ? input : ""), "[^a-zA-Z0-9]", "_");
    _sh_sset(&sanitized, (char*)(_v43.p));
    static _sh_vbuf _v44 = {0, 0};
    { const char *_u = (sanitized ? sanitized : ""); _sh_vgrow(&_v44, strlen(_u) + 1); char *_d = _v44.p; size_t _i; for (_i = 0; _u[_i]; _i++) _d[_i] = (char)toupper((unsigned char)_u[_i]); _d[_i] = 0; }
    _sh_sset(&uppercase, (char*)(_v44.p));
    static _sh_vbuf _v45 = {0, 0};
    { const char *_a46_0 = ((input ? input : "")); const char *_a46_1 = ((sanitized ? sanitized : "")); const char *_a46_2 = ((uppercase ? uppercase : "")); size_t _l46_0 = strlen(_a46_0); size_t _l46_1 = strlen(_a46_1); size_t _l46_2 = strlen(_a46_2); _sh_vgrow(&_v45, 8 + _l46_0 + 17 + _l46_1 + 17 + _l46_2 + 1 + 1); char *_w46 = _v45.p; memcpy(_w46, "Input: '", 8); _w46 += 8; memcpy(_w46, _a46_0, _l46_0); _w46 += _l46_0; memcpy(_w46, "' -> Sanitized: '", 17); _w46 += 17; memcpy(_w46, _a46_1, _l46_1); _w46 += _l46_1; memcpy(_w46, "' -> Uppercase: '", 17); _w46 += 17; memcpy(_w46, _a46_2, _l46_2); _w46 += _l46_2; memcpy(_w46, "'", 1); _w46 += 1; _w46[0] = 0; }
    printf("%s\n", (char*)(_v45.p));
    _sh_rc = 0;
    ((_sh_is_f(((1 < _sh_argc && _sh_argv[1]) ? _sh_argv[1] : "")) && (fputs("File exists\n", stdout), _sh_rc = 0, 1)) || (fputs("File not found\n", stdout), _sh_rc = 0, 1));
    (((_sh_is_d(((2 < _sh_argc && _sh_argv[2]) ? _sh_argv[2] : "")) && ({ int _r = chdir(((2 < _sh_argc && _sh_argv[2]) ? _sh_argv[2] : "")); _sh_rc = (_r == 0 ? 0 : 1); if (_r == 0) setenv("PWD", getcwd(0, 0), 1); _r == 0; })) && _sh_site_20()) || (fputs("Directory not accessible\n", stdout), _sh_rc = 0, 1));
    printf("%s\n", (char*)("Script completed successfully!"));
    _sh_rc = 0;
    _sh_wrap_cmd("echo \"Cleaning up...\"; rm -f /tmp/064_hard_temp_*"); system(_sh_wrap);
    free(__ps_tmp1);
    free(__ps_tmp3);
    free(basename_);
    free(extension);
    free(input);
    free(j);
    free(message);
    free(result);
    free(sanitized);
    free(uppercase);
      mpz_clear(_bigint_1000);
      mpz_clear(_bigint_2);
    return _sh_rc;
}