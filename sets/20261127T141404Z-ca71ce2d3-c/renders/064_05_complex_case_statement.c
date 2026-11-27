#define _GNU_SOURCE
#include <stdio.h>
#include <fnmatch.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(int argc, char **argv) {
    const char *_cd0 = (((1 < argc && argv[1]) ? argv[1] : ""));
    if (fnmatch("[a-z]*", _cd0, 0) == 0) {
        fputs("Lowercase start\n", stdout);
    }
    else if (fnmatch("[A-Z]*", _cd0, 0) == 0) {
        fputs("Uppercase start\n", stdout);
    }
    else if (fnmatch("[0-9]*", _cd0, 0) == 0) {
        fputs("Number start\n", stdout);
    }
    else if (fnmatch("?", _cd0, 0) == 0) {
        fputs("Single character\n", stdout);
    }
    else if (fnmatch("*", _cd0, 0) == 0) {
        fputs("Something else\n", stdout);
    }
    else { }
    return 0;
}