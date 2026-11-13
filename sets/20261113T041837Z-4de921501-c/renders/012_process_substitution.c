#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <fcntl.h>
#include <regex.h>


/* sh2 runtime: exit-status register + shell-out helpers */
static int _sh_rc;
/* grepMatches(text, pat, flags) — the `grep -o` lift: print
   each match on its own line (grep -o's output), set _sh_rc
   (0 iff any match), return the newline-joined matches */
static char *_sh_grep_matches(const char *text, const char *pat, const char *flags, int *rc) {
  static char *buf = 0; static size_t cap = 0;
  const char *p = pat; size_t dn = 0;
  size_t n = strlen(text);
  char *re = (char*)malloc(2 * strlen(pat) + 1);
  if (!re) { fputs("sh2c: out of memory\n", stderr); exit(127); }
  if (2 * n + 2 > cap) {
    char *np = (char*)realloc(buf, 2 * n + 2);
    if (!np) { fputs("sh2c: out of memory\n", stderr); exit(127); }
    buf = np; cap = 2 * n + 2;
  }
  buf[0] = 0;
  if (strchr(flags, 'F')) {
    for (; *p; p++) {
      if (strchr("\\^$.|?*+()[]{}${}", *p)) re[dn++] = '\\';
      re[dn++] = *p;
    }
  } else if (!strchr(flags, 'E')) {
    for (; *p; p++) {
      if (*p == '\\' && p[1] && strchr("(){}?+|", p[1])) { re[dn++] = p[1]; p++; }
      else re[dn++] = *p;
    }
  } else {
    dn = strlen(pat); memcpy(re, pat, dn);
  }
  re[dn] = 0;
  regex_t rx;
  int rcomp = regcomp(&rx, re, REG_EXTENDED | REG_NEWLINE | (strchr(flags, 'i') ? REG_ICASE : 0));
  free(re);
  if (rcomp != 0) { *rc = 2; return buf; /* bad pattern: grep exits 2 */ }
  size_t out = 0, pos = 0, any = 0;
  regmatch_t m;
  while (pos <= n) {
    int rc = regexec(&rx, text + pos, 1, &m, 0);
    if (rc != 0) break;
    size_t ms = (size_t)m.rm_so, me = (size_t)m.rm_eo;
    if (me == ms) { pos++; continue; } /* grep -o skips empty matches */
    any = 1;
    printf("%.*s\n", (int)(me - ms), text + pos + ms);
    size_t len = me - ms;
    memcpy(buf + out, text + pos + ms, len);
    out += len;
    buf[out++] = '\n';
    pos += me;
  }
  buf[out] = 0;
  regfree(&rx);
  *rc = any ? 0 : 1;
  return buf;
}

static char *lines[1024] = {0};
static size_t lines_len;

char* __ps_tmp3 = NULL;
char* __ps_tmp4 = NULL;


