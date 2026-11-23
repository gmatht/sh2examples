#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("== Escape sequences =="));
    printf("%s\n", (char*)("bell\007"));
    printf("%s\n", (char*)("backspace\010"));
    printf("%s\n", (char*)("formfeed\014"));
    printf("%s\n", (char*)("newline\n"));
    printf("%s\n", (char*)("carriage\rreturn"));
    printf("%s\n", (char*)("tab\tseparated"));
    printf("%s\n", (char*)("vertical\013tab"));
    return 0;
}