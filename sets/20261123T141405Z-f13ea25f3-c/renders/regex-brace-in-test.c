#define _GNU_SOURCE
#include <stdio.h>
#include <regex.h>
/* [[ s =~ re ]] — POSIX ERE match */
static int _sh_regex_match(const char *s, const char *re) {
  regex_t rx;
  if (regcomp(&rx, re, REG_EXTENDED | REG_NOSUB) != 0) return 0;
  int rc = regexec(&rx, s, 0, 0, 0);
  regfree(&rx);
  return rc == 0;
}


char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    if (_sh_regex_match("abc", "^[a-z]{1,3}$")) {
        printf("%s\n", (char*)("match"));
    }
    return 0;
}