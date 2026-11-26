#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("text with 'single quote' inside"));
    return 0;
}