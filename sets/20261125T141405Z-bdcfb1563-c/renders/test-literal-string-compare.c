#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("eq"));
    printf("%s\n", (char*)("ne"));
    printf("%s\n", (char*)("not-equal"));
    printf("%s\n", (char*)("var-eq"));
    printf("%s\n", (char*)("var-ne"));
    printf("%s\n", (char*)("num-eq"));
    printf("%s\n", (char*)("num-string-distinct"));
    return 0;
}