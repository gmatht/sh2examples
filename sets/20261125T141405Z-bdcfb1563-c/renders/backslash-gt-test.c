#include <stdio.h>
#include <string.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    if ((strcmp("5", "10") > 0)) {
        printf("%s\n", (char*)("a > b"));
    }
    fputs((char*)("a=[5]\n"), stdout);
    fputs((char*)("b=[10]\n"), stdout);
    return 0;
}