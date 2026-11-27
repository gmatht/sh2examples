#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("echo-line"));
    fputs((char*)("printf-line\n"), stdout);
    return 0;
}