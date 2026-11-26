#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("== Practical examples =="));
    fputs((char*)("Name       Age        City\n"), stdout);
    fputs((char*)("John       25         NYC\n"), stdout);
    fputs((char*)("Jane       30         LA\n"), stdout);
    return 0;
}