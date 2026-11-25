#include <stdio.h>
#include <string.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    if ((strlen("hello") >= strlen("llo") && strcmp("hello" + strlen("hello") - strlen("llo"), "llo") == 0)) {
        printf("%s\n", (char*)("case=match"));
    } else {
        printf("%s\n", (char*)("case=no"));
    }
    if ((strlen("abc") >= strlen("llo") && strcmp("abc" + strlen("abc") - strlen("llo"), "llo") == 0)) {
        printf("%s\n", (char*)("case2=match"));
    } else {
        printf("%s\n", (char*)("case2=no"));
    }
    return 0;
}