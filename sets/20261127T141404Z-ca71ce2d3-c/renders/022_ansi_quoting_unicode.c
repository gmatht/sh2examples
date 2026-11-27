#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("== Unicode and hex =="));
    printf("%s\n", (char*)("Hello"));
    printf("%s\n", (char*)("Hello"));
    return 0;
}