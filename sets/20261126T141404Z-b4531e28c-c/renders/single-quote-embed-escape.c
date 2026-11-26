#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("\n  s/'/'\\\\''/g\n"));
    return 0;
}