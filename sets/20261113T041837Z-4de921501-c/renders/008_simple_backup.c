#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <sys/stat.h>

/* [ -f/-d/-e/-s/... ] file tests (only used ops) */
/* mkdir -p: create parents as needed (EEXIST is success) */
static int _sh_mkdir_p(const char *p) {
  char buf[4096];
  size_t n = strlen(p);
  if (n >= sizeof buf) return 1;
  memcpy(buf, p, n + 1);
  if (mkdir(buf, 0777) == 0) return 0;
  if (errno == EEXIST) { struct stat st; return (stat(buf, &st) == 0 && S_ISDIR(st.st_mode)) ? 0 : 1; }
  for (char *q = buf + 1; *q; q++) {
    if (*q != '/') continue;
    *q = 0;
    if (mkdir(buf, 0777) != 0 && errno != EEXIST) { *q = '/'; return 1; }
    *q = '/';
  }
  if (mkdir(buf, 0777) == 0 || errno == EEXIST) return 0;
  return 1;
}

int main(void) {
    int sh_status = 0;
    char* d = NULL;
    fputs("Hello, World!\n", stdout);
    { char *_old0 = d; d = ({ const char *_sd0 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); char *_mk = mkdtemp(_mt); _mk ? strdup(_mk) : strdup(""); })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old0); }
    static char *_v1; free(_v1); _v1 = 0;
    if (asprintf(&_v1, "%s/sub1", (d ? d : "")) < 0) _v1 = 0;
    static char *_v2; free(_v2); _v2 = 0;
    if (asprintf(&_v2, "%s/sub2", (d ? d : "")) < 0) _v2 = 0;
    ({ int _mr = 0; { _mr |= _sh_mkdir_p(_v1); } { _mr |= _sh_mkdir_p(_v2); } sh_status = (_mr == 0 ? 0 : 1); _mr == 0; });
    static char *_v3; free(_v3); _v3 = 0;
    if (asprintf(&_v3, "%s/a.txt", (d ? d : "")) < 0) _v3 = 0;
    static char *_v4; free(_v4); _v4 = 0;
    if (asprintf(&_v4, "%s/b.txt", (d ? d : "")) < 0) _v4 = 0;
    ({ { FILE *_tf = fopen(_v3, "a"); if (_tf) fclose(_tf); } { FILE *_tf = fopen(_v4, "a"); if (_tf) fclose(_tf); } sh_status = 0; (sh_status == 0); });
    ({ int _r = chdir((d ? d : "")); sh_status = (_r == 0 ? 0 : 1); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; });
    char *_a5_0[] = {"ls", "-1", 0};
    char *_a5_1[] = {"grep", "-v", "a.txt", 0};
    ({ int _pf5[2]; int _ok5 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf5 + 2*_k) != 0) { _ok5 = 0; break; } if (!_ok5) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp5[2]; for (int _t = 0; (_cp5[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp5[0] == 0) { dup2(_pf5[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } execvp(_a5_0[0], _a5_0); _exit(127); } for (int _t = 0; (_cp5[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp5[1] == 0) { dup2(_pf5[0], 0); for (int _k = 0; _k < 1; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } execvp(_a5_1[0], _a5_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf5[2*_k]); close(_pf5[2*_k+1]); } int _st5 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp5[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st5 = _s; } sh_status = WIFEXITED(_st5) ? WEXITSTATUS(_st5) : 1; } (sh_status == 0); });
    char *_a6_0[] = {"ls", 0};
    char *_a6_1[] = {"grep", "-v", "a.txt", 0};
    static char *_pipe6 = 0; static size_t _pipe6_cap = 0;
    static char *_cw7;
    _cw7 = (char*)(({ if (_pipe6_cap < 1 || !_pipe6) { _pipe6_cap = 64; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { exit(127); } } _pipe6[0] = 0; int _pf6[4]; int _ok6 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf6 + 2*_k) != 0) { _ok6 = 0; break; } if (!_ok6) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp6[2]; for (int _t = 0; (_cp6[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[0] == 0) { dup2(_pf6[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_0[0], _a6_0); _exit(127); } for (int _t = 0; (_cp6[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[1] == 0) { dup2(_pf6[0], 0); dup2(_pf6[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_1[0], _a6_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } close(_pf6[3]); size_t _gn6 = 0; for (;;) { if (_gn6 + 8192 > _pipe6_cap) { _pipe6_cap = _gn6 + 16384; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf6[2], _pipe6 + _gn6, _pipe6_cap - _gn6 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn6 += (size_t)_r; } close(_pf6[2]); _pipe6[_gn6] = 0; int _st6 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp6[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st6 = _s; } sh_status = WIFEXITED(_st6) ? WEXITSTATUS(_st6) : 1; while (_gn6 > 0 && (_pipe6[_gn6-1] == '\n' || _pipe6[_gn6-1] == '\r')) _pipe6[--_gn6] = 0; } (char*)_pipe6; }));
    static char *_sj8b = 0; static size_t _sj8b_cap = 0; free(_sj8b); _sj8b_cap = strlen(_cw7) + 2; _sj8b = (char*)malloc(_sj8b_cap); if (!_sj8b) { _sj8b_cap = 0; fputs("sh2c: out of memory\n", stderr); exit(127); } strcpy(_sj8b + 1, _cw7); size_t _sj8o = 1; { char *_sj8i = _sj8b + 1, *_sj8d = _sj8b + 1; while (*_sj8i == ' ' || *_sj8i == '\t' || *_sj8i == '\n') _sj8i++; while (*_sj8i) { char _sj8c = *_sj8i++; if (_sj8c == ' ' || _sj8c == '\t' || _sj8c == '\n') { while (*_sj8i == ' ' || *_sj8i == '\t' || *_sj8i == '\n') _sj8i++; if (*_sj8i) *_sj8d++ = ' '; } else *_sj8d++ = _sj8c; } *_sj8d = 0; }
    printf("%s\n", ((char*)(_sj8b + _sj8o) ? (char*)(_sj8b + _sj8o) : ""));
    ({ int _r = chdir("/"); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; });
    char *_a9[] = {"rm", "-r", "-f", (d ? d : ""), 0};
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a9)[0], (_a9)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    free(d);
    return sh_status;
}