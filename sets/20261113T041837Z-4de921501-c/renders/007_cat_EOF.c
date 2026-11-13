#include <stdio.h>


int main(void) {
    int sh_status = 0;
    fputs((char*)("alpha\nbeta\ngamma ...\n"), stdout);
    fputs((char*)("oyster\nsnapper\nsalmon\n"), stdout);
    fputs("Fin. That is all folks.\n", stdout), sh_status = 0;
    printf("exit: %d\n", sh_status), sh_status = 0;
    return sh_status;
}