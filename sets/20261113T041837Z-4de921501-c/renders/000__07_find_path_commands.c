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
char* d = NULL;


static char *_cap_0(int *rc) {
  static char *buf = 0; static size_t buf_cap = 0;
{ const char *_se6 = ((d ? d : "")); setenv("d", _se6 ? _se6 : "", 1); }
/* capture: bash -c */ { char *_ca0[] = {"bash", "-c", (char*)" 'cd' \"$d\" && 'find' '.' '-name' '*.sh' '-type' 'f' | 'sort'", 0}; ({ if (buf_cap < 1 || !buf) { buf_cap = 64; buf = (char*)realloc(buf, buf_cap); if (!buf) { exit(127); } } buf[0] = 0; int _pf0[2]; int _ok0 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf0 + 2*_k) != 0) { _ok0 = 0; break; } if (!_ok0) { (*rc) = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp0[1]; for (int _t = 0; (_cp0[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp0[0] == 0) { dup2(_pf0[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } execvp(_ca0[0], _ca0); _exit(127); } close(_pf0[1]); size_t _gn0 = 0; for (;;) { if (_gn0 + 8192 > buf_cap) { buf_cap = _gn0 + 16384; buf = (char*)realloc(buf, buf_cap); if (!buf) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf0[0], buf + _gn0, buf_cap - _gn0 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn0 += (size_t)_r; } close(_pf0[0]); buf[_gn0] = 0; int _st0 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp0[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st0 = _s; } (*rc) = WIFEXITED(_st0) ? WEXITSTATUS(_st0) : 1; while (_gn0 > 0 && (buf[_gn0-1] == '\n' || buf[_gn0-1] == '\r')) buf[--_gn0] = 0; } (char*)buf; }); }
return buf;
}

int main(void) {
    int sh_status = 0;
    char* found_files = NULL;
    { char *_old0 = d; d = ({ const char *_sd0 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); char *_mk = mkdtemp(_mt); _mk ? strdup(_mk) : strdup(""); })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old0); }
    static char *_v1; free(_v1); _v1 = 0;
    if (asprintf(&_v1, "%s/sub", (d ? d : "")) < 0) _v1 = 0;
    ({ int _mr = 0; { _mr |= _sh_mkdir_p(_v1); } sh_status = (_mr == 0 ? 0 : 1); _mr == 0; });
    static char *_v2; free(_v2); _v2 = 0;
    if (asprintf(&_v2, "%s/a.sh", (d ? d : "")) < 0) _v2 = 0;
    static char *_v3; free(_v3); _v3 = 0;
    if (asprintf(&_v3, "%s/b.sh", (d ? d : "")) < 0) _v3 = 0;
    static char *_v4; free(_v4); _v4 = 0;
    if (asprintf(&_v4, "%s/sub/c.sh", (d ? d : "")) < 0) _v4 = 0;
    static char *_v5; free(_v5); _v5 = 0;
    if (asprintf(&_v5, "%s/data.txt", (d ? d : "")) < 0) _v5 = 0;
    ({ { FILE *_tf = fopen(_v2, "a"); if (_tf) fclose(_tf); } { FILE *_tf = fopen(_v3, "a"); if (_tf) fclose(_tf); } { FILE *_tf = fopen(_v4, "a"); if (_tf) fclose(_tf); } { FILE *_tf = fopen(_v5, "a"); if (_tf) fclose(_tf); } sh_status = 0; (sh_status == 0); });
    { char *_old7 = found_files; found_files = ({ const char *_sd1 = (char*)(_cap_0(&sh_status)); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; }); free(_old7); }
    fputs("Found shell scripts:\n", stdout);
    printf("%s\n", found_files);
    char *_a8[] = {"rm", "-r", "-f", (d ? d : ""), 0};
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a8)[0], (_a8)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    free(found_files);
    return sh_status;
}