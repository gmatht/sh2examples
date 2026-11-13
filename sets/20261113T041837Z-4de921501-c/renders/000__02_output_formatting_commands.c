#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <fcntl.h>

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */

static int _stg8_0(void) {
  fputs("test output\n", stdout);
  return 0;
}

int main(void) {
    int sh_status = 0;
    char* d = NULL;
    char* perl_result = NULL;
    char* sha256_result = NULL;
    char* sha512_result = NULL;
    char* strings_result = NULL;
    char* tee_result = NULL;
    fputs("=== Output and Formatting Commands ===\n", stdout);
    printf("Echo result: %s\n", "Hello from backticks");
    printf("Printf result: %s\n", "Number: 42, String: test");
    fputs("=== Compression Commands ===\n", stdout);
    fputs("=== Network Commands ===\n", stdout);
    fputs("=== Process Management Commands ===\n", stdout);
    { char *_old0 = d; d = ({ const char *_sd0 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); char *_mk = mkdtemp(_mt); _mk ? strdup(_mk) : strdup(""); })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old0); }
    { if ((({ int _r = chdir((d ? d : "")); sh_status = (_r == 0 ? 0 : 1); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; }))) { sh_status = 0; } else { sh_status = 1;
        exit(1);
      }
    }
    fputs("=== Checksum Commands ===\n", stdout);
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
    char *_a6_0[] = {"strings", "test_binary.txt", 0};
    char *_a6_1[] = {"head", "-3", 0};
    static char *_pipe6 = 0; static size_t _pipe6_cap = 0;
    { char *_old7 = strings_result; strings_result = ({ const char *_sd3 = (char*)(({ if (_pipe6_cap < 1 || !_pipe6) { _pipe6_cap = 64; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { exit(127); } } _pipe6[0] = 0; int _pf6[4]; int _ok6 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf6 + 2*_k) != 0) { _ok6 = 0; break; } if (!_ok6) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp6[2]; for (int _t = 0; (_cp6[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[0] == 0) { dup2(_pf6[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_0[0], _a6_0); _exit(127); } for (int _t = 0; (_cp6[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[1] == 0) { dup2(_pf6[0], 0); dup2(_pf6[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_1[0], _a6_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } close(_pf6[3]); size_t _gn6 = 0; for (;;) { if (_gn6 + 8192 > _pipe6_cap) { _pipe6_cap = _gn6 + 16384; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf6[2], _pipe6 + _gn6, _pipe6_cap - _gn6 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn6 += (size_t)_r; } close(_pf6[2]); _pipe6[_gn6] = 0; int _st6 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp6[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st6 = _s; } sh_status = WIFEXITED(_st6) ? WEXITSTATUS(_st6) : 1; while (_gn6 > 0 && (_pipe6[_gn6-1] == '\n' || _pipe6[_gn6-1] == '\r')) _pipe6[--_gn6] = 0; } (char*)_pipe6; })); char *_xp3 = _sd3 ? strdup(_sd3) : 0; if (_sd3 && !_xp3) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp3; }); free(_old7); }
    fputs("Strings result:\n", stdout);
    printf("%s\n", strings_result);
    fputs("=== I/O Redirection Commands ===\n", stdout);
    char *_a8_1[] = {"tee", "test_tee.txt", 0};
    static char *_pipe8 = 0; static size_t _pipe8_cap = 0;
    { char *_old9 = tee_result; tee_result = ({ const char *_sd4 = (char*)(({ if (_pipe8_cap < 1 || !_pipe8) { _pipe8_cap = 64; _pipe8 = (char*)realloc(_pipe8, _pipe8_cap); if (!_pipe8) { exit(127); } } _pipe8[0] = 0; int _pf8[4]; int _ok8 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf8 + 2*_k) != 0) { _ok8 = 0; break; } if (!_ok8) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp8[2]; for (int _t = 0; (_cp8[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp8[0] == 0) { dup2(_pf8[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } int _tr = _stg8_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp8[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp8[1] == 0) { dup2(_pf8[0], 0); dup2(_pf8[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } execvp(_a8_1[0], _a8_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } close(_pf8[3]); size_t _gn8 = 0; for (;;) { if (_gn8 + 8192 > _pipe8_cap) { _pipe8_cap = _gn8 + 16384; _pipe8 = (char*)realloc(_pipe8, _pipe8_cap); if (!_pipe8) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf8[2], _pipe8 + _gn8, _pipe8_cap - _gn8 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn8 += (size_t)_r; } close(_pf8[2]); _pipe8[_gn8] = 0; int _st8 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp8[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st8 = _s; } sh_status = WIFEXITED(_st8) ? WEXITSTATUS(_st8) : 1; while (_gn8 > 0 && (_pipe8[_gn8-1] == '\n' || _pipe8[_gn8-1] == '\r')) _pipe8[--_gn8] = 0; } (char*)_pipe8; })); char *_xp4 = _sd4 ? strdup(_sd4) : 0; if (_sd4 && !_xp4) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp4; }); free(_old9); }
    printf("Tee result: %s\n", tee_result);
    fputs("=== Perl Command ===\n", stdout);
    char *_a10_0[] = {"perl", "-e", "print \"Hello from Perl\\n\"", 0};
    static char *_pipe10 = 0; static size_t _pipe10_cap = 0;
    { char *_old11 = perl_result; perl_result = ({ const char *_sd5 = (char*)(({ if (_pipe10_cap < 1 || !_pipe10) { _pipe10_cap = 64; _pipe10 = (char*)realloc(_pipe10, _pipe10_cap); if (!_pipe10) { exit(127); } } _pipe10[0] = 0; int _pf10[2]; int _ok10 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf10 + 2*_k) != 0) { _ok10 = 0; break; } if (!_ok10) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp10[1]; for (int _t = 0; (_cp10[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp10[0] == 0) { dup2(_pf10[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf10[2*_k]); close(_pf10[2*_k+1]); } execvp(_a10_0[0], _a10_0); _exit(127); } close(_pf10[1]); size_t _gn10 = 0; for (;;) { if (_gn10 + 8192 > _pipe10_cap) { _pipe10_cap = _gn10 + 16384; _pipe10 = (char*)realloc(_pipe10, _pipe10_cap); if (!_pipe10) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf10[0], _pipe10 + _gn10, _pipe10_cap - _gn10 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn10 += (size_t)_r; } close(_pf10[0]); _pipe10[_gn10] = 0; int _st10 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp10[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st10 = _s; } sh_status = WIFEXITED(_st10) ? WEXITSTATUS(_st10) : 1; while (_gn10 > 0 && (_pipe10[_gn10-1] == '\n' || _pipe10[_gn10-1] == '\r')) _pipe10[--_gn10] = 0; } (char*)_pipe10; })); char *_xp5 = _sd5 ? strdup(_sd5) : 0; if (_sd5 && !_xp5) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp5; }); free(_old11); }
    printf("Perl result: %s\n", perl_result), sh_status = 0;
    ({ int _mr = 0; { if (unlink("test_checksum.txt") != 0 && errno != ENOENT) _mr = 1; } { if (unlink("test_tee.txt") != 0 && errno != ENOENT) _mr = 1; } sh_status = (_mr == 0 ? 0 : 1); (sh_status == 0); });
    ({ int _r = chdir("/"); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; });
    char *_a12[] = {"rm", "-r", "-f", (d ? d : ""), 0};
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a12)[0], (_a12)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    free(sha512_result);
    free(d);
    free(strings_result);
    free(perl_result);
    free(tee_result);
    free(sha256_result);
    return sh_status;
}