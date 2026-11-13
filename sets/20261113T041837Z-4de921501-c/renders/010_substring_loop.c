#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>

char i[21] = "";
const uint8_t len = 10;
const char s[11] = "HELLOWORLD";

int main(void) {
    int sh_status = 0;
    char* ch = NULL;
    assert(strlen(i) <= 20);

    strncpy(i, (char*)("0"), 21);
    { int _went = 0;
    int _sh_lrc0 = 0;
    while ((atoll(i) < len)) {
        _went = 1;
        static char *_v1; free(_v1); _v1 = 0;
        if (asprintf(&_v1, "%s", i) < 0) _v1 = 0;
        static char *_v2;
        { const char *_su = s; if (!_su) _su = ""; long long _o = (long long)atoll(_v1), _l = 1; size_t _n = strlen(_su); long long _b = _o < 0 ? (long long)_n + _o : _o; if (_b < 0) _b = 0; if (_b > (long long)_n) _b = (long long)_n; long long _e = _b + ((_l < 0) ? (long long)_n + _l : _l); if (_e > (long long)_n) _e = (long long)_n; if (_e < _b) _e = _b; size_t _on = (size_t)(_e - _b); char *_sn = malloc(_on + 1); if (!_sn) { fputs("sh2c: out of memory\n", stderr); exit(127); } memcpy(_sn, _su + _b, _on); _sn[_on] = 0; free(_v2); _v2 = _sn; }
        { char *_old3 = ch; ch = ({ const char *_sd0 = (char*)(_v2); char *_xp0 = _sd0 ? strdup(_sd0) : 0; if (_sd0 && !_xp0) { fputs("sh2c: out of memory\n", stderr); exit(127); } _xp0; }); free(_old3); }
        (sh_status = 0, printf("%s", (ch ? ch : "")));
        static char _s4[32];
        snprintf(_s4, sizeof _s4, "%lld", (long long)((atoll(i) + 1)));
        assert(strlen((char*)(_s4)) <= 20);
        strncpy(i, (char*)(_s4), 21);
        i[20] = '\0';
        _sh_lrc0 = sh_status;
    }
    if (!_went) { sh_status = 0; } else { sh_status = _sh_lrc0; }
    }
    fputs("\n", stdout), sh_status = 0;
    free(ch);
    return sh_status;
}