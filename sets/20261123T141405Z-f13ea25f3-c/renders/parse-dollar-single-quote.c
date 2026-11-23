#include <stdio.h>
#include <string.h>
#include <assert.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(void) {
    assert(strlen((char*)("\n\t")) <= 2);
    strncpy(IFS, (char*)("\n\t"), 3);
    IFS[2] = '\0';
    fputs((char*)("IFS value (hex)="), stdout);
    fputs((char*)("$'\\n\\t'"), stdout);
    fputs((char*)("\n"), stdout);
    return 0;
}