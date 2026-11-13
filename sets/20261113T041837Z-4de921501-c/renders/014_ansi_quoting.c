#include <stdio.h>


int main(void) {
    fputs("== ANSI-C quoting ==\n", stdout);
    fputs("line1\nline2\tTabbed\n", stdout);
    fputs("== Escape sequences ==\n", stdout);
    fputs("bell\x07\n", stdout);
    fputs("backspace\x08\n", stdout);
    fputs("formfeed\x0c\n", stdout);
    fputs("newline\n\n", stdout);
    fputs("carriage\rreturn\n", stdout);
    fputs("tab\tseparated\n", stdout);
    fputs("vertical\x0btab\n", stdout);
    fputs("== Unicode and hex ==\n", stdout);
    fputs("Hello\n", stdout);
    fputs("Hello\n", stdout);
    fputs("== Practical examples ==\n", stdout);
    printf("%-10s %-10s %s\n", "Name", "Age", "City");
    printf("%-10s %-10s %s\n", "John", "25", "NYC");
    printf("%-10s %-10s %s\n", "Jane", "30", "LA");
    return 0;
}