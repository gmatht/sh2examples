#include <stdio.h>


int main(void) {
    int sh_status = 0;
    fputs("Hello, World!\n", stdout);
    fputs("Single quoted\n", stdout);
    fputs("String with \"escaped\" quotes\n", stdout);
    fputs("String with 'single' quotes\n", stdout), sh_status = 0;
    printf("exit: %d\n", sh_status), sh_status = 0;
    return sh_status;
}