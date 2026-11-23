#include <stdio.h>
#include <string.h>
#include <assert.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */
char __sh2_tc0[6] = "";

int main(void) {
    assert(strlen(__sh2_tc0) <= 5);

    {
        strncpy(__sh2_tc0, (char*)("hello"), 6);
        if ((strcmp(__sh2_tc0, "hello") == 0)) {
            printf("%s\n", (char*)("match"));
        }
    }
    return 0;
}