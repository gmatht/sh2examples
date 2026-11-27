#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("\n"));
    printf("%s\n", (char*)("hello\tworld"));
    return 0;
}