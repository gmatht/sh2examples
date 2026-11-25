#include <stdio.h>
#include <stdlib.h>
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


char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    char* UMASK = NULL;
    _sh_sset(&UMASK, (char*)(((((UMASK ? UMASK : "")) && ((UMASK ? UMASK : ""))[0]) ? ((UMASK ? UMASK : "")) : ("0640"))));
    printf("%s\n", (char*)((UMASK ? UMASK : "")));
    free(UMASK);
    return 0;
}