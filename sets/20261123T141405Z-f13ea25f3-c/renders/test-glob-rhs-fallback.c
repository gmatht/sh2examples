#include <stdio.h>
#include <string.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    if ((strcmp("abc", "a*") == 0)) {
        printf("%s\n", (char*)("bad"));
    } else {
        printf("%s\n", (char*)("literal-star-nomatch"));
    }
    if ((strcmp("xyz", "a*") == 0)) {
        printf("%s\n", (char*)("bad"));
    } else {
        printf("%s\n", (char*)("const-star-nomatch"));
    }
    if ((strcmp("abc", "a*") == 0)) {
        printf("%s\n", (char*)("bad"));
    } else {
        printf("%s\n", (char*)("var-star-nomatch"));
    }
    if ((strcmp("abc", "a*") == 0)) {
        printf("%s\n", (char*)("bad"));
    } else {
        printf("%s\n", (char*)("db-bracket-nomatch"));
    }
    if ((strcmp("abc", "[ab]*") == 0)) {
        printf("%s\n", (char*)("bad"));
    } else {
        printf("%s\n", (char*)("bracket-literal-nomatch"));
    }
    return 0;
}