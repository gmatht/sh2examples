#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gmp.h>
#define _SH_MPZ_INIT(name) static mpz_t name; static int name##_i = 0; if (!name##_i) { mpz_init(name); name##_i = 1; }
mpz_t _bigint_0;
static inline void _bigint_init(void) {
  mpz_init_set_str(_bigint_0, "0", 10);
}

#include <assert.h>
#include <gmp.h>
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


char IFS[256] = " \t\n"; /* bash's startup IFS */
char __sh2_tc0[4] = "";

int main(void) {
  _bigint_init();
    assert(strlen(__sh2_tc0) <= 3);

    {
        strncpy(__sh2_tc0, (char*)("foo"), 4);
        if ((atoll(__sh2_tc0) != 0)) {
            printf("%s\n", (char*)("bar"));
        }
    }
      mpz_clear(_bigint_0);
    return 0;
}