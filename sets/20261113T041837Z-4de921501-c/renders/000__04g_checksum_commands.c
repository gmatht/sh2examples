#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <fcntl.h>

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */

int main(void) {
    int sh_status = 0;
    char* d = NULL;
    char* sha256_result = NULL;
    char* sha512_result = NULL;
    char* strings_result = NULL;
    fputs("=== Checksum Commands ===\n", stdout);
    { char *_old0 = d; d = ({ const char *_sd0 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); char *_mk = mkdtemp(_mt); _mk ? strdup(_mk) : strdup(""); })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old0); }
    { if ((({ int _r = chdir((d ? d : "")); sh_status = (_r == 0 ? 0 : 1); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; }))) { sh_status = 0; } else { sh_status = 1;
        exit(1);
      }
    }
    fflush(stdout);
    int _nfd1_1 = open("test_checksum.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd1_1 = dup(1);
    if (_nfd1_1 >= 0) { dup2(_nfd1_1, 1); close(_nfd1_1); }
    if (_nfd1_1 >= 0) {
        fputs("test content\n", stdout);
    }
    {
    fflush(stdout);
    dup2(_sfd1_1, 1); close(_sfd1_1);
    }
    char *_a2_0[] = {"sha256sum", "test_checksum.txt", 0};
    static char *_pipe2 = 0; static size_t _pipe2_cap = 0;
    { char *_old3 = sha256_result; sha256_result = ({ const char *_sd1 = (char*)(({ if (_pipe2_cap < 1 || !_pipe2) { _pipe2_cap = 64; _pipe2 = (char*)realloc(_pipe2, _pipe2_cap); if (!_pipe2) { exit(127); } } _pipe2[0] = 0; int _pf2[2]; int _ok2 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf2 + 2*_k) != 0) { _ok2 = 0; break; } if (!_ok2) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp2[1]; for (int _t = 0; (_cp2[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp2[0] == 0) { dup2(_pf2[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf2[2*_k]); close(_pf2[2*_k+1]); } execvp(_a2_0[0], _a2_0); _exit(127); } close(_pf2[1]); size_t _gn2 = 0; for (;;) { if (_gn2 + 8192 > _pipe2_cap) { _pipe2_cap = _gn2 + 16384; _pipe2 = (char*)realloc(_pipe2, _pipe2_cap); if (!_pipe2) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf2[0], _pipe2 + _gn2, _pipe2_cap - _gn2 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn2 += (size_t)_r; } close(_pf2[0]); _pipe2[_gn2] = 0; int _st2 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp2[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st2 = _s; } sh_status = WIFEXITED(_st2) ? WEXITSTATUS(_st2) : 1; while (_gn2 > 0 && (_pipe2[_gn2-1] == '\n' || _pipe2[_gn2-1] == '\r')) _pipe2[--_gn2] = 0; } (char*)_pipe2; })); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; }); free(_old3); }
    printf("SHA256 result: %s\n", sha256_result);
    char *_a4_0[] = {"sha512sum", "test_checksum.txt", 0};
    static char *_pipe4 = 0; static size_t _pipe4_cap = 0;
    { char *_old5 = sha512_result; sha512_result = ({ const char *_sd2 = (char*)(({ if (_pipe4_cap < 1 || !_pipe4) { _pipe4_cap = 64; _pipe4 = (char*)realloc(_pipe4, _pipe4_cap); if (!_pipe4) { exit(127); } } _pipe4[0] = 0; int _pf4[2]; int _ok4 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf4 + 2*_k) != 0) { _ok4 = 0; break; } if (!_ok4) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp4[1]; for (int _t = 0; (_cp4[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp4[0] == 0) { dup2(_pf4[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf4[2*_k]); close(_pf4[2*_k+1]); } execvp(_a4_0[0], _a4_0); _exit(127); } close(_pf4[1]); size_t _gn4 = 0; for (;;) { if (_gn4 + 8192 > _pipe4_cap) { _pipe4_cap = _gn4 + 16384; _pipe4 = (char*)realloc(_pipe4, _pipe4_cap); if (!_pipe4) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf4[0], _pipe4 + _gn4, _pipe4_cap - _gn4 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn4 += (size_t)_r; } close(_pf4[0]); _pipe4[_gn4] = 0; int _st4 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp4[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st4 = _s; } sh_status = WIFEXITED(_st4) ? WEXITSTATUS(_st4) : 1; while (_gn4 > 0 && (_pipe4[_gn4-1] == '\n' || _pipe4[_gn4-1] == '\r')) _pipe4[--_gn4] = 0; } (char*)_pipe4; })); char *_xp2 = _sd2 ? strdup(_sd2) : 0; if (_sd2 && !_xp2) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp2; }); free(_old5); }
    printf("SHA512 result: %s\n", sha512_result);
    char *_a6_0[] = {"strings", "target/debug/debashc.exe", 0};
    char *_a6_1[] = {"head", "-3", 0};
    static char *_pipe6 = 0; static size_t _pipe6_cap = 0;
    { char *_old7 = strings_result; strings_result = ({ const char *_sd3 = (char*)(({ if (_pipe6_cap < 1 || !_pipe6) { _pipe6_cap = 64; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { exit(127); } } _pipe6[0] = 0; int _pf6[4]; int _ok6 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf6 + 2*_k) != 0) { _ok6 = 0; break; } if (!_ok6) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp6[2]; for (int _t = 0; (_cp6[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[0] == 0) { dup2(_pf6[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_0[0], _a6_0); _exit(127); } for (int _t = 0; (_cp6[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[1] == 0) { dup2(_pf6[0], 0); dup2(_pf6[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_1[0], _a6_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } close(_pf6[3]); size_t _gn6 = 0; for (;;) { if (_gn6 + 8192 > _pipe6_cap) { _pipe6_cap = _gn6 + 16384; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf6[2], _pipe6 + _gn6, _pipe6_cap - _gn6 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn6 += (size_t)_r; } close(_pf6[2]); _pipe6[_gn6] = 0; int _st6 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp6[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st6 = _s; } sh_status = WIFEXITED(_st6) ? WEXITSTATUS(_st6) : 1; while (_gn6 > 0 && (_pipe6[_gn6-1] == '\n' || _pipe6[_gn6-1] == '\r')) _pipe6[--_gn6] = 0; } (char*)_pipe6; })); char *_xp3 = _sd3 ? strdup(_sd3) : 0; if (_sd3 && !_xp3) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp3; }); free(_old7); }
    fputs("Strings result:\n", stdout);
    printf("%s\n", strings_result), sh_status = 0;
    ({ int _mr = 0; { if (unlink("test_checksum.txt") != 0 && errno != ENOENT) _mr = 1; } sh_status = (_mr == 0 ? 0 : 1); (sh_status == 0); });
    fputs("=== Checksum Commands Complete ===\n", stdout);
    ({ int _r = chdir("/"); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; });
    char *_a8[] = {"rm", "-r", "-f", (d ? d : ""), 0};
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a8)[0], (_a8)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    free(d);
    free(sha512_result);
    free(sha256_result);
    free(strings_result);
    return sh_status;
}