#include <stdio.h>
#include <stdlib.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */

int main(int argc, char **argv) {
    printf("Invalid configuration `%s\\': more than four components\n", (char*)(((1 < argc && argv[1]) ? argv[1] : "")));
    exit(1);
    return 0;
}