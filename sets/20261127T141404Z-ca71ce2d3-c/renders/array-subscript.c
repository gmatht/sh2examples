#include <stdio.h>
#include <stdlib.h>
#include <string.h>
/* checked strdup (P1 OOM policy): fail-stop, never NULL-on-OOM */
static char *_sh_xstrdup(const char *s) {
  char *p = strdup(s);
  if (s && !p) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
  return p;
}


char IFS[256] = " \t\n"; /* bash's startup IFS */
static char *arr[3] = {0};
static size_t arr_len;


int main(void) {
    size_t _ai0 = 0;
    for (size_t _fi = 0; _fi < arr_len; _fi++) { free(arr[_fi]); arr[_fi] = 0; }
    arr_len = 0;
    arr[_ai0] = _sh_xstrdup((char*)("a"));
    arr_len = ++_ai0;
    arr[_ai0] = _sh_xstrdup((char*)("b"));
    arr_len = ++_ai0;
    arr[_ai0] = _sh_xstrdup((char*)("c"));
    arr_len = ++_ai0;
    printf("%s\n", (char*)(((1 < arr_len && arr[1]) ? arr[1] : "")));
    return 0;
}