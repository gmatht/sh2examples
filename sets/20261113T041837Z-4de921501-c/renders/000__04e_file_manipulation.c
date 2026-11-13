#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <fcntl.h>

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */


static char *_cap_0(int *rc) {
  static char *buf = 0; static size_t buf_cap = 0;
/* capture: bash -c */ { char *_ca0[] = {"bash", "-c", (char*)" 'cp' 'test_file.txt' 'test_file_copy.txt' && 'echo' 'Copy successful'", 0}; ({ if (buf_cap < 1 || !buf) { buf_cap = 64; buf = (char*)realloc(buf, buf_cap); if (!buf) { exit(127); } } buf[0] = 0; int _pf0[2]; int _ok0 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf0 + 2*_k) != 0) { _ok0 = 0; break; } if (!_ok0) { (*rc) = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp0[1]; for (int _t = 0; (_cp0[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp0[0] == 0) { dup2(_pf0[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } execvp(_ca0[0], _ca0); _exit(127); } close(_pf0[1]); size_t _gn0 = 0; for (;;) { if (_gn0 + 8192 > buf_cap) { buf_cap = _gn0 + 16384; buf = (char*)realloc(buf, buf_cap); if (!buf) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf0[0], buf + _gn0, buf_cap - _gn0 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn0 += (size_t)_r; } close(_pf0[0]); buf[_gn0] = 0; int _st0 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp0[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st0 = _s; } (*rc) = WIFEXITED(_st0) ? WEXITSTATUS(_st0) : 1; while (_gn0 > 0 && (buf[_gn0-1] == '\n' || buf[_gn0-1] == '\r')) buf[--_gn0] = 0; } (char*)buf; }); }
return buf;
}

static char *_cap_1(int *rc) {
  static char *buf = 0; static size_t buf_cap = 0;
/* capture: bash -c */ { char *_ca1[] = {"bash", "-c", (char*)" 'mv' 'test_file_copy.txt' 'test_file_moved.txt' && 'echo' 'Move successful'", 0}; ({ if (buf_cap < 1 || !buf) { buf_cap = 64; buf = (char*)realloc(buf, buf_cap); if (!buf) { exit(127); } } buf[0] = 0; int _pf1[2]; int _ok1 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf1 + 2*_k) != 0) { _ok1 = 0; break; } if (!_ok1) { (*rc) = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp1[1]; for (int _t = 0; (_cp1[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp1[0] == 0) { dup2(_pf1[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf1[2*_k]); close(_pf1[2*_k+1]); } execvp(_ca1[0], _ca1); _exit(127); } close(_pf1[1]); size_t _gn1 = 0; for (;;) { if (_gn1 + 8192 > buf_cap) { buf_cap = _gn1 + 16384; buf = (char*)realloc(buf, buf_cap); if (!buf) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf1[0], buf + _gn1, buf_cap - _gn1 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn1 += (size_t)_r; } close(_pf1[0]); buf[_gn1] = 0; int _st1 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp1[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st1 = _s; } (*rc) = WIFEXITED(_st1) ? WEXITSTATUS(_st1) : 1; while (_gn1 > 0 && (buf[_gn1-1] == '\n' || buf[_gn1-1] == '\r')) buf[--_gn1] = 0; } (char*)buf; }); }
return buf;
}

static char *_cap_2(int *rc) {
  static char *buf = 0; static size_t buf_cap = 0;
/* capture: bash -c */ { char *_ca2[] = {"bash", "-c", (char*)" 'rm' 'test_file.txt' 'test_file_moved.txt' && 'echo' 'Remove successful'", 0}; ({ if (buf_cap < 1 || !buf) { buf_cap = 64; buf = (char*)realloc(buf, buf_cap); if (!buf) { exit(127); } } buf[0] = 0; int _pf2[2]; int _ok2 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf2 + 2*_k) != 0) { _ok2 = 0; break; } if (!_ok2) { (*rc) = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp2[1]; for (int _t = 0; (_cp2[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp2[0] == 0) { dup2(_pf2[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf2[2*_k]); close(_pf2[2*_k+1]); } execvp(_ca2[0], _ca2); _exit(127); } close(_pf2[1]); size_t _gn2 = 0; for (;;) { if (_gn2 + 8192 > buf_cap) { buf_cap = _gn2 + 16384; buf = (char*)realloc(buf, buf_cap); if (!buf) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf2[0], buf + _gn2, buf_cap - _gn2 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn2 += (size_t)_r; } close(_pf2[0]); buf[_gn2] = 0; int _st2 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp2[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st2 = _s; } (*rc) = WIFEXITED(_st2) ? WEXITSTATUS(_st2) : 1; while (_gn2 > 0 && (buf[_gn2-1] == '\n' || buf[_gn2-1] == '\r')) buf[--_gn2] = 0; } (char*)buf; }); }
return buf;
}

static char *_cap_3(int *rc) {
  static char *buf = 0; static size_t buf_cap = 0;
/* capture: bash -c */ { char *_ca3[] = {"bash", "-c", (char*)" 'mkdir' 'test_dir' && 'echo' 'Directory created'", 0}; ({ if (buf_cap < 1 || !buf) { buf_cap = 64; buf = (char*)realloc(buf, buf_cap); if (!buf) { exit(127); } } buf[0] = 0; int _pf3[2]; int _ok3 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf3 + 2*_k) != 0) { _ok3 = 0; break; } if (!_ok3) { (*rc) = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp3[1]; for (int _t = 0; (_cp3[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp3[0] == 0) { dup2(_pf3[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf3[2*_k]); close(_pf3[2*_k+1]); } execvp(_ca3[0], _ca3); _exit(127); } close(_pf3[1]); size_t _gn3 = 0; for (;;) { if (_gn3 + 8192 > buf_cap) { buf_cap = _gn3 + 16384; buf = (char*)realloc(buf, buf_cap); if (!buf) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf3[0], buf + _gn3, buf_cap - _gn3 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn3 += (size_t)_r; } close(_pf3[0]); buf[_gn3] = 0; int _st3 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp3[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st3 = _s; } (*rc) = WIFEXITED(_st3) ? WEXITSTATUS(_st3) : 1; while (_gn3 > 0 && (buf[_gn3-1] == '\n' || buf[_gn3-1] == '\r')) buf[--_gn3] = 0; } (char*)buf; }); }
return buf;
}

static char *_cap_4(int *rc) {
  static char *buf = 0; static size_t buf_cap = 0;
/* capture: bash -c */ { char *_ca4[] = {"bash", "-c", (char*)" 'touch' 'test_file.txt' && 'echo' 'File touched'", 0}; ({ if (buf_cap < 1 || !buf) { buf_cap = 64; buf = (char*)realloc(buf, buf_cap); if (!buf) { exit(127); } } buf[0] = 0; int _pf4[2]; int _ok4 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf4 + 2*_k) != 0) { _ok4 = 0; break; } if (!_ok4) { (*rc) = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp4[1]; for (int _t = 0; (_cp4[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp4[0] == 0) { dup2(_pf4[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf4[2*_k]); close(_pf4[2*_k+1]); } execvp(_ca4[0], _ca4); _exit(127); } close(_pf4[1]); size_t _gn4 = 0; for (;;) { if (_gn4 + 8192 > buf_cap) { buf_cap = _gn4 + 16384; buf = (char*)realloc(buf, buf_cap); if (!buf) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf4[0], buf + _gn4, buf_cap - _gn4 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn4 += (size_t)_r; } close(_pf4[0]); buf[_gn4] = 0; int _st4 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp4[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st4 = _s; } (*rc) = WIFEXITED(_st4) ? WEXITSTATUS(_st4) : 1; while (_gn4 > 0 && (buf[_gn4-1] == '\n' || buf[_gn4-1] == '\r')) buf[--_gn4] = 0; } (char*)buf; }); }
return buf;
}

int main(void) {
    int sh_status = 0;
    char* cp_result = NULL;
    char* d = NULL;
    char* mkdir_result = NULL;
    char* mv_result = NULL;
    char* rm_result = NULL;
    char* touch_result = NULL;
    fputs("=== File Manipulation Commands ===\n", stdout);
    { char *_old0 = d; d = ({ const char *_sd0 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); char *_mk = mkdtemp(_mt); _mk ? strdup(_mk) : strdup(""); })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old0); }
    { if ((({ int _r = chdir((d ? d : "")); sh_status = (_r == 0 ? 0 : 1); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; }))) { sh_status = 0; } else { sh_status = 1;
        exit(1);
      }
    }
    fflush(stdout);
    int _nfd1_1 = open("test_file.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd1_1 = dup(1);
    if (_nfd1_1 >= 0) { dup2(_nfd1_1, 1); close(_nfd1_1); }
    if (_nfd1_1 >= 0) {
        fputs("test content\n", stdout), sh_status = 0;
    }
    {
    fflush(stdout);
    dup2(_sfd1_1, 1); close(_sfd1_1);
    }
    { char *_old2 = cp_result; cp_result = ({ const char *_sd1 = (char*)(_cap_0(&sh_status)); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; }); free(_old2); }
    printf("Copy result: %s\n", cp_result), sh_status = 0;
    (({ int _nfd3_0 = open("/dev/null", O_WRONLY|O_CREAT|O_TRUNC, 0666); int _sfd3_0 = dup(2); if (_nfd3_0 >= 0) { dup2(_nfd3_0, 2); close(_nfd3_0); } char *_a3[] = {"ls", "test_file.txt", "test_file_copy.txt", "test_file_moved.txt", 0}; sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a3)[0], (_a3)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; }); if (_sfd3_0 >= 0) { dup2(_sfd3_0, 2); close(_sfd3_0); } (sh_status == 0); }) || (fputs("No test files found\n", stdout), sh_status = 0, 1));
    { char *_old4 = mv_result; mv_result = ({ const char *_sd2 = (char*)(_cap_1(&sh_status)); char *_xp2 = _sd2 ? strdup(_sd2) : 0; if (_sd2 && !_xp2) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp2; }); free(_old4); }
    printf("Move result: %s\n", mv_result), sh_status = 0;
    (({ int _nfd5_0 = open("/dev/null", O_WRONLY|O_CREAT|O_TRUNC, 0666); int _sfd5_0 = dup(2); if (_nfd5_0 >= 0) { dup2(_nfd5_0, 2); close(_nfd5_0); } char *_a5[] = {"ls", "test_file.txt", "test_file_copy.txt", "test_file_moved.txt", 0}; sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep3; for (int _t = 0; (_ep3 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es3 = 127; if (_ep3 == 0) { execvp((_a5)[0], (_a5)); _exit(127); } else if (_ep3 > 0) { while (waitpid(_ep3, &_es3, 0) < 0 && errno == EINTR) { } _es3 = WIFEXITED(_es3) ? WEXITSTATUS(_es3) : 1; } _es3; }); if (_sfd5_0 >= 0) { dup2(_sfd5_0, 2); close(_sfd5_0); } (sh_status == 0); }) || (fputs("No test files found\n", stdout), sh_status = 0, 1));
    { char *_old6 = rm_result; rm_result = ({ const char *_sd3 = (char*)(_cap_2(&sh_status)); char *_xp3 = _sd3 ? strdup(_sd3) : 0; if (_sd3 && !_xp3) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp3; }); free(_old6); }
    printf("Remove result: %s\n", rm_result), sh_status = 0;
    (({ int _nfd7_0 = open("/dev/null", O_WRONLY|O_CREAT|O_TRUNC, 0666); int _sfd7_0 = dup(2); if (_nfd7_0 >= 0) { dup2(_nfd7_0, 2); close(_nfd7_0); } char *_a7[] = {"ls", "test_file.txt", "test_file_copy.txt", "test_file_moved.txt", 0}; sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep4; for (int _t = 0; (_ep4 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es4 = 127; if (_ep4 == 0) { execvp((_a7)[0], (_a7)); _exit(127); } else if (_ep4 > 0) { while (waitpid(_ep4, &_es4, 0) < 0 && errno == EINTR) { } _es4 = WIFEXITED(_es4) ? WEXITSTATUS(_es4) : 1; } _es4; }); if (_sfd7_0 >= 0) { dup2(_sfd7_0, 2); close(_sfd7_0); } (sh_status == 0); }) || (fputs("No test files found\n", stdout), sh_status = 0, 1));
    { char *_old8 = mkdir_result; mkdir_result = ({ const char *_sd4 = (char*)(_cap_3(&sh_status)); char *_xp4 = _sd4 ? strdup(_sd4) : 0; if (_sd4 && !_xp4) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp4; }); free(_old8); }
    printf("Mkdir result: %s\n", mkdir_result), sh_status = 0;
    ({ { FILE *_tf = fopen("test_dir/file", "a"); if (_tf) fclose(_tf); } sh_status = 0; (sh_status == 0); });
    (({ int _nfd9_0 = open("/dev/null", O_WRONLY|O_CREAT|O_TRUNC, 0666); int _sfd9_0 = dup(2); if (_nfd9_0 >= 0) { dup2(_nfd9_0, 2); close(_nfd9_0); } char *_a9[] = {"ls", "test_dir", 0}; sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep5; for (int _t = 0; (_ep5 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es5 = 127; if (_ep5 == 0) { execvp((_a9)[0], (_a9)); _exit(127); } else if (_ep5 > 0) { while (waitpid(_ep5, &_es5, 0) < 0 && errno == EINTR) { } _es5 = WIFEXITED(_es5) ? WEXITSTATUS(_es5) : 1; } _es5; }); if (_sfd9_0 >= 0) { dup2(_sfd9_0, 2); close(_sfd9_0); } (sh_status == 0); }) || (fputs("Directory not found\n", stdout), sh_status = 0, 1));
    { char *_old10 = touch_result; touch_result = ({ const char *_sd5 = (char*)(_cap_4(&sh_status)); char *_xp5 = _sd5 ? strdup(_sd5) : 0; if (_sd5 && !_xp5) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp5; }); free(_old10); }
    printf("Touch result: %s\n", touch_result), sh_status = 0;
    ({ int _mr = 0; { if (unlink("test_file.txt") != 0 && errno != ENOENT) _mr = 1; } { if (unlink("test_file_copy.txt") != 0 && errno != ENOENT) _mr = 1; } { if (unlink("test_file_moved.txt") != 0 && errno != ENOENT) _mr = 1; } sh_status = (_mr == 0 ? 0 : 1); (sh_status == 0); });
    (({ int _nfd11_0 = open("/dev/null", O_WRONLY|O_CREAT|O_TRUNC, 0666); int _sfd11_0 = dup(2); if (_nfd11_0 >= 0) { dup2(_nfd11_0, 2); close(_nfd11_0); } char *_a11[] = {"rm", "-r", "-f", "test_dir", 0}; sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep6; for (int _t = 0; (_ep6 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es6 = 127; if (_ep6 == 0) { execvp((_a11)[0], (_a11)); _exit(127); } else if (_ep6 > 0) { while (waitpid(_ep6, &_es6, 0) < 0 && errno == EINTR) { } _es6 = WIFEXITED(_es6) ? WEXITSTATUS(_es6) : 1; } _es6; }); if (_sfd11_0 >= 0) { dup2(_sfd11_0, 2); close(_sfd11_0); } (sh_status == 0); }) || (sh_status = 0, 1));
    fputs("=== File Manipulation Commands Complete ===\n", stdout);
    ({ int _r = chdir("/"); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; });
    char *_a12[] = {"rm", "-r", "-f", (d ? d : ""), 0};
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep7; for (int _t = 0; (_ep7 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es7 = 127; if (_ep7 == 0) { execvp((_a12)[0], (_a12)); _exit(127); } else if (_ep7 > 0) { while (waitpid(_ep7, &_es7, 0) < 0 && errno == EINTR) { } _es7 = WIFEXITED(_es7) ? WEXITSTATUS(_es7) : 1; } _es7; });
    free(mkdir_result);
    free(rm_result);
    free(mv_result);
    free(touch_result);
    free(d);
    free(cp_result);
    return sh_status;
}