#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("== Case modification in parameter expansion =="));
    printf("%s\n", (char*)("WORLD"));
    printf("%s\n", (char*)("world"));
    printf("%s\n", (char*)("World"));
    return 0;
}