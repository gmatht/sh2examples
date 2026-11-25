#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */
char maybe[8] = "";

int main(void) {
    assert(strlen(maybe) <= 7);

    printf("%s\n", (char*)("== Default values =="));
    maybe[0] = '\0';
    printf("%s\n", ((char*)((((maybe) && (maybe)[0]) ? (maybe) : ("default"))) ? (char*)((((maybe) && (maybe)[0]) ? (maybe) : ("default"))) : ""));
    printf("%s\n", ((char*)((strcpy(maybe, (((maybe) && (maybe)[0]) ? (maybe) : ("default"))), (((maybe) && (maybe)[0]) ? (maybe) : ("default")))) ? (char*)((strcpy(maybe, (((maybe) && (maybe)[0]) ? (maybe) : ("default"))), (((maybe) && (maybe)[0]) ? (maybe) : ("default")))) : ""));
    printf("%s\n", ((char*)((((maybe) && (maybe)[0]) ? (maybe) : (fprintf(stderr, "%s\n", "error"), exit(1), (char*)0))) ? (char*)((((maybe) && (maybe)[0]) ? (maybe) : (fprintf(stderr, "%s\n", "error"), exit(1), (char*)0))) : ""));
    return 0;
}