#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    fputs((char*)("OPTIONS_SPEC=[--help --version ]\n"), stdout);
    return 0;
}