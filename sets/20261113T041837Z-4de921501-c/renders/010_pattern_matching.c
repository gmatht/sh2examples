#define _GNU_SOURCE
#include <stdio.h>
#include <string.h>
#include <fnmatch.h>
#include <regex.h>

#include <assert.h>

char f1[8] = "";
char f2[13] = "";
char s[9] = "";
char word[4] = "";

int main(void) {
    int sh_status = 0;
    assert(strlen(f1) <= 7);
    assert(strlen(f2) <= 12);
    assert(strlen(s) <= 8);
    assert(strlen(word) <= 3);

    fputs("== [[ pattern and regex ]]\n", stdout);
    strncpy(s, (char*)("file.txt"), 9);
    { if ((fnmatch("*.txt", s, 0) == 0)) {
        fputs("pattern-match\n", stdout);
      }
    }
    { if ((({ regex_t _rx2; int _rc2 = regcomp(&_rx2, "^file\\.[a-z]+$", REG_EXTENDED | REG_NOSUB); int _m2 = 0; if (_rc2 == 0) { _m2 = regexec(&_rx2, s, 0, 0, 0) == 0; regfree(&_rx2); } _m2; }))) {
        fputs("regex-match\n", stdout);
      }
    }
    fputs("== extglob ==\n", stdout);
    strncpy(f1, (char*)("file.js"), 8);
    strncpy(f2, (char*)("thing.min.js"), 13);
    { if ((fnmatch("!(*.min).js", f1, 0 | FNM_EXTMATCH) == 0)) {
        fputs("f1-ok\n", stdout);
      }
    }
    { if ((fnmatch("!(*.min).js", f2, 0 | FNM_EXTMATCH) == 0)) { sh_status = 0; } else { sh_status = 1;
        fputs("f2-filtered\n", stdout);
      }
    }
    fputs("== nocasematch ==\n", stdout);
    strncpy(word, (char*)("Foo"), 4);
    { if (((fnmatch("foo", word, FNM_CASEFOLD) == 0))) { sh_status = 0;
        fputs("ci-match\n", stdout), sh_status = 0;
      } else { sh_status = 1; }
    }
    return sh_status;
}