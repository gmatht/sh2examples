#include <stdio.h>
#include <string.h>
static const char *_sh_sc(const char *x) { return x ? x : ""; }
#include <assert.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */
char __sh2_tc0[4] = "";

int main(void) {
    assert(strlen(__sh2_tc0) <= 3);

    {
        strncpy(__sh2_tc0, (char*)("foo"), 4);
        if ((strcmp(_sh_sc(__sh2_tc0), "") != 0)) {
            printf("%s\n", (char*)("yes"));
        }
    }
    return 0;
}