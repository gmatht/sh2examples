#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("== ANSI-C quoting =="));
    printf("%s\n", (char*)("line1\nline2\tTabbed"));
    return 0;
}