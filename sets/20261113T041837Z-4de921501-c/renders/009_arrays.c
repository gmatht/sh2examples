#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>

static char *arr[3] = {0};
static size_t arr_len;
static char *map_k[1024] = {0};
static char *map_v[1024] = {0};
static size_t map_n;

char* k = NULL;
char* x = NULL;

static int _stg1_0(void) {
for (size_t _ai_map = 0; _ai_map < map_n; _ai_map++) {
    k = map_k[_ai_map];
    printf("%s => %s\n", ((char*)(k) ? (char*)(k) : ""), (char*)({ const char *_gk2 = (k ? k : ""); const char *_gv2 = ""; if (_gk2) for (size_t _gi2 = 0; _gi2 < (size_t)(map_n); _gi2++) if ((map_k)[_gi2] && strcmp((map_k)[_gi2], _gk2) == 0) { _gv2 = (map_v)[_gi2] ? (map_v)[_gi2] : ""; break; } _gv2; }));
}
  return 0;
}

int main(void) {
    int sh_status = 0;
    fputs("== Indexed arrays ==\n", stdout);
    size_t _ai0 = 0;
    arr_len = 0;
    arr[_ai0] = ({ const char *_sd0 = (char*)("one"); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; });
    arr_len = ++_ai0;
    arr[_ai0] = ({ const char *_sd1 = (char*)("two"); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; });
    arr_len = ++_ai0;
    arr[_ai0] = ({ const char *_sd2 = (char*)("three"); char *_xp2 = _sd2 ? strdup(_sd2) : 0; if (_sd2 && !_xp2) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp2; });
    arr_len = ++_ai0;
    printf("%s\n", (char*)(((1 < arr_len && arr[1]) ? arr[1] : "")));
    printf("%lld\n", (long long)(arr_len));
    for (size_t _ai_arr = 0; _ai_arr < arr_len; _ai_arr++) {
        x = arr[_ai_arr];
        ( printf("%s ", (x ? x : "")));
    }
    fputs("\n", stdout);
    fputs("== Associative arrays ==\n", stdout);
    { map_k[0] = "foo"; map_v[0] = "bar"; map_n = 1; }
    { map_k[1] = "answer"; map_v[1] = "42"; map_n = 2; }
    { map_k[2] = "two"; map_v[2] = "1 + 1"; map_n = 3; }
    printf("%s\n", (char*)({ const char *_gk3 = "foo"; const char *_gv3 = ""; if (_gk3) for (size_t _gi3 = 0; _gi3 < (size_t)(map_n); _gi3++) if ((map_k)[_gi3] && strcmp((map_k)[_gi3], _gk3) == 0) { _gv3 = (map_v)[_gi3] ? (map_v)[_gi3] : ""; break; } _gv3; }));
    printf("%s\n", (char*)({ const char *_gk4 = "answer"; const char *_gv4 = ""; if (_gk4) for (size_t _gi4 = 0; _gi4 < (size_t)(map_n); _gi4++) if ((map_k)[_gi4] && strcmp((map_k)[_gi4], _gk4) == 0) { _gv4 = (map_v)[_gi4] ? (map_v)[_gi4] : ""; break; } _gv4; })), sh_status = 0;
    char *_a1_1[] = {"sort", 0};
    ({ int _pf1[2]; int _ok1 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf1 + 2*_k) != 0) { _ok1 = 0; break; } if (!_ok1) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp1[2]; for (int _t = 0; (_cp1[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp1[0] == 0) { dup2(_pf1[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf1[2*_k]); close(_pf1[2*_k+1]); } int _tr = _stg1_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp1[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp1[1] == 0) { dup2(_pf1[0], 0); for (int _k = 0; _k < 1; _k++) { close(_pf1[2*_k]); close(_pf1[2*_k+1]); } execvp(_a1_1[0], _a1_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf1[2*_k]); close(_pf1[2*_k+1]); } int _st1 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp1[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st1 = _s; } sh_status = WIFEXITED(_st1) ? WEXITSTATUS(_st1) : 1; } (sh_status == 0); });
    return sh_status;
}