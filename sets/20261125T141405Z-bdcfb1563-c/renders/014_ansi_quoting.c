#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("== ANSI-C quoting =="));
    printf("%s\n", (char*)("line1\nline2\tTabbed"));
    printf("%s\n", (char*)("== Escape sequences =="));
    printf("%s\n", (char*)("bell\007"));
    printf("%s\n", (char*)("backspace\010"));
    printf("%s\n", (char*)("formfeed\014"));
    printf("%s\n", (char*)("newline\n"));
    printf("%s\n", (char*)("carriage\rreturn"));
    printf("%s\n", (char*)("tab\tseparated"));
    printf("%s\n", (char*)("vertical\013tab"));
    printf("%s\n", (char*)("== Unicode and hex =="));
    printf("%s\n", (char*)("Hello"));
    printf("%s\n", (char*)("Hello"));
    printf("%s\n", (char*)("== Practical examples =="));
    fputs((char*)("Name       Age        City\n"), stdout);
    fputs((char*)("John       25         NYC\n"), stdout);
    fputs((char*)("Jane       30         LA\n"), stdout);
    return 0;
}