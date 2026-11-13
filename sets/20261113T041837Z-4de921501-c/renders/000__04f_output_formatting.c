#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */

static int _stg1_0(void) {
  fputs("test output\n", stdout);
  return 0;
}

int main(void) {
    int sh_status = 0;
    char* d = NULL;
    char* tee_result = NULL;
    fputs("=== Output and Formatting Commands ===\n", stdout);
    printf("Echo result: %s\n", "Hello from backticks");
    { char *_old0 = d; d = ({ const char *_sd0 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); char *_mk = mkdtemp(_mt); _mk ? strdup(_mk) : strdup(""); })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old0); }
    { if ((({ int _r = chdir((d ? d : "")); sh_status = (_r == 0 ? 0 : 1); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; }))) { sh_status = 0; } else { sh_status = 1;
        exit(1);
      }
    }
    printf("Printf result: %s\n", "Number: 42, String: test");
    char *_a1_1[] = {"tee", "test_tee.txt", 0};
    static char *_pipe1 = 0; static size_t _pipe1_cap = 0;
    { char *_old2 = tee_result; tee_result = ({ const char *_sd1 = (char*)(({ if (_pipe1_cap < 1 || !_pipe1) { _pipe1_cap = 64; _pipe1 = (char*)realloc(_pipe1, _pipe1_cap); if (!_pipe1) { exit(127); } } _pipe1[0] = 0; int _pf1[4]; int _ok1 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf1 + 2*_k) != 0) { _ok1 = 0; break; } if (!_ok1) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp1[2]; for (int _t = 0; (_cp1[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp1[0] == 0) { dup2(_pf1[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf1[2*_k]); close(_pf1[2*_k+1]); } int _tr = _stg1_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp1[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp1[1] == 0) { dup2(_pf1[0], 0); dup2(_pf1[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf1[2*_k]); close(_pf1[2*_k+1]); } execvp(_a1_1[0], _a1_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf1[2*_k]); close(_pf1[2*_k+1]); } close(_pf1[3]); size_t _gn1 = 0; for (;;) { if (_gn1 + 8192 > _pipe1_cap) { _pipe1_cap = _gn1 + 16384; _pipe1 = (char*)realloc(_pipe1, _pipe1_cap); if (!_pipe1) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf1[2], _pipe1 + _gn1, _pipe1_cap - _gn1 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn1 += (size_t)_r; } close(_pf1[2]); _pipe1[_gn1] = 0; int _st1 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp1[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st1 = _s; } sh_status = WIFEXITED(_st1) ? WEXITSTATUS(_st1) : 1; while (_gn1 > 0 && (_pipe1[_gn1-1] == '\n' || _pipe1[_gn1-1] == '\r')) _pipe1[--_gn1] = 0; } (char*)_pipe1; })); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; }); free(_old2); }
    printf("Tee result: %s\n", tee_result), sh_status = 0;
    ({ int _mr = 0; { if (unlink("test_tee.txt") != 0 && errno != ENOENT) _mr = 1; } sh_status = (_mr == 0 ? 0 : 1); (sh_status == 0); });
    fputs("=== Output and Formatting Commands Complete ===\n", stdout);
    ({ int _r = chdir("/"); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; });
    char *_a3[] = {"rm", "-r", "-f", (d ? d : ""), 0};
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a3)[0], (_a3)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    free(tee_result);
    free(d);
    return sh_status;
}