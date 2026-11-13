#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <fcntl.h>
#include <fnmatch.h>
#include <glob.h>

/* sh2 runtime: exit-status register + shell-out helpers */
static int _sh_rc;
static char *_sh_cmd = 0; static size_t _sh_cap = 0;
static char *__attribute__((returns_nonnull)) _sh_grow(char **b, size_t *cap, size_t need) {
  if (*b && need <= *cap) return *b;
  size_t ncap = *cap ? *cap : 256;
  if (ncap < need) ncap = (need - ncap <= ncap) ? ncap + ncap : need;
  char *np = (char*)realloc(*b, ncap);
  if (!np) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
  *b = np; *cap = ncap;
  return np;
}
static void _sh_add(const char *s) {
  size_t l = _sh_cmd ? strlen(_sh_cmd) : 0, n = strlen(s);
  _sh_grow(&_sh_cmd, &_sh_cap, l + n + 1);
  if (_sh_cmd) memcpy(_sh_cmd + l, s, n + 1);
}
static void _sh_addraw(const char *s) { _sh_add(" "); _sh_add(s); }
/* buffer-parameterized variants (capture sites' private buffers) */
/* [ -f/-d/-e/-s/... ] file tests (only used ops) */
static char *files[1024] = {0};
static size_t files_len;

char* __ps_tmp0 = NULL;
char* __ps_tmp1 = NULL;
char* file = NULL;
char* size = NULL;


