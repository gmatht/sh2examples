#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <sys/stat.h>

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */
uint8_t i = 0;

int main(void) {
    fputs("Hello, World!\n", stdout);
    if (({ struct stat _st2; stat("test.txt", &_st2) == 0 && S_ISREG(_st2.st_mode); })) {
        fputs("File exists\n", stdout);
    }
    static const long long _for_i_0[] = {1, 2, 3, 4, 5};
    for (size_t _i_i = 0; _i_i < 5; _i_i++) {
        i = _for_i_0[_i_i];
        static char _s0[32];
        snprintf(_s0, sizeof _s0, "%lld", (long long)(i));
        static char *_sj1b = 0; static size_t _sj1b_cap = 0; free(_sj1b); _sj1b_cap = strlen(_s0) + 2; _sj1b = (char*)malloc(_sj1b_cap); if (!_sj1b) { _sj1b_cap = 0; fputs("sh2c: out of memory\n", stderr); exit(127); } strcpy(_sj1b + 1, _s0); size_t _sj1o = 1; { char *_sj1i = _sj1b + 1, *_sj1d = _sj1b + 1; while (*_sj1i == ' ' || *_sj1i == '\t' || *_sj1i == '\n') _sj1i++; while (*_sj1i) { char _sj1c = *_sj1i++; if (_sj1c == ' ' || _sj1c == '\t' || _sj1c == '\n') { while (*_sj1i == ' ' || *_sj1i == '\t' || *_sj1i == '\n') _sj1i++; if (*_sj1i) *_sj1d++ = ' '; } else *_sj1d++ = _sj1c; } *_sj1d = 0; }
        printf("%s\n", ((char*)(_sj1b + _sj1o) ? (char*)(_sj1b + _sj1o) : ""));
    }
    return 0;
}