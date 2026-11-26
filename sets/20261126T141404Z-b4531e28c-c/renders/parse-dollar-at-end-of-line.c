#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("the price is $"));
    printf("%s\n", (char*)("$"));
    return 0;
}