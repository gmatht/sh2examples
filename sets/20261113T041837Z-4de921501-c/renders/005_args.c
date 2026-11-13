#include <stdio.h>

char* a = NULL;

int main(int argc, char **argv) {
    fputs("== Argument count ==\n", stdout);
    printf("%d\n", (argc - 1));
    fputs("== Arguments ==\n", stdout);
    for (size_t _ai_a = 1; _ai_a < (size_t)argc; _ai_a++) {
        a = (argv && argv[_ai_a]) ? argv[_ai_a] : "";
        printf("Arg: %s\n", a);
    }
    return 0;
}