static int _sh_site_0(int *rc) {
__ps_tmp3 = ({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); int _fd = mkstemp(_mt); if (_fd >= 0) close(_fd); (*rc) = (_fd >= 0) ? 0 : 1; (_fd >= 0) ? strdup(_mt) : strdup(""); });
const char *_cs18 = (__ps_tmp3 ? __ps_tmp3 : "");
static char *_cq19;
{ const char *_qw = ((__ps_tmp3 ? __ps_tmp3 : "")); size_t _qn = 3; const char *_qp; for (_qp = _qw; *_qp; _qp++) _qn += (*_qp == '\'') ? 5u : 1u; free(_cq19); _cq19 = malloc(_qn); if (!_cq19) { fputs("sh2c: out of memory\n", stderr); exit(127); } char *_qd = _cq19; *_qd++ = ' '; *_qd++ = '\''; for (_qp = _qw; *_qp; _qp++) { if (*_qp == '\'') { memcpy(_qd, "'\"'\"'", 5); _qd += 5; } else *_qd++ = *_qp; } *_qd++ = '\''; *_qd = 0; }
__ps_tmp4 = ({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); int _fd = mkstemp(_mt); if (_fd >= 0) close(_fd); (*rc) = (_fd >= 0) ? 0 : 1; (_fd >= 0) ? strdup(_mt) : strdup(""); });
const char *_cs20 = (__ps_tmp4 ? __ps_tmp4 : "");
static char *_cq21;
{ const char *_qw = ((__ps_tmp4 ? __ps_tmp4 : "")); size_t _qn = 3; const char *_qp; for (_qp = _qw; *_qp; _qp++) _qn += (*_qp == '\'') ? 5u : 1u; free(_cq21); _cq21 = malloc(_qn); if (!_cq21) { fputs("sh2c: out of memory\n", stderr); exit(127); } char *_qd = _cq21; *_qd++ = ' '; *_qd++ = '\''; for (_qp = _qw; *_qp; _qp++) { if (*_qp == '\'') { memcpy(_qd, "'\"'\"'", 5); _qd += 5; } else *_qd++ = *_qp; } *_qd++ = '\''; *_qd = 0; }
static char *_cq22;
{ const char *_qw = ((__ps_tmp3 ? __ps_tmp3 : "")); size_t _qn = 3; const char *_qp; for (_qp = _qw; *_qp; _qp++) _qn += (*_qp == '\'') ? 5u : 1u; free(_cq22); _cq22 = malloc(_qn); if (!_cq22) { fputs("sh2c: out of memory\n", stderr); exit(127); } char *_qd = _cq22; *_qd++ = ' '; *_qd++ = '\''; for (_qp = _qw; *_qp; _qp++) { if (*_qp == '\'') { memcpy(_qd, "'\"'\"'", 5); _qd += 5; } else *_qd++ = *_qp; } *_qd++ = '\''; *_qd = 0; }
static char *_cq23;
{ const char *_qw = ((__ps_tmp4 ? __ps_tmp4 : "")); size_t _qn = 3; const char *_qp; for (_qp = _qw; *_qp; _qp++) _qn += (*_qp == '\'') ? 5u : 1u; free(_cq23); _cq23 = malloc(_qn); if (!_cq23) { fputs("sh2c: out of memory\n", stderr); exit(127); } char *_qd = _cq23; *_qd++ = ' '; *_qd++ = '\''; for (_qp = _qw; *_qp; _qp++) { if (*_qp == '\'') { memcpy(_qd, "'\"'\"'", 5); _qd += 5; } else *_qd++ = *_qp; } *_qd++ = '\''; *_qd = 0; }
  static char *_cn24; free(_cn24); _cn24 = 0; if (asprintf(&_cn24, "%s%s%s%s%s%s%s%s%s%s%s%s", " __ps_tmp3=", _cs18, " ; { ( 'echo' 'a\nc\nb' | 'sort' ) >", _cq19, " ; } ; __ps_tmp4=", _cs20, " ; { ( 'echo' 'a\nb\nd' | 'sort' ) >", _cq21, " ; } ; 'diff'", _cq22, _cq23, " && { kill $(cat ${__ps_tmp3}.pid) 2>/dev/null; kill $(cat ${__ps_tmp4}.pid) 2>/dev/null; rm -f ${__ps_tmp3} ${__ps_tmp3}.pid ${__ps_tmp4} ${__ps_tmp4}.pid ; }") < 0) _cn24 = 0;
  return ((*rc = ({ fflush(stdout); fflush(stderr); pid_t _sb2; for (int _t = 0; (_sb2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _sr2 = 127; if (_sb2 == 0) { execvp("bash", (char*[]) {"bash", "-c", (char*)((_cn24) ? (_cn24) : ""), 0}); _exit(127); } else if (_sb2 > 0) { while (waitpid(_sb2, &_sr2, 0) < 0 && errno == EINTR) { } _sr2 = WIFEXITED(_sr2) ? WEXITSTATUS(_sr2) : 1; } _sr2; })) == 0);
}

int main(void) {
    int sh_status = 0;
    char* __ps_tmp0 = NULL;
    char* __ps_tmp1 = NULL;
    char* __ps_tmp2 = NULL;
    char* __ps_tmp5 = NULL;
    char* __ps_tmp6 = NULL;
    sh_status = 0;
    fputs("== Here-string with grep -o ==\n", stdout), sh_status = 0;
    _sh_grep_matches("some pattern here", "pattern", "", &sh_status);
    fputs("== Process substitution with comm ==\n", stdout), sh_status = 0;
    {
        { char *_old0 = __ps_tmp0; __ps_tmp0 = ({ const char *_sd0 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); int _fd = mkstemp(_mt); if (_fd >= 0) close(_fd); sh_status = (_fd >= 0) ? 0 : 1; (_fd >= 0) ? strdup(_mt) : strdup(""); })); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old0); }
        fflush(stdout);
        int _nfd1_1 = open((__ps_tmp0 ? __ps_tmp0 : ""), O_WRONLY|O_CREAT|O_TRUNC, 0666);
        int _sfd1_1 = dup(1);
        if (_nfd1_1 >= 0) { dup2(_nfd1_1, 1); close(_nfd1_1); }
        if (_nfd1_1 >= 0) {
            (sh_status = 0, fputs("a\nb\n", stdout));
        }
        {
        fflush(stdout);
        dup2(_sfd1_1, 1); close(_sfd1_1);
        }
        { char *_old2 = __ps_tmp1; __ps_tmp1 = ({ const char *_sd1 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); int _fd = mkstemp(_mt); if (_fd >= 0) close(_fd); sh_status = (_fd >= 0) ? 0 : 1; (_fd >= 0) ? strdup(_mt) : strdup(""); })); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; }); free(_old2); }
        fflush(stdout);
        int _nfd3_1 = open((__ps_tmp1 ? __ps_tmp1 : ""), O_WRONLY|O_CREAT|O_TRUNC, 0666);
        int _sfd3_1 = dup(1);
        if (_nfd3_1 >= 0) { dup2(_nfd3_1, 1); close(_nfd3_1); }
        if (_nfd3_1 >= 0) {
            (sh_status = 0, fputs("b\nc\n", stdout));
        }
        {
        fflush(stdout);
        dup2(_sfd3_1, 1); close(_sfd3_1);
        }
        char *_a4[] = {"comm", "-12", (__ps_tmp0 ? __ps_tmp0 : ""), (__ps_tmp1 ? __ps_tmp1 : ""), 0};
        sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep3; for (int _t = 0; (_ep3 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es3 = 127; if (_ep3 == 0) { execvp((_a4)[0], (_a4)); _exit(127); } else if (_ep3 > 0) { while (waitpid(_ep3, &_es3, 0) < 0 && errno == EINTR) { } _es3 = WIFEXITED(_es3) ? WEXITSTATUS(_es3) : 1; } _es3; });
        {
        FILE *_pc5f = NULL; char _pc5p[4096];
        if (__ps_tmp0) {
        snprintf(_pc5p, sizeof _pc5p, "%s.pid", __ps_tmp0);
        _pc5f = fopen(_pc5p, "r");
        if (_pc5f) {
        int _pc5n = 0;
        if (fscanf(_pc5f, "%d", &_pc5n) == 1 && _pc5n > 1 && _pc5n != getpid()) kill(_pc5n, 15);
        fclose(_pc5f);
        }
        unlink(__ps_tmp0);
        unlink(_pc5p);
        }
        }
        {
        FILE *_pc6f = NULL; char _pc6p[4096];
        if (__ps_tmp1) {
        snprintf(_pc6p, sizeof _pc6p, "%s.pid", __ps_tmp1);
        _pc6f = fopen(_pc6p, "r");
        if (_pc6f) {
        int _pc6n = 0;
        if (fscanf(_pc6f, "%d", &_pc6n) == 1 && _pc6n > 1 && _pc6n != getpid()) kill(_pc6n, 15);
        fclose(_pc6f);
        }
        unlink(__ps_tmp1);
        unlink(_pc6p);
        }
        }
    }
    fputs("== readarray/mapfile ==\n", stdout), sh_status = 0;
    {
        { char *_old7 = __ps_tmp2; __ps_tmp2 = ({ const char *_sd2 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); int _fd = mkstemp(_mt); if (_fd >= 0) close(_fd); sh_status = (_fd >= 0) ? 0 : 1; (_fd >= 0) ? strdup(_mt) : strdup(""); })); char *_xp2 = _sd2 ? strdup(_sd2) : 0; if (_sd2 && !_xp2) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp2; }); free(_old7); }
        fflush(stdout);
        int _nfd8_1 = open((__ps_tmp2 ? __ps_tmp2 : ""), O_WRONLY|O_CREAT|O_TRUNC, 0666);
        int _sfd8_1 = dup(1);
        if (_nfd8_1 >= 0) { dup2(_nfd8_1, 1); close(_nfd8_1); }
        if (_nfd8_1 >= 0) {
            (sh_status = 0, fputs("x\ny\n", stdout));
        }
        {
        fflush(stdout);
        dup2(_sfd8_1, 1); close(_sfd8_1);
        }
        { FILE *_mf = fopen((__ps_tmp2 ? __ps_tmp2 : ""), "r"); if (_mf) { static char *_mfl = 0; static size_t _mfl_cap = 0; while (getline(&_mfl, &_mfl_cap, _mf) > 0) { size_t _mn = strlen(_mfl); if (1) { while (_mn && (_mfl[_mn-1]=='\n'||_mfl[_mn-1]=='\r')) _mfl[--_mn]=0; } if (lines_len >= 1024) { fprintf(stderr, "sh2c: capacity exceeded: lines\n"); exit(127); } lines[lines_len++] = ({ const char *_sd3 = _mfl; char *_xp3 = _sd3 ? strdup(_sd3) : 0; if (_sd3 && !_xp3) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp3; }); } fclose(_mf); } }
        {
        FILE *_pc9f = NULL; char _pc9p[4096];
        if (__ps_tmp2) {
        snprintf(_pc9p, sizeof _pc9p, "%s.pid", __ps_tmp2);
        _pc9f = fopen(_pc9p, "r");
        if (_pc9f) {
        int _pc9n = 0;
        if (fscanf(_pc9f, "%d", &_pc9n) == 1 && _pc9n > 1 && _pc9n != getpid()) kill(_pc9n, 15);
        fclose(_pc9f);
        }
        unlink(__ps_tmp2);
        unlink(_pc9p);
        }
        }
    }
    static char *_jj10;
    { char **_ja = lines; size_t _jn = lines_len; size_t _jl = 1; for (size_t _ji = 0; _ji < _jn; _ji++) if (_ja[_ji]) _jl += strlen(_ja[_ji]) + 1; char *_jd = malloc(_jl); if (!_jd) { fputs("sh2c: out of memory\n", stderr); exit(127); } char *_jw = _jd; for (size_t _ji = 0; _ji < _jn; _ji++) { if (_ji) *_jw++ = ' '; if (!_ja[_ji]) continue; size_t _jw2 = strlen(_ja[_ji]); memcpy(_jw, _ja[_ji], _jw2); _jw += _jw2; } *_jw = 0; free(_jj10); _jj10 = _jd; }
    (sh_status = 0, printf("%s ", _jj10));
    fputs("\n", stdout), sh_status = 0;
    fputs("== More process substitution examples ==\n", stdout), sh_status = 0;
    (_sh_site_0(&sh_status) || (fputs("Files differ\n", stdout), sh_status = 0, 1));
    {
        { char *_old11 = __ps_tmp5; __ps_tmp5 = ({ const char *_sd4 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); int _fd = mkstemp(_mt); if (_fd >= 0) close(_fd); sh_status = (_fd >= 0) ? 0 : 1; (_fd >= 0) ? strdup(_mt) : strdup(""); })); char *_xp4 = _sd4 ? strdup(_sd4) : 0; if (_sd4 && !_xp4) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp4; }); free(_old11); }
        fflush(stdout);
        int _nfd12_1 = open((__ps_tmp5 ? __ps_tmp5 : ""), O_WRONLY|O_CREAT|O_TRUNC, 0666);
        int _sfd12_1 = dup(1);
        if (_nfd12_1 >= 0) { dup2(_nfd12_1, 1); close(_nfd12_1); }
        if (_nfd12_1 >= 0) {
            fputs("name1\nname2\n", stdout), sh_status = 0;
        }
        {
        fflush(stdout);
        dup2(_sfd12_1, 1); close(_sfd12_1);
        }
        { char *_old13 = __ps_tmp6; __ps_tmp6 = ({ const char *_sd5 = (char*)(({ char _mt[4096]; const char *_td = getenv("TMPDIR"); snprintf(_mt, sizeof _mt, "%s/tmp.XXXXXX", (_td && *_td) ? _td : "/tmp"); int _fd = mkstemp(_mt); if (_fd >= 0) close(_fd); sh_status = (_fd >= 0) ? 0 : 1; (_fd >= 0) ? strdup(_mt) : strdup(""); })); char *_xp5 = _sd5 ? strdup(_sd5) : 0; if (_sd5 && !_xp5) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp5; }); free(_old13); }
        fflush(stdout);
        int _nfd14_1 = open((__ps_tmp6 ? __ps_tmp6 : ""), O_WRONLY|O_CREAT|O_TRUNC, 0666);
        int _sfd14_1 = dup(1);
        if (_nfd14_1 >= 0) { dup2(_nfd14_1, 1); close(_nfd14_1); }
        if (_nfd14_1 >= 0) {
            fputs("value1\nvalue2\n", stdout), sh_status = 0;
        }
        {
        fflush(stdout);
        dup2(_sfd14_1, 1); close(_sfd14_1);
        }
        char *_a15[] = {"paste", (__ps_tmp5 ? __ps_tmp5 : ""), (__ps_tmp6 ? __ps_tmp6 : ""), 0};
        sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep4; for (int _t = 0; (_ep4 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es4 = 127; if (_ep4 == 0) { execvp((_a15)[0], (_a15)); _exit(127); } else if (_ep4 > 0) { while (waitpid(_ep4, &_es4, 0) < 0 && errno == EINTR) { } _es4 = WIFEXITED(_es4) ? WEXITSTATUS(_es4) : 1; } _es4; });
        {
        FILE *_pc16f = NULL; char _pc16p[4096];
        if (__ps_tmp5) {
        snprintf(_pc16p, sizeof _pc16p, "%s.pid", __ps_tmp5);
        _pc16f = fopen(_pc16p, "r");
        if (_pc16f) {
        int _pc16n = 0;
        if (fscanf(_pc16f, "%d", &_pc16n) == 1 && _pc16n > 1 && _pc16n != getpid()) kill(_pc16n, 15);
        fclose(_pc16f);
        }
        unlink(__ps_tmp5);
        unlink(_pc16p);
        }
        }
        {
        FILE *_pc17f = NULL; char _pc17p[4096];
        if (__ps_tmp6) {
        snprintf(_pc17p, sizeof _pc17p, "%s.pid", __ps_tmp6);
        _pc17f = fopen(_pc17p, "r");
        if (_pc17f) {
        int _pc17n = 0;
        if (fscanf(_pc17f, "%d", &_pc17n) == 1 && _pc17n > 1 && _pc17n != getpid()) kill(_pc17n, 15);
        fclose(_pc17f);
        }
        unlink(__ps_tmp6);
        unlink(_pc17p);
        }
        }
    }
    free(__ps_tmp5);
    free(__ps_tmp6);
    free(__ps_tmp0);
    free(__ps_tmp1);
    free(__ps_tmp2);
    return sh_status;
}