static int _stg1_0(void) {
char *_a2_0[] = {"yes", "well", 0};
char *_a2_1[] = {"head", "-3", 0};
static char *_pipe2 = 0; static size_t _pipe2_cap = 0;
long long _q3 = _sh_rc;
static char *_v4 = 0; static size_t _v4_cap = 0;
_v4 = ({ if (_pipe2_cap < 1 || !_pipe2) { _pipe2_cap = 64; _pipe2 = (char*)realloc(_pipe2, _pipe2_cap); if (!_pipe2) { exit(127); } } _pipe2[0] = 0; int _pf2[4]; int _ok2 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf2 + 2*_k) != 0) { _ok2 = 0; break; } if (!_ok2) { _q3 = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp2[2]; for (int _t = 0; (_cp2[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp2[0] == 0) { dup2(_pf2[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf2[2*_k]); close(_pf2[2*_k+1]); } execvp(_a2_0[0], _a2_0); _exit(127); } for (int _t = 0; (_cp2[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp2[1] == 0) { dup2(_pf2[0], 0); dup2(_pf2[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf2[2*_k]); close(_pf2[2*_k+1]); } execvp(_a2_1[0], _a2_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf2[2*_k]); close(_pf2[2*_k+1]); } close(_pf2[3]); size_t _gn2 = 0; for (;;) { if (_gn2 + 8192 > _pipe2_cap) { _pipe2_cap = _gn2 + 16384; _pipe2 = (char*)realloc(_pipe2, _pipe2_cap); if (!_pipe2) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf2[2], _pipe2 + _gn2, _pipe2_cap - _gn2 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn2 += (size_t)_r; } close(_pf2[2]); _pipe2[_gn2] = 0; int _st2 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp2[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st2 = _s; } _q3 = WIFEXITED(_st2) ? WEXITSTATUS(_st2) : 1; while (_gn2 > 0 && (_pipe2[_gn2-1] == '\n' || _pipe2[_gn2-1] == '\r')) _pipe2[--_gn2] = 0; } (char*)_pipe2; });
  printf("Three wells: %s\n", ((char*)(_v4) ? (char*)(_v4) : ""));
  return 0;
}

static char *_cap_0(int *rc) {
  static char *buf = 0; static size_t buf_cap = 0;
__ps_tmp0 = ({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); int _fd = mkstemp(_mt); if (_fd >= 0) close(_fd); (*rc) = (_fd >= 0) ? 0 : 1; (_fd >= 0) ? strdup(_mt) : strdup(""); });
_sh_addraw("__ps_tmp0=");
const char *_cs22 = (__ps_tmp0 ? __ps_tmp0 : "");
static char *_cq23;
{ const char *_qw = ((__ps_tmp0 ? __ps_tmp0 : "")); size_t _qn = 3; const char *_qp; for (_qp = _qw; *_qp; _qp++) _qn += (*_qp == '\'') ? 5u : 1u; free(_cq23); _cq23 = malloc(_qn); if (!_cq23) { fputs("sh2c: out of memory\n", stderr); exit(127); } char *_qd = _cq23; *_qd++ = ' '; *_qd++ = '\''; for (_qp = _qw; *_qp; _qp++) { if (*_qp == '\'') { memcpy(_qd, "'\"'\"'", 5); _qd += 5; } else *_qd++ = *_qp; } *_qd++ = '\''; *_qd = 0; }
__ps_tmp1 = ({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); int _fd = mkstemp(_mt); if (_fd >= 0) close(_fd); (*rc) = (_fd >= 0) ? 0 : 1; (_fd >= 0) ? strdup(_mt) : strdup(""); });
_sh_addraw("__ps_tmp1=");
const char *_cs24 = (__ps_tmp1 ? __ps_tmp1 : "");
static char *_cq25;
{ const char *_qw = ((__ps_tmp1 ? __ps_tmp1 : "")); size_t _qn = 3; const char *_qp; for (_qp = _qw; *_qp; _qp++) _qn += (*_qp == '\'') ? 5u : 1u; free(_cq25); _cq25 = malloc(_qn); if (!_cq25) { fputs("sh2c: out of memory\n", stderr); exit(127); } char *_qd = _cq25; *_qd++ = ' '; *_qd++ = '\''; for (_qp = _qw; *_qp; _qp++) { if (*_qp == '\'') { memcpy(_qd, "'\"'\"'", 5); _qd += 5; } else *_qd++ = *_qp; } *_qd++ = '\''; *_qd = 0; }
static char *_cq26;
{ const char *_qw = ((__ps_tmp0 ? __ps_tmp0 : "")); size_t _qn = 3; const char *_qp; for (_qp = _qw; *_qp; _qp++) _qn += (*_qp == '\'') ? 5u : 1u; free(_cq26); _cq26 = malloc(_qn); if (!_cq26) { fputs("sh2c: out of memory\n", stderr); exit(127); } char *_qd = _cq26; *_qd++ = ' '; *_qd++ = '\''; for (_qp = _qw; *_qp; _qp++) { if (*_qp == '\'') { memcpy(_qd, "'\"'\"'", 5); _qd += 5; } else *_qd++ = *_qp; } *_qd++ = '\''; *_qd = 0; }
static char *_cq27;
{ const char *_qw = ((__ps_tmp1 ? __ps_tmp1 : "")); size_t _qn = 3; const char *_qp; for (_qp = _qw; *_qp; _qp++) _qn += (*_qp == '\'') ? 5u : 1u; free(_cq27); _cq27 = malloc(_qn); if (!_cq27) { fputs("sh2c: out of memory\n", stderr); exit(127); } char *_qd = _cq27; *_qd++ = ' '; *_qd++ = '\''; for (_qp = _qw; *_qp; _qp++) { if (*_qp == '\'') { memcpy(_qd, "'\"'\"'", 5); _qd += 5; } else *_qd++ = *_qp; } *_qd++ = '\''; *_qd = 0; }
static char *_cn28; free(_cn28); _cn28 = 0; if (asprintf(&_cn28, "%s%s%s%s%s%s%s%s%s%s%s", _cs22, " ; { ( 'sort' 'file1.txt' ) >", _cq23, " ; } ;", _cs24, " ; { ( 'sort' 'file2.txt' ) >", _cq25, " ; } ; 'comm' '-23'", _cq26, _cq27, " && { kill $(cat ${__ps_tmp0}.pid) 2>/dev/null; kill $(cat ${__ps_tmp1}.pid) 2>/dev/null; rm -f ${__ps_tmp0} ${__ps_tmp0}.pid ${__ps_tmp1} ${__ps_tmp1}.pid ; }") < 0) _cn28 = 0;
/* capture: bash -c */ { char *_ca0[] = {"bash", "-c", (char*)((_cn28) ? (_cn28) : ""), 0}; ({ if (buf_cap < 1 || !buf) { buf_cap = 64; buf = (char*)realloc(buf, buf_cap); if (!buf) { exit(127); } } buf[0] = 0; int _pf0[2]; int _ok0 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf0 + 2*_k) != 0) { _ok0 = 0; break; } if (!_ok0) { (*rc) = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp0[1]; for (int _t = 0; (_cp0[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp0[0] == 0) { dup2(_pf0[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } execvp(_ca0[0], _ca0); _exit(127); } close(_pf0[1]); size_t _gn0 = 0; for (;;) { if (_gn0 + 8192 > buf_cap) { buf_cap = _gn0 + 16384; buf = (char*)realloc(buf, buf_cap); if (!buf) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf0[0], buf + _gn0, buf_cap - _gn0 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn0 += (size_t)_r; } close(_pf0[0]); buf[_gn0] = 0; int _st0 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp0[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st0 = _s; } (*rc) = WIFEXITED(_st0) ? WEXITSTATUS(_st0) : 1; while (_gn0 > 0 && (buf[_gn0-1] == '\n' || buf[_gn0-1] == '\r')) buf[--_gn0] = 0; } (char*)buf; }); }
return buf;
}

static void get_file_size(int argc, char **argv, int *_rcp, int _noecho) {
    file = ({ const char *_sd0 = (char*)(((1 < argc && argv[1]) ? argv[1] : "")); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; });
    ((*_rcp) = 0, 1);
    char *_a21_0[] = {"wc", "-c", 0};
    static char *_pipe21 = 0; static size_t _pipe21_cap = 0;
    size = ({ const char *_sd1 = (char*)(({ int _nfd21_0 = open((file ? file : ""), O_RDONLY); int _sfd21_0 = dup(0); if (_nfd21_0 >= 0) { dup2(_nfd21_0, 0); close(_nfd21_0); } const char *_cv21 = 0; if (_nfd21_0 >= 0) { _cv21 = ({ if (_pipe21_cap < 1 || !_pipe21) { _pipe21_cap = 64; _pipe21 = (char*)realloc(_pipe21, _pipe21_cap); if (!_pipe21) { exit(127); } } _pipe21[0] = 0; int _pf21[2]; int _ok21 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf21 + 2*_k) != 0) { _ok21 = 0; break; } if (!_ok21) { (*_rcp) = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp21[1]; for (int _t = 0; (_cp21[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp21[0] == 0) { dup2(_pf21[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf21[2*_k]); close(_pf21[2*_k+1]); } execvp(_a21_0[0], _a21_0); _exit(127); } close(_pf21[1]); size_t _gn21 = 0; for (;;) { if (_gn21 + 8192 > _pipe21_cap) { _pipe21_cap = _gn21 + 16384; _pipe21 = (char*)realloc(_pipe21, _pipe21_cap); if (!_pipe21) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf21[0], _pipe21 + _gn21, _pipe21_cap - _gn21 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn21 += (size_t)_r; } close(_pf21[0]); _pipe21[_gn21] = 0; int _st21 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp21[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st21 = _s; } (*_rcp) = WIFEXITED(_st21) ? WEXITSTATUS(_st21) : 1; while (_gn21 > 0 && (_pipe21[_gn21-1] == '\n' || _pipe21[_gn21-1] == '\r')) _pipe21[--_gn21] = 0; } (char*)_pipe21; }); } else { (*_rcp) = 1; } if (_sfd21_0 >= 0) { dup2(_sfd21_0, 0); close(_sfd21_0); } _cv21 ? (char*)_cv21 : ""; })); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; });
    ((*_rcp) = 0, 1);
    printf("File %s has %s bytes\n", (char*)((file ? file : "")), (char*)((size ? size : ""))), (*_rcp) = 0;
}

int main(void) {
    int sh_status = 0;
    char* count = NULL;
    char* d = NULL;
    char* here_string_result = NULL;
    char* nested_result = NULL;
    char* perl_result = NULL;
    char* process_result = NULL;
    { char *_old0 = d; d = ({ const char *_sd2 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); char *_mk = mkdtemp(_mt); sh_status = _mk ? 0 : 1; _mk ? strdup(_mk) : strdup(""); })); char *_xp2 = _sd2 ? strdup(_sd2) : 0; if (_sd2 && !_xp2) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp2; }); free(_old0); }
    { if ((({ int _r = chdir((d ? d : "")); sh_status = (_r == 0 ? 0 : 1); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; }))) { sh_status = 0; } else { sh_status = 1;
        exit(1);
      }
    }
    fputs("=== Complex Backtick Examples ===\n", stdout), sh_status = 0;
    static char *_pipe1 = 0; static size_t _pipe1_cap = 0;
    { char *_old5 = nested_result; nested_result = ({ const char *_sd3 = (char*)(({ if (_pipe1_cap < 1 || !_pipe1) { _pipe1_cap = 64; _pipe1 = (char*)realloc(_pipe1, _pipe1_cap); if (!_pipe1) { exit(127); } } _pipe1[0] = 0; int _pf1[2]; int _ok1 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf1 + 2*_k) != 0) { _ok1 = 0; break; } if (!_ok1) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp1[1]; for (int _t = 0; (_cp1[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp1[0] == 0) { dup2(_pf1[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf1[2*_k]); close(_pf1[2*_k+1]); } int _tr = _stg1_0(); fflush(stdout); _exit(_tr); } close(_pf1[1]); size_t _gn1 = 0; for (;;) { if (_gn1 + 8192 > _pipe1_cap) { _pipe1_cap = _gn1 + 16384; _pipe1 = (char*)realloc(_pipe1, _pipe1_cap); if (!_pipe1) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf1[0], _pipe1 + _gn1, _pipe1_cap - _gn1 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn1 += (size_t)_r; } close(_pf1[0]); _pipe1[_gn1] = 0; int _st1 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp1[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st1 = _s; } sh_status = WIFEXITED(_st1) ? WEXITSTATUS(_st1) : 1; while (_gn1 > 0 && (_pipe1[_gn1-1] == '\n' || _pipe1[_gn1-1] == '\r')) _pipe1[--_gn1] = 0; } (char*)_pipe1; })); char *_xp3 = _sd3 ? strdup(_sd3) : 0; if (_sd3 && !_xp3) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp3; }); free(_old5); }
    printf("Nested backticks: %s\n", nested_result), sh_status = 0;
    char *_a6_0[] = {"ls", "-1", 0};
    char *_a6_1[] = {"wc", "-l", 0};
    static char *_pipe6 = 0; static size_t _pipe6_cap = 0;
    { char *_old7 = count; count = ({ const char *_sd4 = (char*)(({ if (_pipe6_cap < 1 || !_pipe6) { _pipe6_cap = 64; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { exit(127); } } _pipe6[0] = 0; int _pf6[4]; int _ok6 = 1; for (int _k = 0; _k < 2; _k++) if (pipe(_pf6 + 2*_k) != 0) { _ok6 = 0; break; } if (!_ok6) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp6[2]; for (int _t = 0; (_cp6[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[0] == 0) { dup2(_pf6[1], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_0[0], _a6_0); _exit(127); } for (int _t = 0; (_cp6[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp6[1] == 0) { dup2(_pf6[0], 0); dup2(_pf6[3], 1); for (int _k = 0; _k < 2; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } execvp(_a6_1[0], _a6_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf6[2*_k]); close(_pf6[2*_k+1]); } close(_pf6[3]); size_t _gn6 = 0; for (;;) { if (_gn6 + 8192 > _pipe6_cap) { _pipe6_cap = _gn6 + 16384; _pipe6 = (char*)realloc(_pipe6, _pipe6_cap); if (!_pipe6) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf6[2], _pipe6 + _gn6, _pipe6_cap - _gn6 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn6 += (size_t)_r; } close(_pf6[2]); _pipe6[_gn6] = 0; int _st6 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp6[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st6 = _s; } sh_status = WIFEXITED(_st6) ? WEXITSTATUS(_st6) : 1; while (_gn6 > 0 && (_pipe6[_gn6-1] == '\n' || _pipe6[_gn6-1] == '\r')) _pipe6[--_gn6] = 0; } (char*)_pipe6; })); char *_xp4 = _sd4 ? strdup(_sd4) : 0; if (_sd4 && !_xp4) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp4; }); free(_old7); }
    printf("File count: %s\n", count), sh_status = 0;
    if ((!(sh_status = ((strcmp("root", "root") != 0)) ? 1 : 0))) {
        fputs("Running as root\n", stdout), sh_status = 0;
    } else {
        fputs("Not running as root\n", stdout), sh_status = 0;
    }
    if (fnmatch("Linux", "Darwin", 0) == 0) {
        fputs("Running on Linux\n", stdout), sh_status = 0;
    }
    else if (fnmatch("Darwin", "Darwin", 0) == 0) {
        fputs("Running on macOS\n", stdout), sh_status = 0;
    }
    else if (fnmatch("*", "Darwin", 0) == 0) {
        fputs("Running on other system\n", stdout), sh_status = 0;
    }
    else { sh_status = 0; }
    get_file_size(2, (char*[]){"get_file_size", "000__01_file_directory_operations.sh"}, &sh_status, 1);
    size_t _ai8 = 0;
    files_len = 0;
    char *_a9_0[1024]; int _an9_0 = 0;
    _a9_0[_an9_0++] = "ls";
    _a9_0[_an9_0++] = "-1";
    glob_t _gg9_0_1 = {0}; const char *_pg9_0_1 = "*.sh"; int _gm9_0_1 = _an9_0;
    if (glob(_pg9_0_1, 0, 0, &_gg9_0_1) == 0) { for (size_t _gi = 0; _gi < _gg9_0_1.gl_pathc && _an9_0 < 1022; _gi++) _a9_0[_an9_0++] = _gg9_0_1.gl_pathv[_gi]; }
    if (_an9_0 == _gm9_0_1) _a9_0[_an9_0++] = (char*)_pg9_0_1;
    glob_t _gg9_0_2 = {0}; const char *_pg9_0_2 = "examples/*.sh"; int _gm9_0_2 = _an9_0;
    if (glob(_pg9_0_2, 0, 0, &_gg9_0_2) == 0) { for (size_t _gi = 0; _gi < _gg9_0_2.gl_pathc && _an9_0 < 1022; _gi++) _a9_0[_an9_0++] = _gg9_0_2.gl_pathv[_gi]; }
    if (_an9_0 == _gm9_0_2) _a9_0[_an9_0++] = (char*)_pg9_0_2;
    _a9_0[_an9_0] = 0;
    static char *_pipe9 = 0; static size_t _pipe9_cap = 0;
    static char *_sp10 = 0; static size_t _sp10_cap = 0; char *_sp10_sv = 0; size_t _sn_sp10 = 0; _sh_grow(&_sp10, &_sp10_cap, strlen(({ int _nfd9_2 = open("/dev/null", O_WRONLY|O_CREAT|O_TRUNC, 0666); int _sfd9_2 = dup(2); if (_nfd9_2 >= 0) { dup2(_nfd9_2, 2); close(_nfd9_2); } const char *_cv9 = 0; if (_nfd9_2 >= 0) { _cv9 = ({ if (_pipe9_cap < 1 || !_pipe9) { _pipe9_cap = 64; _pipe9 = (char*)realloc(_pipe9, _pipe9_cap); if (!_pipe9) { exit(127); } } _pipe9[0] = 0; int _pf9[2]; int _ok9 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf9 + 2*_k) != 0) { _ok9 = 0; break; } if (!_ok9) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp9[1]; for (int _t = 0; (_cp9[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp9[0] == 0) { dup2(_pf9[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf9[2*_k]); close(_pf9[2*_k+1]); } execvp(_a9_0[0], _a9_0); _exit(127); } close(_pf9[1]); size_t _gn9 = 0; for (;;) { if (_gn9 + 8192 > _pipe9_cap) { _pipe9_cap = _gn9 + 16384; _pipe9 = (char*)realloc(_pipe9, _pipe9_cap); if (!_pipe9) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf9[0], _pipe9 + _gn9, _pipe9_cap - _gn9 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn9 += (size_t)_r; } close(_pf9[0]); _pipe9[_gn9] = 0; int _st9 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp9[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st9 = _s; } sh_status = WIFEXITED(_st9) ? WEXITSTATUS(_st9) : 1; while (_gn9 > 0 && (_pipe9[_gn9-1] == '\n' || _pipe9[_gn9-1] == '\r')) _pipe9[--_gn9] = 0; } (char*)_pipe9; }); } else { sh_status = 1; } if (_sfd9_2 >= 0) { dup2(_sfd9_2, 2); close(_sfd9_2); } _cv9 ? (char*)_cv9 : ""; })) + 1); strcpy(_sp10, ({ int _nfd9_2 = open("/dev/null", O_WRONLY|O_CREAT|O_TRUNC, 0666); int _sfd9_2 = dup(2); if (_nfd9_2 >= 0) { dup2(_nfd9_2, 2); close(_nfd9_2); } const char *_cv9 = 0; if (_nfd9_2 >= 0) { _cv9 = ({ if (_pipe9_cap < 1 || !_pipe9) { _pipe9_cap = 64; _pipe9 = (char*)realloc(_pipe9, _pipe9_cap); if (!_pipe9) { exit(127); } } _pipe9[0] = 0; int _pf9[2]; int _ok9 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf9 + 2*_k) != 0) { _ok9 = 0; break; } if (!_ok9) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp9[1]; for (int _t = 0; (_cp9[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp9[0] == 0) { dup2(_pf9[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf9[2*_k]); close(_pf9[2*_k+1]); } execvp(_a9_0[0], _a9_0); _exit(127); } close(_pf9[1]); size_t _gn9 = 0; for (;;) { if (_gn9 + 8192 > _pipe9_cap) { _pipe9_cap = _gn9 + 16384; _pipe9 = (char*)realloc(_pipe9, _pipe9_cap); if (!_pipe9) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf9[0], _pipe9 + _gn9, _pipe9_cap - _gn9 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn9 += (size_t)_r; } close(_pf9[0]); _pipe9[_gn9] = 0; int _st9 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp9[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st9 = _s; } sh_status = WIFEXITED(_st9) ? WEXITSTATUS(_st9) : 1; while (_gn9 > 0 && (_pipe9[_gn9-1] == '\n' || _pipe9[_gn9-1] == '\r')) _pipe9[--_gn9] = 0; } (char*)_pipe9; }); } else { sh_status = 1; } if (_sfd9_2 >= 0) { dup2(_sfd9_2, 2); close(_sfd9_2); } _cv9 ? (char*)_cv9 : ""; }));
    for (char *_wt_sp10 = strtok_r(_sp10, " \t\n", &_sp10_sv); _wt_sp10; _wt_sp10 = strtok_r(0, " \t\n", &_sp10_sv)) {
        if (_ai8 + _sn_sp10 >= 1024) { fprintf(stderr, "sh2c: capacity exceeded: files\n"); exit(127); }
        files[_ai8 + _sn_sp10] = ({ const char *_sd5 = _wt_sp10; char *_xp5 = _sd5 ? strdup(_sd5) : 0; if (_sd5 && !_xp5) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp5; });
        _sn_sp10++;
    }
    files_len = _ai8 + _sn_sp10;
    globfree(&_gg9_0_1);
    globfree(&_gg9_0_2);
    printf("Shell scripts found: %lld\n", (long long)(files_len)), sh_status = 0;
    for (size_t _ai_files = 0; _ai_files < files_len; _ai_files++) {
        file = files[_ai_files];
        printf("  - %s\n", (char*)((file ? file : ""))), sh_status = 0;
    }
    fflush(stdout);
    int _nfd12_1 = open("file1.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd12_1 = dup(1);
    if (_nfd12_1 >= 0) { dup2(_nfd12_1, 1); close(_nfd12_1); }
    if (_nfd12_1 >= 0) {
        fputs("apple\nbanana\ncherry\n", stdout), sh_status = 0;
    }
    {
    fflush(stdout);
    dup2(_sfd12_1, 1); close(_sfd12_1);
    }
    fflush(stdout);
    int _nfd13_1 = open("file2.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666);
    int _sfd13_1 = dup(1);
    if (_nfd13_1 >= 0) { dup2(_nfd13_1, 1); close(_nfd13_1); }
    if (_nfd13_1 >= 0) {
        fputs("banana\ncherry\ndate\n", stdout), sh_status = 0;
    }
    {
    fflush(stdout);
    dup2(_sfd13_1, 1); close(_sfd13_1);
    }
    { char *_old15 = process_result; process_result = ({ const char *_sd6 = (char*)(_cap_0(&sh_status)); char *_xp6 = _sd6 ? strdup(_sd6) : 0; if (_sd6 && !_xp6) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp6; }); free(_old15); }
    fputs("Process substitution result:\n", stdout), sh_status = 0;
    printf("%s\n", process_result), sh_status = 0;
    { char *_old17 = here_string_result; here_string_result = ({ const char *_sd7 = (char*)(({ static const unsigned char _trp16[][2] = {{97,65},{98,66},{99,67},{100,68},{101,69},{102,70},{103,71},{104,72},{105,73},{106,74},{107,75},{108,76},{109,77},{110,78},{111,79},{112,80},{113,81},{114,82},{115,83},{116,84},{117,85},{118,86},{119,87},{120,88},{121,89},{122,90}}; const char *_trs16 = "hello world"; char *_trb16 = strdup(_trs16 ? _trs16 : ""); if (!_trb16) { fputs("sh2c: out of memory\n", stderr); exit(127); } for (char *_p = _trb16; *_p; _p++) for (size_t _k = 0; _k < sizeof _trp16 / sizeof _trp16[0]; _k++) if (_trp16[_k][0] == (unsigned char)*_p) { *_p = (char)_trp16[_k][1]; break; } _trb16; })); char *_xp7 = _sd7 ? strdup(_sd7) : 0; if (_sd7 && !_xp7) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp7; }); free(_old17); }
    printf("Here string result: %s\n", here_string_result), sh_status = 0;
    char *_a18_0[] = {"perl", "-e", "print \"Hello from Perl\\n\"", 0};
    static char *_pipe18 = 0; static size_t _pipe18_cap = 0;
    { char *_old19 = perl_result; perl_result = ({ const char *_sd8 = (char*)(({ if (_pipe18_cap < 1 || !_pipe18) { _pipe18_cap = 64; _pipe18 = (char*)realloc(_pipe18, _pipe18_cap); if (!_pipe18) { exit(127); } } _pipe18[0] = 0; int _pf18[2]; int _ok18 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf18 + 2*_k) != 0) { _ok18 = 0; break; } if (!_ok18) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp18[1]; for (int _t = 0; (_cp18[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp18[0] == 0) { dup2(_pf18[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf18[2*_k]); close(_pf18[2*_k+1]); } execvp(_a18_0[0], _a18_0); _exit(127); } close(_pf18[1]); size_t _gn18 = 0; for (;;) { if (_gn18 + 8192 > _pipe18_cap) { _pipe18_cap = _gn18 + 16384; _pipe18 = (char*)realloc(_pipe18, _pipe18_cap); if (!_pipe18) { fputs("sh2c: out of memory\n", stderr); exit(127); } } ssize_t _r = read(_pf18[0], _pipe18 + _gn18, _pipe18_cap - _gn18 - 1); if (_r < 0) { if (errno == EINTR) continue; break; } if (_r == 0) break; _gn18 += (size_t)_r; } close(_pf18[0]); _pipe18[_gn18] = 0; int _st18 = 0; for (int _k = 0; _k < 1; _k++) { int _s = 0; while (waitpid(_cp18[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 0) _st18 = _s; } sh_status = WIFEXITED(_st18) ? WEXITSTATUS(_st18) : 1; while (_gn18 > 0 && (_pipe18[_gn18-1] == '\n' || _pipe18[_gn18-1] == '\r')) _pipe18[--_gn18] = 0; } (char*)_pipe18; })); char *_xp8 = _sd8 ? strdup(_sd8) : 0; if (_sd8 && !_xp8) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp8; }); free(_old19); }
    printf("Perl result: %s\n", perl_result), sh_status = 0;
    ({ int _mr = 0; { if (unlink("file1.txt") != 0 && errno != ENOENT) _mr = 1; } { if (unlink("file2.txt") != 0 && errno != ENOENT) _mr = 1; } sh_status = (_mr == 0 ? 0 : 1); (sh_status == 0); });
    fputs("=== Complex Backtick Examples Complete ===\n", stdout), sh_status = 0;
    ({ int _r = chdir("/"); sh_status = (_r == 0 ? 0 : 1); if (_r == 0) { char *_pw = getcwd(0, 0); if (_pw) { setenv("PWD", _pw, 1); free(_pw); } } _r == 0; });
    char *_a20[] = {"rm", "-r", "-f", (d ? d : ""), 0};
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_a20)[0], (_a20)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    free(process_result);
    free(here_string_result);
    free(d);
    free(perl_result);
    free(count);
    free(nested_result);
    return sh_status;
}