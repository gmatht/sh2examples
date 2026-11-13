#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>


int main(void) {
    int sh_status = 0;
    char* current_date = NULL;
    char* current_dir = NULL;
    fputs("=== Basic Command Substitution ===\n", stdout), sh_status = 0;
    char *_a0_0[] = {"date", "+%Y", 0};
    static char *_pipe0 = 0; static size_t _pipe0_cap = 0;
    long long _q1 = sh_status;
    static char *_v2 = 0; static size_t _v2_cap = 0;
    _v2 = ({ if (_pipe0_cap < 1 || !_pipe0) { _pipe0_cap = 64; _pipe0 = (char*)realloc(_pipe0, _pipe0_cap); if (!_pipe0) { exit(127); } } _pipe0[0] = 0; int _pf0[2]; int _ok0 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf0 + 2*_k) != 0) { _ok0 = 0; break; } if (!_ok0) { _q1 = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp0[1]; for (int _t = 0; (_cp0[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp0[0] == 0) { dup2(_pf0[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } execvp(_a0_0[0], _a0_0); _exit(127); } close(_pf0[1]); size_t _gn0 = 0; for (;;) { if (_gn0 + 8192 > _pipe0_cap) { _pipe0_cap = _gn0 + 16384; _pipe0 = (char*)realloc(_pipe0, _pipe0_cap); if (!_pipe0) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf0[0], _pipe0 + _gn0, _pipe0_cap - _gn0 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn0 += (size_t)_r; } close(_pf0[0]); _pipe0[_gn0] = 0; int _st0 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp0[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st0 = _s; } _q1 = WIFEXITED(_st0) ? WEXITSTATUS(_st0) : 1; while (_gn0 > 0 && (_pipe0[_gn0-1] == '\n' || _pipe0[_gn0-1] == '\r')) _pipe0[--_gn0] = 0; } (char*)_pipe0; });
    printf("Current date: %s\n", ((char*)(_v2) ? (char*)(_v2) : "")), sh_status = 0;
    long long _q3 = sh_status;
    static char *_v4 = 0; static size_t _v4_cap = 0;
    _v4 = ({ const char *_u = ({ char *_p = getcwd(0, 0); _q3 = _p ? 0 : 1; _p ? _p : strdup(""); }); size_t _l = strlen(_u); while (_l > 1 && _u[_l - 1] == '/') _l--; size_t _k = _l; while (_k > 0 && _u[_k - 1] != '/') _k--; const char *_b = _u + _k; size_t _n = _l - _k; if (_n == 0 && _l > 0 && _u[0] == '/') { _b = "/"; _n = 1; } const char *_suf = ""; size_t _sn = strlen(_suf); if (_sn && _sn < _n && !memcmp(_b + _n - _sn, _suf, _sn)) _n -= _sn; char *_d = (char*)malloc(_n + 1); if (!_d) { fputs("sh2c: out of memory\n", stderr); exit(127); } memcpy(_d, _b, _n); _d[_n] = 0; _q3 = 0; _d; });
    printf("Current directory: %s\n", ((char*)(_v4) ? (char*)(_v4) : ""));
    char *_a5_0[] = {"date", "+%Y%m", 0};
    static char *_pipe5 = 0; static size_t _pipe5_cap = 0;
    { char *_old6 = current_date; current_date = ({ const char *_sd0 = (char*)(({ if (_pipe5_cap < 1 || !_pipe5) { _pipe5_cap = 64; _pipe5 = (char*)realloc(_pipe5, _pipe5_cap); if (!_pipe5) { exit(127); } } _pipe5[0] = 0; int _pf5[2]; int _ok5 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf5 + 2*_k) != 0) { _ok5 = 0; break; } if (!_ok5) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp5[1]; for (int _t = 0; (_cp5[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp5[0] == 0) { dup2(_pf5[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } execvp(_a5_0[0], _a5_0); _exit(127); } close(_pf5[1]); size_t _gn5 = 0; for (;;) { if (_gn5 + 8192 > _pipe5_cap) { _pipe5_cap = _gn5 + 16384; _pipe5 = (char*)realloc(_pipe5, _pipe5_cap); if (!_pipe5) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf5[0], _pipe5 + _gn5, _pipe5_cap - _gn5 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn5 += (size_t)_r; } close(_pf5[0]); _pipe5[_gn5] = 0; int _st5 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp5[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st5 = _s; } sh_status = WIFEXITED(_st5) ? WEXITSTATUS(_st5) : 1; while (_gn5 > 0 && (_pipe5[_gn5-1] == '\n' || _pipe5[_gn5-1] == '\r')) _pipe5[--_gn5] = 0; } (char*)_pipe5; })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old6); }
    { char *_old7 = current_dir; current_dir = ({ const char *_sd1 = (char*)(({ const char *_u = ({ char *_p = getcwd(0, 0); sh_status = _p ? 0 : 1; _p ? _p : strdup(""); }); size_t _l = strlen(_u); while (_l > 1 && _u[_l - 1] == '/') _l--; size_t _k = _l; while (_k > 0 && _u[_k - 1] != '/') _k--; const char *_b = _u + _k; size_t _n = _l - _k; if (_n == 0 && _l > 0 && _u[0] == '/') { _b = "/"; _n = 1; } const char *_suf = ""; size_t _sn = strlen(_suf); if (_sn && _sn < _n && !memcmp(_b + _n - _sn, _suf, _sn)) _n -= _sn; char *_d = (char*)malloc(_n + 1); if (!_d) { fputs("sh2c: out of memory\n", stderr); exit(127); } memcpy(_d, _b, _n); _d[_n] = 0; sh_status = 0; _d; })); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; }); free(_old7); }
    printf("Stored date: %s\n", current_date);
    printf("Stored directory: %s\n", current_dir);
    fputs("=== Basic Command Substitution Complete ===\n", stdout), sh_status = 0;
    free(current_dir);
    free(current_date);
    return sh_status;
}