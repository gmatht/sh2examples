#define _GNU_SOURCE
#include <stdio.h>
#include <fnmatch.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(int argc, char **argv) {
    const char *_cd0 = (((1 < argc && argv[1]) ? argv[1] : ""));
    if (fnmatch("-[CDFISUWXx]", _cd0, 0) == 0) {
        fputs("option\n", stdout);
    }
    else { }
    return 0;
}