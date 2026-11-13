#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>


int main(void) {
    int sh_status = 0;
    char* formatted_date = NULL;
    char* yes_result = NULL;
    fputs("=== System Utilities ===\n", stdout);
    char *_a0_0[] = {"date", "+%Y-%m-%d", 0};
    static char *_pipe0 = 0; static size_t _pipe0_cap = 0;
    { char *_old1 = formatted_date; formatted_date = ({ const char *_sd0 = (char*)(({ if (_pipe0_cap < 1 || !_pipe0) { _pipe0_cap = 64; _pipe0 = (char*)realloc(_pipe0, _pipe0_cap); if (!_pipe0) { exit(127); } } _pipe0[0] = 0; int _pf0[2]; int _ok0 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf0 + 2*_k) != 0) { _ok0 = 0; break; } if (!_ok0) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp0[1]; for (int _t = 0; (_cp0[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp0[0] == 0) { dup2(_pf0[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } execvp(_a0_0[0], _a0_0); _exit(127); } close(_pf0[1]); size_t _gn0 = 0; for (;;) { if (_gn0 + 8192 > _pipe0_cap) { _pipe0_cap = _gn0 + 16384; _pipe0 = (char*)realloc(_pipe0, _pipe0_cap); if (!_pipe0) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf0[0], _pipe0 + _gn0, _pipe0_cap - _gn0 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn0 += (size_t)_r; } close(_pf0[0]); _pipe0[_gn0] = 0; int _st0 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp0[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st0 = _s; } sh_status = WIFEXITED(_st0) ? WEXITSTATUS(_st0) : 1; while (_gn0 > 0 && (_pipe0[_gn0-1] == '\n' || _pipe0[_gn0-1] == '\r')) _pipe0[--_gn0] = 0; } (char*)_pipe0; })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old1); }
    printf("Formatted date: %s\n", formatted_date);
    printf("Sleeping for %s seconds...\n", "1");
    char *_a2[] = {"sleep", "1", 0};
     ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a2)[0], (_a2)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    char *_a3_0[] = {"yes", "Hello", 0};
    char *_a3_1[] = {"head", "-3", 0};
    static char *_pipe3 = 0; static size_t _pipe3_cap = 0;
    { char *_old4 = yes_result; yes_result = ({ const char *_sd1 = (char*)(({ if (_pipe3_cap < 1 || !_pipe3) { _pipe3_cap = 64; _pipe3 = (char*)realloc(_pipe3, _pipe3_cap); if (!_pipe3) { exit(127); } } _pipe3[0] = 0; int _pf3[4]; int _ok3 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf3 + 2*_k) != 0) { _ok3 = 0; break; } if (!_ok3) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp3[2]; for (int _t = 0; (_cp3[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp3[0] == 0) { dup2(_pf3[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf3[2*_k]); close(_pf3[2*_k+1]); } execvp(_a3_0[0], _a3_0); _exit(127); } for (int _t = 0; (_cp3[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp3[1] == 0) { dup2(_pf3[0], 0); dup2(_pf3[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf3[2*_k]); close(_pf3[2*_k+1]); } execvp(_a3_1[0], _a3_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf3[2*_k]); close(_pf3[2*_k+1]); } close(_pf3[3]); size_t _gn3 = 0; for (;;) { if (_gn3 + 8192 > _pipe3_cap) { _pipe3_cap = _gn3 + 16384; _pipe3 = (char*)realloc(_pipe3, _pipe3_cap); if (!_pipe3) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf3[2], _pipe3 + _gn3, _pipe3_cap - _gn3 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn3 += (size_t)_r; } close(_pf3[2]); _pipe3[_gn3] = 0; int _st3 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp3[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st3 = _s; } sh_status = WIFEXITED(_st3) ? WEXITSTATUS(_st3) : 1; while (_gn3 > 0 && (_pipe3[_gn3-1] == '\n' || _pipe3[_gn3-1] == '\r')) _pipe3[--_gn3] = 0; } (char*)_pipe3; })); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; }); free(_old4); }
    fputs("Yes command result:\n", stdout);
    printf("%s\n", yes_result);
    fputs("=== System Utilities Complete ===\n", stdout), sh_status = 0;
    free(yes_result);
    free(formatted_date);
    return sh_status;
}