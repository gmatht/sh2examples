#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

static char *__attribute__((returns_nonnull)) _sh_grow(char **b, size_t *cap, size_t need) {
  if (*b && need <= *cap) return *b;
  size_t ncap = *cap ? *cap : 256;
  if (ncap < need) ncap = (need - ncap <= ncap) ? ncap + ncap : need;
  char *np = (char*)realloc(*b, ncap);
  if (!np) { fprintf(stderr, "sh2c: out of memory\n"); exit(127); }
  *b = np; *cap = ncap;
  return np;
}
char* ch = NULL;
char i[21] = "";
char* len = NULL;
char* s = NULL;

static void print_chars(int argc, char **argv, int *_rcp, int _noecho) {
    s = ({ const char *_sd0 = (char*)(((1 < argc && argv[1]) ? argv[1] : "")); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; });
    strncpy(i, (char*)("0"), 21);
    len = ({ const char *_sd1 = (char*)(((2 < argc && argv[2]) ? argv[2] : "")); char *_xp1 = _sd1 ? strdup(_sd1) : 0; if (_sd1 && !_xp1) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp1; });
    { int _went = 0;
    int _sh_lrc0 = 0;
    while ((atoll(i) < atoll((len ? len : "")))) {
        _went = 1;
        static char *_v1 = 0; static size_t _v1_cap = 0;
        _sh_grow(&_v1, &_v1_cap, strlen((s ? s : "")) + 1); strcpy(_v1, (s ? s : ""));
        static char *_v2; free(_v2); _v2 = 0;
        if (asprintf(&_v2, "%s", i) < 0) _v2 = 0;
        static char *_v3;
        { const char *_su = _v1; if (!_su) _su = ""; long long _o = (long long)atoll(_v2), _l = 1; size_t _n = strlen(_su); long long _b = _o < 0 ? (long long)_n + _o : _o; if (_b < 0) _b = 0; if (_b > (long long)_n) _b = (long long)_n; long long _e = _b + ((_l < 0) ? (long long)_n + _l : _l); if (_e > (long long)_n) _e = (long long)_n; if (_e < _b) _e = _b; size_t _on = (size_t)(_e - _b); char *_sn = malloc(_on + 1); if (!_sn) { fputs("sh2c: out of memory\n", stderr); exit(127); } memcpy(_sn, _su + _b, _on); _sn[_on] = 0; free(_v3); _v3 = _sn; }
        ch = ({ const char *_sd2 = (char*)(_v3); char *_xp2 = _sd2 ? strdup(_sd2) : 0; if (_sd2 && !_xp2) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp2; });
        ((*_rcp) = 0, printf("%s", (ch ? ch : "")));
        static char _s4[32];
        snprintf(_s4, sizeof _s4, "%lld", (long long)((atoll(i) + 1)));
        assert(strlen((char*)(_s4)) <= 20);
        strncpy(i, (char*)(_s4), 21);
        i[20] = '\0';
        _sh_lrc0 = (*_rcp);
    }
    if (!_went) { (*_rcp) = 0; } else { (*_rcp) = _sh_lrc0; }
    }
    fputs("\n", stdout), (*_rcp) = 0;
}

int main(void) {
    int sh_status = 0;
    print_chars(3, (char*[]){"print_chars", "HELLOWORLD", "10"}, &sh_status, 1);
    return sh_status;
}