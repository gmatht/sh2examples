#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    printf("%s\n", (char*)("hello"));
    {
        fprintf(stderr, "%s\n", (char*)("syntax error near unexpected token `}'"));
        return 2;
    }
    return 0;
}