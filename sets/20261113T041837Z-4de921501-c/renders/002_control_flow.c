#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */

static char *greet(char *_p1, int *_rcp, int _noecho) {
    static char *_v2; free(_v2); _v2 = 0;
    if (asprintf(&_v2, "Hello, %s!", ((_p1) ? (_p1) : "")) < 0) _v2 = 0;
    char *_ret3 = ({ const char *_sd0 = (char*)(_v2); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; });
    return _ret3;
}

int main(void) {
    int sh_status = 0;
    long long i = 0;
    if (({ struct stat _st2; stat("file.txt", &_st2) == 0 && S_ISREG(_st2.st_mode); })) {
        fputs("File exists\n", stdout);
    } else {
        fputs("File does not exist\n", stdout);
    }
    static const long long _for_i_0[] = {1, 2, 3, 4, 5};
    for (size_t _i_i = 0; _i_i < 5; _i_i++) {
        i = _for_i_0[_i_i];
        printf("Number: %lld\n", i);
    }
    sh_status = 0;
    while ((i < 10)) {
        printf("Counter: %lld\n", i), sh_status = 0;
        i++;
    }
    char *_greet_World = greet("World", &sh_status, 1);
    printf("%s\n", ((char*)(_greet_World) ? (char*)(_greet_World) : "")), sh_status = 0;
    free(_greet_World);
    return sh_status;
}