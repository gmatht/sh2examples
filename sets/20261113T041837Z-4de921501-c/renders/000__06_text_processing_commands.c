#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <fcntl.h>

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */

static int _stg4_0(void) {
  fputs("Hello World\n", stdout);
  return 0;
}

static int _stg6_0(void) {
  fputs("1 2 3 4 5\n", stdout);
  return 0;
}

static int _stg8_0(void) {
  fputs("zebra\napple\nbanana\n", stdout);
  return 0;
}

static int _stg10_0(void) {
  fputs("apple\napple\nbanana\nbanana\ncherry\n", stdout);
  return 0;
}

static int _stg12_0(void) {
  fputs("Hello World\n", stdout);
  return 0;
}

static int _stg14_0(void) {
  fputs("line1\nline2\nline3\n", stdout);
  return 0;
}

static int _stg20_0(void) {
  fputs("apple:banana:cherry\n", stdout);
  return 0;
}

static int _stg32_0(void) {
  fputs("HELLO WORLD\n", stdout);
  return 0;
}

static int _stg32_1(void) {
  static const unsigned char _trp32_1[][2] = {{65,97},{66,98},{67,99},{68,100},{69,101},{70,102},{71,103},{72,104},{73,105},{74,106},{75,107},{76,108},{77,109},{78,110},{79,111},{80,112},{81,113},{82,114},{83,115},{84,116},{85,117},{86,118},{87,119},{88,120},{89,121},{90,122}};
  char *_tl = 0; size_t _tlc = 0; long _tn;
  while ((_tn = getline(&_tl, &_tlc, stdin)) > 0) { char *_te = _tl + _tn; for (char *_p = _tl; _p < _te; _p++) for (size_t _k = 0; _k < sizeof _trp32_1 / sizeof _trp32_1[0]; _k++) if (_trp32_1[_k][0] == (unsigned char)*_p) { *_p = (char)_trp32_1[_k][1]; break; } fwrite(_tl, 1, (size_t)_tn, stdout); }
  free(_tl); return 0;
}

static int _stg34_0(void) {
  fputs("1 2 3\n", stdout);
  return 0;
}

