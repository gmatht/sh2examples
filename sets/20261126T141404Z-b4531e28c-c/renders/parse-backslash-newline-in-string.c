#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("/tmp/test_deterministic.txt"));
    return 0;
}