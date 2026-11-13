#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <fcntl.h>


static int _stg7_1(void) {
  static const unsigned char _trp7_1[][2] = {{97,98}};
  char *_tl = 0; size_t _tlc = 0; long _tn;
  while ((_tn = getline(&_tl, &_tlc, stdin)) > 0) { char *_te = _tl + _tn; for (char *_p = _tl; _p < _te; _p++) for (size_t _k = 0; _k < sizeof _trp7_1 / sizeof _trp7_1[0]; _k++) if (_trp7_1[_k][0] == (unsigned char)*_p) { *_p = (char)_trp7_1[_k][1]; break; } fwrite(_tl, 1, (size_t)_tn, stdout); }
  free(_tl); return 0;
}

int main(void) {
    int sh_status = 0;
    char* d = NULL;
    { char *_old0 = d; d = ({ const char *_sd0 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); char *_mk = mkdtemp(_mt); _mk ? strdup(_mk) : strdup(""); })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old0); }
    { if ((({ int _r = chdir((d ? d : "")); sh_status = (_r == 0 ? 0 : 1); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; }))) { sh_status = 0; } else { sh_status = 1;
        exit(1);
      }
    }
    fflush(stdout);
    int _nfd1_1 = open("file.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd1_1 = dup(1);
    if (_nfd1_1 >= 0) { dup2(_nfd1_1, 1); close(_nfd1_1); }
    if (_nfd1_1 >= 0) {
        (sh_status = 0, fputs("apple\napple\n", stdout));
    }
    {
    fflush(stdout);
    dup2(_sfd1_1, 1); close(_sfd1_1);
    }
    fflush(stdout);
    int _nfd2_1 = open("note.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd2_1 = dup(1);
    if (_nfd2_1 >= 0) { dup2(_nfd2_1, 1); close(_nfd2_1); }
    if (_nfd2_1 >= 0) {
        (sh_status = 0, fputs("hello.txt\n", stdout));
    }
    {
    fflush(stdout);
    dup2(_sfd2_1, 1); close(_sfd2_1);
    }
    fflush(stdout);
    int _nfd3_1 = open("s.sh", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd3_1 = dup(1);
    if (_nfd3_1 >= 0) { dup2(_nfd3_1, 1); close(_nfd3_1); }
    if (_nfd3_1 >= 0) {
        (sh_status = 0, fputs("function f() { echo hi; }\n", stdout));
    }
    {
    fflush(stdout);
    dup2(_sfd3_1, 1); close(_sfd3_1);
    }
    char *_a4_0[] = {"ls", 0};
    char *_a4_1[] = {"grep", "\\.txt$", 0};
    char *_a4_2[] = {"wc", "-l", 0};
    ({ int _pf4[4]; int _ok4 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf4 + 2*_k) != 0) { _ok4 = 0; break; } if (!_ok4) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp4[3]; for (int _t = 0; (_cp4[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp4[0] == 0) { dup2(_pf4[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf4[2*_k]); close(_pf4[2*_k+1]); } execvp(_a4_0[0], _a4_0); _exit(127); } for (int _t = 0; (_cp4[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp4[1] == 0) { dup2(_pf4[0], 0); dup2(_pf4[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf4[2*_k]); close(_pf4[2*_k+1]); } execvp(_a4_1[0], _a4_1); _exit(127); } for (int _t = 0; (_cp4[2] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp4[2] == 0) { dup2(_pf4[2], 0); for (int _k = 0; _k < 2; _k++) { close(_pf4[2*_k]); close(_pf4[2*_k+1]); } execvp(_a4_2[0], _a4_2); _exit(127); } for (int _k = 0; _k < 2; _k++) { close(_pf4[2*_k]); close(_pf4[2*_k+1]); } int _st4 = 0; for (int _k = 0; _k < 3; _k++) { int _s = 0; while (waitpid(_cp4[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 2) _st4 = _s; } sh_status = WIFEXITED(_st4) ? WEXITSTATUS(_st4) : 1; } (sh_status == 0); });
    fputs("\n", stdout), sh_status = 0;
    char *_a5_0[] = {"cat", "file.txt", 0};
    char *_a5_1[] = {"sort", 0};
    char *_a5_2[] = {"uniq", "-c", 0};
    char *_a5_3[] = {"sort", "-n", "-r", 0};
    ({ int _pf5[6]; int _ok5 = 1; for (int _k = 0; _k < 3; _k++) if (pipe(_pf5 + 2*_k) != 0) { _ok5 = 0; break; } if (!_ok5) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp5[4]; for (int _t = 0; (_cp5[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp5[0] == 0) { dup2(_pf5[1], 1); for (int _k = 0; _k < 3; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } execvp(_a5_0[0], _a5_0); _exit(127); } for (int _t = 0; (_cp5[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp5[1] == 0) { dup2(_pf5[0], 0); dup2(_pf5[3], 1); for (int _k = 0; _k < 3; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } execvp(_a5_1[0], _a5_1); _exit(127); } for (int _t = 0; (_cp5[2] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp5[2] == 0) { dup2(_pf5[2], 0); dup2(_pf5[5], 1); for (int _k = 0; _k < 3; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } execvp(_a5_2[0], _a5_2); _exit(127); } for (int _t = 0; (_cp5[3] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp5[3] == 0) { dup2(_pf5[4], 0); for (int _k = 0; _k < 3; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } execvp(_a5_3[0], _a5_3); _exit(127); } for (int _k = 0; _k < 3; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } int _st5 = 0; for (int _k = 0; _k < 4; _k++) { int _s = 0; while (waitpid(_cp5[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 3) _st5 = _s; } sh_status = WIFEXITED(_st5) ? WEXITSTATUS(_st5) : 1; } (sh_status == 0); });
    fputs("\n", stdout), sh_status = 0;
    char *_a6_0[] = {"find", ".", "-name", "*.sh", 0};
    char *_a6_1[] = {"xargs", "grep", "-l", "function", 0};
    char *_a6_2[] = {"tr", "-d", "\\\\/", 0};
    ({ int _pf6[4]; int _ok6 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf6 + 2*_k) != 0) { _ok6 = 0; break; } if (!_ok6) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp6[3]; for (int _t = 0; (_cp6[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[0] == 0) { dup2(_pf6[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_0[0], _a6_0); _exit(127); } for (int _t = 0; (_cp6[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[1] == 0) { dup2(_pf6[0], 0); dup2(_pf6[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_1[0], _a6_1); _exit(127); } for (int _t = 0; (_cp6[2] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[2] == 0) { dup2(_pf6[2], 0); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_2[0], _a6_2); _exit(127); } for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } int _st6 = 0; for (int _k = 0; _k < 3; _k++) { int _s = 0; while (waitpid(_cp6[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 2) _st6 = _s; } sh_status = WIFEXITED(_st6) ? WEXITSTATUS(_st6) : 1; } (sh_status == 0); });
    fputs("\n", stdout), sh_status = 0;
    char *_a7_0[] = {"cat", "file.txt", 0};
    char *_a7_2[] = {"grep", "hello", 0};
    ({ int _pf7[4]; int _ok7 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf7 + 2*_k) != 0) { _ok7 = 0; break; } if (!_ok7) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp7[3]; for (int _t = 0; (_cp7[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp7[0] == 0) { dup2(_pf7[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf7[2*_k]); close(_pf7[2*_k+1]); } execvp(_a7_0[0], _a7_0); _exit(127); } for (int _t = 0; (_cp7[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp7[1] == 0) { dup2(_pf7[0], 0); dup2(_pf7[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf7[2*_k]); close(_pf7[2*_k+1]); } int _tr = _stg7_1(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp7[2] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp7[2] == 0) { dup2(_pf7[2], 0); for (int _k = 0; _k < 2; _k++) { close(_pf7[2*_k]); close(_pf7[2*_k+1]); } execvp(_a7_2[0], _a7_2); _exit(127); } for (int _k = 0; _k < 2; _k++) { close(_pf7[2*_k]); close(_pf7[2*_k+1]); } int _st7 = 0; for (int _k = 0; _k < 3; _k++) { int _s = 0; while (waitpid(_cp7[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 2) _st7 = _s; } sh_status = WIFEXITED(_st7) ? WEXITSTATUS(_st7) : 1; } (sh_status == 0); });
    fputs("\n", stdout), sh_status = 0;
    char *_a8_0[] = {"cat", "file.txt", 0};
    char *_a8_1[] = {"sort", 0};
    char *_a8_2[] = {"grep", "hello", 0};
    ({ int _pf8[4]; int _ok8 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf8 + 2*_k) != 0) { _ok8 = 0; break; } if (!_ok8) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp8[3]; for (int _t = 0; (_cp8[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp8[0] == 0) { dup2(_pf8[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } execvp(_a8_0[0], _a8_0); _exit(127); } for (int _t = 0; (_cp8[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp8[1] == 0) { dup2(_pf8[0], 0); dup2(_pf8[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } execvp(_a8_1[0], _a8_1); _exit(127); } for (int _t = 0; (_cp8[2] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp8[2] == 0) { dup2(_pf8[2], 0); for (int _k = 0; _k < 2; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } execvp(_a8_2[0], _a8_2); _exit(127); } for (int _k = 0; _k < 2; _k++) { close(_pf8[2*_k]); close(_pf8[2*_k+1]); } int _st8 = 0; for (int _k = 0; _k < 3; _k++) { int _s = 0; while (waitpid(_cp8[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 2) _st8 = _s; } sh_status = WIFEXITED(_st8) ? WEXITSTATUS(_st8) : 1; } (sh_status == 0); });
    ({ int _r = chdir("/"); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; });
    char *_a9[] = {"rm", "-r", "-f", (d ? d : ""), 0};
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a9)[0], (_a9)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    free(d);
    return sh_status;
}