int main(void) {
    int sh_status = 0;
    char* awk_result = NULL;
    char* comm_result = NULL;
    char* cut_result = NULL;
    char* diff_result = NULL;
    char* file_content = NULL;
    char* grep_result = NULL;
    char* head_result = NULL;
    char* line_count = NULL;
    char* paste_result = NULL;
    char* sed_result = NULL;
    char* sort_result = NULL;
    char* tail_result = NULL;
    char* tr_result = NULL;
    char* uniq_result = NULL;
    char* word_count = NULL;
    char* xargs_result = NULL;
    fputs("=== Text Processing Commands ===\n", stdout);
    char *_a0_0[] = {"cat", "src/main.rs", 0};
    char *_a0_1[] = {"head", "-5", 0};
    static char *_pipe0 = 0; static size_t _pipe0_cap = 0;
    { char *_old1 = file_content; file_content = ({ const char *_sd0 = (char*)(({ if (_pipe0_cap < 1 || !_pipe0) { _pipe0_cap = 64; _pipe0 = (char*)realloc(_pipe0, _pipe0_cap); if (!_pipe0) { exit(127); } } _pipe0[0] = 0; int _pf0[4]; int _ok0 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf0 + 2*_k) != 0) { _ok0 = 0; break; } if (!_ok0) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp0[2]; for (int _t = 0; (_cp0[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp0[0] == 0) { dup2(_pf0[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } execvp(_a0_0[0], _a0_0); _exit(127); } for (int _t = 0; (_cp0[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp0[1] == 0) { dup2(_pf0[0], 0); dup2(_pf0[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } execvp(_a0_1[0], _a0_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } close(_pf0[3]); size_t _gn0 = 0; for (;;) { if (_gn0 + 8192 > _pipe0_cap) { _pipe0_cap = _gn0 + 16384; _pipe0 = (char*)realloc(_pipe0, _pipe0_cap); if (!_pipe0) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf0[2], _pipe0 + _gn0, _pipe0_cap - _gn0 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn0 += (size_t)_r; } close(_pf0[2]); _pipe0[_gn0] = 0; int _st0 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp0[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st0 = _s; } sh_status = WIFEXITED(_st0) ? WEXITSTATUS(_st0) : 1; while (_gn0 > 0 && (_pipe0[_gn0-1] == '\n' || _pipe0[_gn0-1] == '\r')) _pipe0[--_gn0] = 0; } (char*)_pipe0; })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old1); }
    fputs("First 5 lines of main.rs:\n", stdout);
    printf("%s\n", file_content);
    char *_a2_0[] = {"grep", "-n", "fn", "src/main.rs", 0};
    static char *_pipe2 = 0; static size_t _pipe2_cap = 0;
    { char *_old3 = grep_result; grep_result = ({ const char *_sd1 = (char*)(({ if (_pipe2_cap < 1 || !_pipe2) { _pipe2_cap = 64; _pipe2 = (char*)realloc(_pipe2, _pipe2_cap); if (!_pipe2) { exit(127); } } _pipe2[0] = 0; int _pf2[2]; int _ok2 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf2 + 2*_k) != 0) { _ok2 = 0; break; } if (!_ok2) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp2[1]; for (int _t = 0; (_cp2[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp2[0] == 0) { dup2(_pf2[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf2[2*_k]); close(_pf2[2*_k+1]); } execvp(_a2_0[0], _a2_0); _exit(127); } close(_pf2[1]); size_t _gn2 = 0; for (;;) { if (_gn2 + 8192 > _pipe2_cap) { _pipe2_cap = _gn2 + 16384; _pipe2 = (char*)realloc(_pipe2, _pipe2_cap); if (!_pipe2) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf2[0], _pipe2 + _gn2, _pipe2_cap - _gn2 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn2 += (size_t)_r; } close(_pf2[0]); _pipe2[_gn2] = 0; int _st2 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp2[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st2 = _s; } sh_status = WIFEXITED(_st2) ? WEXITSTATUS(_st2) : 1; while (_gn2 > 0 && (_pipe2[_gn2-1] == '\n' || _pipe2[_gn2-1] == '\r')) _pipe2[--_gn2] = 0; } (char*)_pipe2; })); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; }); free(_old3); }
    fputs("Lines containing 'fn':\n", stdout);
    printf("%s\n", grep_result);
    char *_a4_1[] = {"sed", "s/World/Universe/", 0};
    static char *_pipe4 = 0; static size_t _pipe4_cap = 0;
    { char *_old5 = sed_result; sed_result = ({ const char *_sd2 = (char*)(({ if (_pipe4_cap < 1 || !_pipe4) { _pipe4_cap = 64; _pipe4 = (char*)realloc(_pipe4, _pipe4_cap); if (!_pipe4) { exit(127); } } _pipe4[0] = 0; int _pf4[4]; int _ok4 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf4 + 2*_k) != 0) { _ok4 = 0; break; } if (!_ok4) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp4[2]; for (int _t = 0; (_cp4[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp4[0] == 0) { dup2(_pf4[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf4[2*_k]); close(_pf4[2*_k+1]); } int _tr = _stg4_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp4[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp4[1] == 0) { dup2(_pf4[0], 0); dup2(_pf4[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf4[2*_k]); close(_pf4[2*_k+1]); } execvp(_a4_1[0], _a4_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf4[2*_k]); close(_pf4[2*_k+1]); } close(_pf4[3]); size_t _gn4 = 0; for (;;) { if (_gn4 + 8192 > _pipe4_cap) { _pipe4_cap = _gn4 + 16384; _pipe4 = (char*)realloc(_pipe4, _pipe4_cap); if (!_pipe4) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf4[2], _pipe4 + _gn4, _pipe4_cap - _gn4 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn4 += (size_t)_r; } close(_pf4[2]); _pipe4[_gn4] = 0; int _st4 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp4[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st4 = _s; } sh_status = WIFEXITED(_st4) ? WEXITSTATUS(_st4) : 1; while (_gn4 > 0 && (_pipe4[_gn4-1] == '\n' || _pipe4[_gn4-1] == '\r')) _pipe4[--_gn4] = 0; } (char*)_pipe4; })); char *_xp2 = _sd2 ? strdup(_sd2) : 0; if (_sd2 && !_xp2) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp2; }); free(_old5); }
    printf("Sed result: %s\n", sed_result);
    char *_a6_1[] = {"awk", "{print $1 + $2}", 0};
    static char *_pipe6 = 0; static size_t _pipe6_cap = 0;
    { char *_old7 = awk_result; awk_result = ({ const char *_sd3 = (char*)(({ if (_pipe6_cap < 1 || !_pipe6) { _pipe6_cap = 64; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { exit(127); } } _pipe6[0] = 0; int _pf6[4]; int _ok6 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf6 + 2*_k) != 0) { _ok6 = 0; break; } if (!_ok6) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp6[2]; for (int _t = 0; (_cp6[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[0] == 0) { dup2(_pf6[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } int _tr = _stg6_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp6[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[1] == 0) { dup2(_pf6[0], 0); dup2(_pf6[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_1[0], _a6_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } close(_pf6[3]); size_t _gn6 = 0; for (;;) { if (_gn6 + 8192 > _pipe6_cap) { _pipe6_cap = _gn6 + 16384; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf6[2], _pipe6 + _gn6, _pipe6_cap - _gn6 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn6 += (size_t)_r; } close(_pf6[2]); _pipe6[_gn6] = 0; int _st6 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp6[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st6 = _s; } sh_status = WIFEXITED(_st6) ? WEXITSTATUS(_st6) : 1; while (_gn6 > 0 && (_pipe6[_gn6-1] == '\n' || _pipe6[_gn6-1] == '\r')) _pipe6[--_gn6] = 0; } (char*)_pipe6; })); char *_xp3 = _sd3 ? strdup(_sd3) : 0; if (_sd3 && !_xp3) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp3; }); free(_old7); }
    printf("Awk sum result: %s\n", awk_result);
    char *_a8_1[] = {"sort", 0};
    static char *_pipe8 = 0; static size_t _pipe8_cap = 0;
    { char *_old9 = sort_result; sort_result = ({ const char *_sd4 = (char*)(({ if (_pipe8_cap < 1 || !_pipe8) { _pipe8_cap = 64; _pipe8 = (char*)realloc(_pipe8, _pipe8_cap); if (!_pipe8) { exit(127); } } _pipe8[0] = 0; int _pf8[4]; int _ok8 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf8 + 2*_k) != 0) { _ok8 = 0; break; } if (!_ok8) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp8[2]; for (int _t = 0; (_cp8[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp8[0] == 0) { dup2(_pf8[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } int _tr = _stg8_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp8[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp8[1] == 0) { dup2(_pf8[0], 0); dup2(_pf8[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } execvp(_a8_1[0], _a8_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } close(_pf8[3]); size_t _gn8 = 0; for (;;) { if (_gn8 + 8192 > _pipe8_cap) { _pipe8_cap = _gn8 + 16384; _pipe8 = (char*)realloc(_pipe8, _pipe8_cap); if (!_pipe8) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf8[2], _pipe8 + _gn8, _pipe8_cap - _gn8 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn8 += (size_t)_r; } close(_pf8[2]); _pipe8[_gn8] = 0; int _st8 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp8[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st8 = _s; } sh_status = WIFEXITED(_st8) ? WEXITSTATUS(_st8) : 1; while (_gn8 > 0 && (_pipe8[_gn8-1] == '\n' || _pipe8[_gn8-1] == '\r')) _pipe8[--_gn8] = 0; } (char*)_pipe8; })); char *_xp4 = _sd4 ? strdup(_sd4) : 0; if (_sd4 && !_xp4) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp4; }); free(_old9); }
    fputs("Sorted words:\n", stdout);
    printf("%s\n", sort_result);
    char *_a10_1[] = {"uniq", 0};
    static char *_pipe10 = 0; static size_t _pipe10_cap = 0;
    { char *_old11 = uniq_result; uniq_result = ({ const char *_sd5 = (char*)(({ if (_pipe10_cap < 1 || !_pipe10) { _pipe10_cap = 64; _pipe10 = (char*)realloc(_pipe10, _pipe10_cap); if (!_pipe10) { exit(127); } } _pipe10[0] = 0; int _pf10[4]; int _ok10 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf10 + 2*_k) != 0) { _ok10 = 0; break; } if (!_ok10) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp10[2]; for (int _t = 0; (_cp10[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp10[0] == 0) { dup2(_pf10[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf10[2*_k]); close(_pf10[2*_k+1]); } int _tr = _stg10_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp10[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp10[1] == 0) { dup2(_pf10[0], 0); dup2(_pf10[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf10[2*_k]); close(_pf10[2*_k+1]); } execvp(_a10_1[0], _a10_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf10[2*_k]); close(_pf10[2*_k+1]); } close(_pf10[3]); size_t _gn10 = 0; for (;;) { if (_gn10 + 8192 > _pipe10_cap) { _pipe10_cap = _gn10 + 16384; _pipe10 = (char*)realloc(_pipe10, _pipe10_cap); if (!_pipe10) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf10[2], _pipe10 + _gn10, _pipe10_cap - _gn10 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn10 += (size_t)_r; } close(_pf10[2]); _pipe10[_gn10] = 0; int _st10 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp10[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st10 = _s; } sh_status = WIFEXITED(_st10) ? WEXITSTATUS(_st10) : 1; while (_gn10 > 0 && (_pipe10[_gn10-1] == '\n' || _pipe10[_gn10-1] == '\r')) _pipe10[--_gn10] = 0; } (char*)_pipe10; })); char *_xp5 = _sd5 ? strdup(_sd5) : 0; if (_sd5 && !_xp5) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp5; }); free(_old11); }
    fputs("Unique words:\n", stdout);
    printf("%s\n", uniq_result);
    char *_a12_1[] = {"wc", "-w", 0};
    static char *_pipe12 = 0; static size_t _pipe12_cap = 0;
    { char *_old13 = word_count; word_count = ({ const char *_sd6 = (char*)(({ if (_pipe12_cap < 1 || !_pipe12) { _pipe12_cap = 64; _pipe12 = (char*)realloc(_pipe12, _pipe12_cap); if (!_pipe12) { exit(127); } } _pipe12[0] = 0; int _pf12[4]; int _ok12 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf12 + 2*_k) != 0) { _ok12 = 0; break; } if (!_ok12) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp12[2]; for (int _t = 0; (_cp12[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp12[0] == 0) { dup2(_pf12[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf12[2*_k]); close(_pf12[2*_k+1]); } int _tr = _stg12_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp12[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp12[1] == 0) { dup2(_pf12[0], 0); dup2(_pf12[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf12[2*_k]); close(_pf12[2*_k+1]); } execvp(_a12_1[0], _a12_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf12[2*_k]); close(_pf12[2*_k+1]); } close(_pf12[3]); size_t _gn12 = 0; for (;;) { if (_gn12 + 8192 > _pipe12_cap) { _pipe12_cap = _gn12 + 16384; _pipe12 = (char*)realloc(_pipe12, _pipe12_cap); if (!_pipe12) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf12[2], _pipe12 + _gn12, _pipe12_cap - _gn12 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn12 += (size_t)_r; } close(_pf12[2]); _pipe12[_gn12] = 0; int _st12 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp12[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st12 = _s; } sh_status = WIFEXITED(_st12) ? WEXITSTATUS(_st12) : 1; while (_gn12 > 0 && (_pipe12[_gn12-1] == '\n' || _pipe12[_gn12-1] == '\r')) _pipe12[--_gn12] = 0; } (char*)_pipe12; })); char *_xp6 = _sd6 ? strdup(_sd6) : 0; if (_sd6 && !_xp6) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp6; }); free(_old13); }
    char *_a14_1[] = {"wc", "-l", 0};
    static char *_pipe14 = 0; static size_t _pipe14_cap = 0;
    { char *_old15 = line_count; line_count = ({ const char *_sd7 = (char*)(({ if (_pipe14_cap < 1 || !_pipe14) { _pipe14_cap = 64; _pipe14 = (char*)realloc(_pipe14, _pipe14_cap); if (!_pipe14) { exit(127); } } _pipe14[0] = 0; int _pf14[4]; int _ok14 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf14 + 2*_k) != 0) { _ok14 = 0; break; } if (!_ok14) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp14[2]; for (int _t = 0; (_cp14[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp14[0] == 0) { dup2(_pf14[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf14[2*_k]); close(_pf14[2*_k+1]); } int _tr = _stg14_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp14[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp14[1] == 0) { dup2(_pf14[0], 0); dup2(_pf14[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf14[2*_k]); close(_pf14[2*_k+1]); } execvp(_a14_1[0], _a14_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf14[2*_k]); close(_pf14[2*_k+1]); } close(_pf14[3]); size_t _gn14 = 0; for (;;) { if (_gn14 + 8192 > _pipe14_cap) { _pipe14_cap = _gn14 + 16384; _pipe14 = (char*)realloc(_pipe14, _pipe14_cap); if (!_pipe14) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf14[2], _pipe14 + _gn14, _pipe14_cap - _gn14 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn14 += (size_t)_r; } close(_pf14[2]); _pipe14[_gn14] = 0; int _st14 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp14[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st14 = _s; } sh_status = WIFEXITED(_st14) ? WEXITSTATUS(_st14) : 1; while (_gn14 > 0 && (_pipe14[_gn14-1] == '\n' || _pipe14[_gn14-1] == '\r')) _pipe14[--_gn14] = 0; } (char*)_pipe14; })); char *_xp7 = _sd7 ? strdup(_sd7) : 0; if (_sd7 && !_xp7) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp7; }); free(_old15); }
    printf("Word count: %s\n", word_count);
    printf("Line count: %s\n", line_count);
    char *_a16_0[] = {"seq", "1", "10", 0};
    char *_a16_1[] = {"head", "-3", 0};
    static char *_pipe16 = 0; static size_t _pipe16_cap = 0;
    { char *_old17 = head_result; head_result = ({ const char *_sd8 = (char*)(({ if (_pipe16_cap < 1 || !_pipe16) { _pipe16_cap = 64; _pipe16 = (char*)realloc(_pipe16, _pipe16_cap); if (!_pipe16) { exit(127); } } _pipe16[0] = 0; int _pf16[4]; int _ok16 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf16 + 2*_k) != 0) { _ok16 = 0; break; } if (!_ok16) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp16[2]; for (int _t = 0; (_cp16[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp16[0] == 0) { dup2(_pf16[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf16[2*_k]); close(_pf16[2*_k+1]); } execvp(_a16_0[0], _a16_0); _exit(127); } for (int _t = 0; (_cp16[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp16[1] == 0) { dup2(_pf16[0], 0); dup2(_pf16[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf16[2*_k]); close(_pf16[2*_k+1]); } execvp(_a16_1[0], _a16_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf16[2*_k]); close(_pf16[2*_k+1]); } close(_pf16[3]); size_t _gn16 = 0; for (;;) { if (_gn16 + 8192 > _pipe16_cap) { _pipe16_cap = _gn16 + 16384; _pipe16 = (char*)realloc(_pipe16, _pipe16_cap); if (!_pipe16) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf16[2], _pipe16 + _gn16, _pipe16_cap - _gn16 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn16 += (size_t)_r; } close(_pf16[2]); _pipe16[_gn16] = 0; int _st16 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp16[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st16 = _s; } sh_status = WIFEXITED(_st16) ? WEXITSTATUS(_st16) : 1; while (_gn16 > 0 && (_pipe16[_gn16-1] == '\n' || _pipe16[_gn16-1] == '\r')) _pipe16[--_gn16] = 0; } (char*)_pipe16; })); char *_xp8 = _sd8 ? strdup(_sd8) : 0; if (_sd8 && !_xp8) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp8; }); free(_old17); }
    printf("First 3 numbers: %s\n", head_result);
    char *_a18_0[] = {"seq", "1", "10", 0};
    char *_a18_1[] = {"tail", "-3", 0};
    static char *_pipe18 = 0; static size_t _pipe18_cap = 0;
    { char *_old19 = tail_result; tail_result = ({ const char *_sd9 = (char*)(({ if (_pipe18_cap < 1 || !_pipe18) { _pipe18_cap = 64; _pipe18 = (char*)realloc(_pipe18, _pipe18_cap); if (!_pipe18) { exit(127); } } _pipe18[0] = 0; int _pf18[4]; int _ok18 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf18 + 2*_k) != 0) { _ok18 = 0; break; } if (!_ok18) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp18[2]; for (int _t = 0; (_cp18[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp18[0] == 0) { dup2(_pf18[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf18[2*_k]); close(_pf18[2*_k+1]); } execvp(_a18_0[0], _a18_0); _exit(127); } for (int _t = 0; (_cp18[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp18[1] == 0) { dup2(_pf18[0], 0); dup2(_pf18[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf18[2*_k]); close(_pf18[2*_k+1]); } execvp(_a18_1[0], _a18_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf18[2*_k]); close(_pf18[2*_k+1]); } close(_pf18[3]); size_t _gn18 = 0; for (;;) { if (_gn18 + 8192 > _pipe18_cap) { _pipe18_cap = _gn18 + 16384; _pipe18 = (char*)realloc(_pipe18, _pipe18_cap); if (!_pipe18) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf18[2], _pipe18 + _gn18, _pipe18_cap - _gn18 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn18 += (size_t)_r; } close(_pf18[2]); _pipe18[_gn18] = 0; int _st18 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp18[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st18 = _s; } sh_status = WIFEXITED(_st18) ? WEXITSTATUS(_st18) : 1; while (_gn18 > 0 && (_pipe18[_gn18-1] == '\n' || _pipe18[_gn18-1] == '\r')) _pipe18[--_gn18] = 0; } (char*)_pipe18; })); char *_xp9 = _sd9 ? strdup(_sd9) : 0; if (_sd9 && !_xp9) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp9; }); free(_old19); }
    printf("Last 3 numbers: %s\n", tail_result);
    char *_a20_1[] = {"cut", "-d", ":", "-f", "2", 0};
    static char *_pipe20 = 0; static size_t _pipe20_cap = 0;
    { char *_old21 = cut_result; cut_result = ({ const char *_sd10 = (char*)(({ if (_pipe20_cap < 1 || !_pipe20) { _pipe20_cap = 64; _pipe20 = (char*)realloc(_pipe20, _pipe20_cap); if (!_pipe20) { exit(127); } } _pipe20[0] = 0; int _pf20[4]; int _ok20 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf20 + 2*_k) != 0) { _ok20 = 0; break; } if (!_ok20) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp20[2]; for (int _t = 0; (_cp20[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp20[0] == 0) { dup2(_pf20[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf20[2*_k]); close(_pf20[2*_k+1]); } int _tr = _stg20_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp20[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp20[1] == 0) { dup2(_pf20[0], 0); dup2(_pf20[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf20[2*_k]); close(_pf20[2*_k+1]); } execvp(_a20_1[0], _a20_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf20[2*_k]); close(_pf20[2*_k+1]); } close(_pf20[3]); size_t _gn20 = 0; for (;;) { if (_gn20 + 8192 > _pipe20_cap) { _pipe20_cap = _gn20 + 16384; _pipe20 = (char*)realloc(_pipe20, _pipe20_cap); if (!_pipe20) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf20[2], _pipe20 + _gn20, _pipe20_cap - _gn20 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn20 += (size_t)_r; } close(_pf20[2]); _pipe20[_gn20] = 0; int _st20 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp20[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st20 = _s; } sh_status = WIFEXITED(_st20) ? WEXITSTATUS(_st20) : 1; while (_gn20 > 0 && (_pipe20[_gn20-1] == '\n' || _pipe20[_gn20-1] == '\r')) _pipe20[--_gn20] = 0; } (char*)_pipe20; })); char *_xp10 = _sd10 ? strdup(_sd10) : 0; if (_sd10 && !_xp10) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp10; }); free(_old21); }
    printf("Second field: %s\n", cut_result);
    fflush(stdout);
    int _nfd22_1 = open("temp1.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd22_1 = dup(1);
    if (_nfd22_1 >= 0) { dup2(_nfd22_1, 1); close(_nfd22_1); }
    if (_nfd22_1 >= 0) {
        fputs("1\n2\n3\n", stdout);
    }
    {
    fflush(stdout);
    dup2(_sfd22_1, 1); close(_sfd22_1);
    }
    fflush(stdout);
    int _nfd23_1 = open("temp2.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd23_1 = dup(1);
    if (_nfd23_1 >= 0) { dup2(_nfd23_1, 1); close(_nfd23_1); }
    if (_nfd23_1 >= 0) {
        fputs("a\nb\nc\n", stdout);
    }
    {
    fflush(stdout);
    dup2(_sfd23_1, 1); close(_sfd23_1);
    }
    char *_a24_0[] = {"paste", "temp1.txt", "temp2.txt", 0};
    char *_a24_1[] = {"sed", "s/\\t/ /g", 0};
    static char *_pipe24 = 0; static size_t _pipe24_cap = 0;
    { char *_old25 = paste_result; paste_result = ({ const char *_sd11 = (char*)(({ if (_pipe24_cap < 1 || !_pipe24) { _pipe24_cap = 64; _pipe24 = (char*)realloc(_pipe24, _pipe24_cap); if (!_pipe24) { exit(127); } } _pipe24[0] = 0; int _pf24[4]; int _ok24 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf24 + 2*_k) != 0) { _ok24 = 0; break; } if (!_ok24) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp24[2]; for (int _t = 0; (_cp24[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp24[0] == 0) { dup2(_pf24[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf24[2*_k]); close(_pf24[2*_k+1]); } execvp(_a24_0[0], _a24_0); _exit(127); } for (int _t = 0; (_cp24[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp24[1] == 0) { dup2(_pf24[0], 0); dup2(_pf24[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf24[2*_k]); close(_pf24[2*_k+1]); } execvp(_a24_1[0], _a24_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf24[2*_k]); close(_pf24[2*_k+1]); } close(_pf24[3]); size_t _gn24 = 0; for (;;) { if (_gn24 + 8192 > _pipe24_cap) { _pipe24_cap = _gn24 + 16384; _pipe24 = (char*)realloc(_pipe24, _pipe24_cap); if (!_pipe24) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf24[2], _pipe24 + _gn24, _pipe24_cap - _gn24 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn24 += (size_t)_r; } close(_pf24[2]); _pipe24[_gn24] = 0; int _st24 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp24[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st24 = _s; } sh_status = WIFEXITED(_st24) ? WEXITSTATUS(_st24) : 1; while (_gn24 > 0 && (_pipe24[_gn24-1] == '\n' || _pipe24[_gn24-1] == '\r')) _pipe24[--_gn24] = 0; } (char*)_pipe24; })); char *_xp11 = _sd11 ? strdup(_sd11) : 0; if (_sd11 && !_xp11) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp11; }); free(_old25); }
    fputs("Pasted columns:\n", stdout);
    printf("%s\n", paste_result), sh_status = 0;
    ({ int _mr = 0; { if (unlink("temp1.txt") != 0 && errno != ENOENT) _mr = 1; } { if (unlink("temp2.txt") != 0 && errno != ENOENT) _mr = 1; } sh_status = (_mr == 0 ? 0 : 1); (sh_status == 0); });
    fflush(stdout);
    int _nfd26_1 = open("file1.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd26_1 = dup(1);
    if (_nfd26_1 >= 0) { dup2(_nfd26_1, 1); close(_nfd26_1); }
    if (_nfd26_1 >= 0) {
        fputs("apple\nbanana\ncherry\n", stdout);
    }
    {
    fflush(stdout);
    dup2(_sfd26_1, 1); close(_sfd26_1);
    }
    fflush(stdout);
    int _nfd27_1 = open("file2.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd27_1 = dup(1);
    if (_nfd27_1 >= 0) { dup2(_nfd27_1, 1); close(_nfd27_1); }
    if (_nfd27_1 >= 0) {
        fputs("banana\ncherry\ndate\n", stdout);
    }
    {
    fflush(stdout);
    dup2(_sfd27_1, 1); close(_sfd27_1);
    }
    char *_a28_0[] = {"comm", "-12", "file1.txt", "file2.txt", 0};
    static char *_pipe28 = 0; static size_t _pipe28_cap = 0;
    { char *_old29 = comm_result; comm_result = ({ const char *_sd12 = (char*)(({ if (_pipe28_cap < 1 || !_pipe28) { _pipe28_cap = 64; _pipe28 = (char*)realloc(_pipe28, _pipe28_cap); if (!_pipe28) { exit(127); } } _pipe28[0] = 0; int _pf28[2]; int _ok28 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf28 + 2*_k) != 0) { _ok28 = 0; break; } if (!_ok28) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp28[1]; for (int _t = 0; (_cp28[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp28[0] == 0) { dup2(_pf28[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf28[2*_k]); close(_pf28[2*_k+1]); } execvp(_a28_0[0], _a28_0); _exit(127); } close(_pf28[1]); size_t _gn28 = 0; for (;;) { if (_gn28 + 8192 > _pipe28_cap) { _pipe28_cap = _gn28 + 16384; _pipe28 = (char*)realloc(_pipe28, _pipe28_cap); if (!_pipe28) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf28[0], _pipe28 + _gn28, _pipe28_cap - _gn28 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn28 += (size_t)_r; } close(_pf28[0]); _pipe28[_gn28] = 0; int _st28 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp28[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st28 = _s; } sh_status = WIFEXITED(_st28) ? WEXITSTATUS(_st28) : 1; while (_gn28 > 0 && (_pipe28[_gn28-1] == '\n' || _pipe28[_gn28-1] == '\r')) _pipe28[--_gn28] = 0; } (char*)_pipe28; })); char *_xp12 = _sd12 ? strdup(_sd12) : 0; if (_sd12 && !_xp12) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp12; }); free(_old29); }
    fputs("Common lines:\n", stdout);
    printf("%s\n", comm_result);
    char *_a30_0[] = {"diff", "file1.txt", "file2.txt", 0};
    static char *_pipe30 = 0; static size_t _pipe30_cap = 0;
    { char *_old31 = diff_result; diff_result = ({ const char *_sd13 = (char*)(({ if (_pipe30_cap < 1 || !_pipe30) { _pipe30_cap = 64; _pipe30 = (char*)realloc(_pipe30, _pipe30_cap); if (!_pipe30) { exit(127); } } _pipe30[0] = 0; int _pf30[2]; int _ok30 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf30 + 2*_k) != 0) { _ok30 = 0; break; } if (!_ok30) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp30[1]; for (int _t = 0; (_cp30[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp30[0] == 0) { dup2(_pf30[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf30[2*_k]); close(_pf30[2*_k+1]); } execvp(_a30_0[0], _a30_0); _exit(127); } close(_pf30[1]); size_t _gn30 = 0; for (;;) { if (_gn30 + 8192 > _pipe30_cap) { _pipe30_cap = _gn30 + 16384; _pipe30 = (char*)realloc(_pipe30, _pipe30_cap); if (!_pipe30) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf30[0], _pipe30 + _gn30, _pipe30_cap - _gn30 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn30 += (size_t)_r; } close(_pf30[0]); _pipe30[_gn30] = 0; int _st30 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp30[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st30 = _s; } sh_status = WIFEXITED(_st30) ? WEXITSTATUS(_st30) : 1; while (_gn30 > 0 && (_pipe30[_gn30-1] == '\n' || _pipe30[_gn30-1] == '\r')) _pipe30[--_gn30] = 0; } (char*)_pipe30; })); char *_xp13 = _sd13 ? strdup(_sd13) : 0; if (_sd13 && !_xp13) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp13; }); free(_old31); }
    fputs("File differences:\n", stdout);
    printf("%s\n", diff_result);
    static char *_pipe32 = 0; static size_t _pipe32_cap = 0;
    { char *_old33 = tr_result; tr_result = ({ const char *_sd14 = (char*)(({ if (_pipe32_cap < 1 || !_pipe32) { _pipe32_cap = 64; _pipe32 = (char*)realloc(_pipe32, _pipe32_cap); if (!_pipe32) { exit(127); } } _pipe32[0] = 0; int _pf32[4]; int _ok32 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf32 + 2*_k) != 0) { _ok32 = 0; break; } if (!_ok32) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp32[2]; for (int _t = 0; (_cp32[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp32[0] == 0) { dup2(_pf32[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf32[2*_k]); close(_pf32[2*_k+1]); } int _tr = _stg32_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp32[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp32[1] == 0) { dup2(_pf32[0], 0); dup2(_pf32[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf32[2*_k]); close(_pf32[2*_k+1]); } int _tr = _stg32_1(); fflush(stdout); _exit(_tr); } for (int _k = 0; _k < 1; _k++) { close(_pf32[2*_k]); close(_pf32[2*_k+1]); } close(_pf32[3]); size_t _gn32 = 0; for (;;) { if (_gn32 + 8192 > _pipe32_cap) { _pipe32_cap = _gn32 + 16384; _pipe32 = (char*)realloc(_pipe32, _pipe32_cap); if (!_pipe32) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf32[2], _pipe32 + _gn32, _pipe32_cap - _gn32 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn32 += (size_t)_r; } close(_pf32[2]); _pipe32[_gn32] = 0; int _st32 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp32[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st32 = _s; } sh_status = WIFEXITED(_st32) ? WEXITSTATUS(_st32) : 1; while (_gn32 > 0 && (_pipe32[_gn32-1] == '\n' || _pipe32[_gn32-1] == '\r')) _pipe32[--_gn32] = 0; } (char*)_pipe32; })); char *_xp14 = _sd14 ? strdup(_sd14) : 0; if (_sd14 && !_xp14) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp14; }); free(_old33); }
    printf("Lowercase: %s\n", tr_result);
    char *_a34_1[] = {"xargs", "-n", "1", "echo", "Number:", 0};
    static char *_pipe34 = 0; static size_t _pipe34_cap = 0;
    { char *_old35 = xargs_result; xargs_result = ({ const char *_sd15 = (char*)(({ if (_pipe34_cap < 1 || !_pipe34) { _pipe34_cap = 64; _pipe34 = (char*)realloc(_pipe34, _pipe34_cap); if (!_pipe34) { exit(127); } } _pipe34[0] = 0; int _pf34[4]; int _ok34 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf34 + 2*_k) != 0) { _ok34 = 0; break; } if (!_ok34) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp34[2]; for (int _t = 0; (_cp34[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp34[0] == 0) { dup2(_pf34[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf34[2*_k]); close(_pf34[2*_k+1]); } int _tr = _stg34_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp34[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp34[1] == 0) { dup2(_pf34[0], 0); dup2(_pf34[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf34[2*_k]); close(_pf34[2*_k+1]); } execvp(_a34_1[0], _a34_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf34[2*_k]); close(_pf34[2*_k+1]); } close(_pf34[3]); size_t _gn34 = 0; for (;;) { if (_gn34 + 8192 > _pipe34_cap) { _pipe34_cap = _gn34 + 16384; _pipe34 = (char*)realloc(_pipe34, _pipe34_cap); if (!_pipe34) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf34[2], _pipe34 + _gn34, _pipe34_cap - _gn34 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn34 += (size_t)_r; } close(_pf34[2]); _pipe34[_gn34] = 0; int _st34 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp34[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st34 = _s; } sh_status = WIFEXITED(_st34) ? WEXITSTATUS(_st34) : 1; while (_gn34 > 0 && (_pipe34[_gn34-1] == '\n' || _pipe34[_gn34-1] == '\r')) _pipe34[--_gn34] = 0; } (char*)_pipe34; })); char *_xp15 = _sd15 ? strdup(_sd15) : 0; if (_sd15 && !_xp15) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp15; }); free(_old35); }
    fputs("Xargs result:\n", stdout);
    printf("%s\n", xargs_result), sh_status = 0;
    ({ int _mr = 0; { if (unlink("file1.txt") != 0 && errno != ENOENT) _mr = 1; } { if (unlink("file2.txt") != 0 && errno != ENOENT) _mr = 1; } sh_status = (_mr == 0 ? 0 : 1); (sh_status == 0); });
    free(paste_result);
    free(cut_result);
    free(sed_result);
    free(uniq_result);
    free(file_content);
    free(diff_result);
    free(sort_result);
    free(tail_result);
    free(word_count);
    free(grep_result);
    free(line_count);
    free(awk_result);
    free(comm_result);
    free(head_result);
    free(tr_result);
    free(xargs_result);
    return sh_status;
}