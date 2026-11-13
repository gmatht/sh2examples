#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <fcntl.h>
#include <sys/stat.h>

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */

int main(void) {
    int sh_status = 0;
    char* d = NULL;
    char* file_list = NULL;
    char* found_files = NULL;
    fputs("=== File and Directory Operations ===\n", stdout);
    { char *_old0 = d; d = ({ const char *_sd0 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); char *_mk = mkdtemp(_mt); _mk ? strdup(_mk) : strdup(""); })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old0); }
    { if ((({ int _r = chdir((d ? d : "")); sh_status = (_r == 0 ? 0 : 1); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; }))) { sh_status = 0; } else { sh_status = 1;
        exit(1);
      }
    }
    fflush(stdout);
    int _nfd1_1 = open("a.sh", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd1_1 = dup(1);
    if (_nfd1_1 >= 0) { dup2(_nfd1_1, 1); close(_nfd1_1); }
    if (_nfd1_1 >= 0) {
        ( fputs("a\n", stdout));
    }
    {
    fflush(stdout);
    dup2(_sfd1_1, 1); close(_sfd1_1);
    }
    fflush(stdout);
    int _nfd2_1 = open("b.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd2_1 = dup(1);
    if (_nfd2_1 >= 0) { dup2(_nfd2_1, 1); close(_nfd2_1); }
    if (_nfd2_1 >= 0) {
        ( fputs("b\n", stdout));
    }
    {
    fflush(stdout);
    dup2(_sfd2_1, 1); close(_sfd2_1);
    }
    ({ int _mr = 0; { if (mkdir("sub", 0777) != 0) _mr = 1; } _mr == 0; });
    fflush(stdout);
    int _nfd3_1 = open("sub/c.sh", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd3_1 = dup(1);
    if (_nfd3_1 >= 0) { dup2(_nfd3_1, 1); close(_nfd3_1); }
    if (_nfd3_1 >= 0) {
        ( fputs("c\n", stdout));
    }
    {
    fflush(stdout);
    dup2(_sfd3_1, 1); close(_sfd3_1);
    }
    fflush(stdout);
    int _nfd4_1 = open(".hidden", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd4_1 = dup(1);
    if (_nfd4_1 >= 0) { dup2(_nfd4_1, 1); close(_nfd4_1); }
    if (_nfd4_1 >= 0) {
        ( fputs("x\n", stdout));
    }
    {
    fflush(stdout);
    dup2(_sfd4_1, 1); close(_sfd4_1);
    }
    char *_a5_0[] = {"ls", "-A", 0};
    static char *_pipe5 = 0; static size_t _pipe5_cap = 0;
    { char *_old6 = file_list; file_list = ({ const char *_sd1 = (char*)(({ if (_pipe5_cap < 1 || !_pipe5) { _pipe5_cap = 64; _pipe5 = (char*)realloc(_pipe5, _pipe5_cap); if (!_pipe5) { exit(127); } } _pipe5[0] = 0; int _pf5[2]; int _ok5 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf5 + 2*_k) != 0) { _ok5 = 0; break; } if (!_ok5) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp5[1]; for (int _t = 0; (_cp5[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp5[0] == 0) { dup2(_pf5[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } execvp(_a5_0[0], _a5_0); _exit(127); } close(_pf5[1]); size_t _gn5 = 0; for (;;) { if (_gn5 + 8192 > _pipe5_cap) { _pipe5_cap = _gn5 + 16384; _pipe5 = (char*)realloc(_pipe5, _pipe5_cap); if (!_pipe5) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf5[0], _pipe5 + _gn5, _pipe5_cap - _gn5 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn5 += (size_t)_r; } close(_pf5[0]); _pipe5[_gn5] = 0; int _st5 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp5[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st5 = _s; } sh_status = WIFEXITED(_st5) ? WEXITSTATUS(_st5) : 1; while (_gn5 > 0 && (_pipe5[_gn5-1] == '\n' || _pipe5[_gn5-1] == '\r')) _pipe5[--_gn5] = 0; } (char*)_pipe5; })); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; }); free(_old6); }
    fputs("File listing:\n", stdout);
    printf("%s\n", file_list);
    char *_a7_0[] = {"find", ".", "-name", "*.sh", "-type", "f", 0};
    static char *_pipe7 = 0; static size_t _pipe7_cap = 0;
    { char *_old8 = found_files; found_files = ({ const char *_sd2 = (char*)(({ if (_pipe7_cap < 1 || !_pipe7) { _pipe7_cap = 64; _pipe7 = (char*)realloc(_pipe7, _pipe7_cap); if (!_pipe7) { exit(127); } } _pipe7[0] = 0; int _pf7[2]; int _ok7 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf7 + 2*_k) != 0) { _ok7 = 0; break; } if (!_ok7) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp7[1]; for (int _t = 0; (_cp7[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp7[0] == 0) { dup2(_pf7[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf7[2*_k]); close(_pf7[2*_k+1]); } execvp(_a7_0[0], _a7_0); _exit(127); } close(_pf7[1]); size_t _gn7 = 0; for (;;) { if (_gn7 + 8192 > _pipe7_cap) { _pipe7_cap = _gn7 + 16384; _pipe7 = (char*)realloc(_pipe7, _pipe7_cap); if (!_pipe7) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf7[0], _pipe7 + _gn7, _pipe7_cap - _gn7 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn7 += (size_t)_r; } close(_pf7[0]); _pipe7[_gn7] = 0; int _st7 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp7[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st7 = _s; } sh_status = WIFEXITED(_st7) ? WEXITSTATUS(_st7) : 1; while (_gn7 > 0 && (_pipe7[_gn7-1] == '\n' || _pipe7[_gn7-1] == '\r')) _pipe7[--_gn7] = 0; } (char*)_pipe7; })); char *_xp2 = _sd2 ? strdup(_sd2) : 0; if (_sd2 && !_xp2) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp2; }); free(_old8); }
    fputs("Found shell scripts:\n", stdout);
    printf("%s\n", found_files);
    ({ int _r = chdir("/"); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; });
    char *_a9[] = {"rm", "-r", "-f", (d ? d : ""), 0};
     ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a9)[0], (_a9)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    fputs("=== File and Directory Operations Complete ===\n", stdout), sh_status = 0;
    free(file_list);
    free(found_files);
    free(d);
    return sh_status;
}