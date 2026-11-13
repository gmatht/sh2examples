#include <stdio.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <glob.h>


int main(void) {
    int sh_status = 0;
    fputs("== Basic brace expansion ==\n", stdout);
    fputs("1 2 3 4 5\n", stdout);
    fputs("a b c\n", stdout);
    fputs("00 02 04\n", stdout);
    fputs("== Advanced brace expansion ==\n", stdout);
    fputs("a1 a2 a3 b1 b2 b3 c1 c2 c3\n", stdout);
    fputs("1 3 5 7 9\n", stdout);
    fputs("a d g j m p s v y\n", stdout);
    fputs("== Practical examples ==\n", stdout), sh_status = 0;
    ({ { FILE *_tf = fopen("file_001.txt", "a"); if (_tf) fclose(_tf); } { FILE *_tf = fopen("file_002.txt", "a"); if (_tf) fclose(_tf); } { FILE *_tf = fopen("file_003.txt", "a"); if (_tf) fclose(_tf); } { FILE *_tf = fopen("file_004.txt", "a"); if (_tf) fclose(_tf); } { FILE *_tf = fopen("file_005.txt", "a"); if (_tf) fclose(_tf); } sh_status = 0; (sh_status == 0); });
    char *_av0[1024]; int _an0 = 0;
    _av0[_an0++] = "ls";
    glob_t _gg0_0 = {0}; const char *_pg0_0 = "file_*.txt"; int _gm0_0 = _an0;
    if (glob(_pg0_0, 0, 0, &_gg0_0) == 0) { for (size_t _gi = 0; _gi < _gg0_0.gl_pathc && _an0 < 1022; _gi++) _av0[_an0++] = _gg0_0.gl_pathv[_gi]; }
    if (_an0 == _gm0_0) _av0[_an0++] = (char*)_pg0_0;
    _av0[_an0] = 0;
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep2; for (int _t = 0; (_ep2 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es2 = 127; if (_ep2 == 0) { execvp((_av0)[0], (_av0)); _exit(127); } else if (_ep2 > 0) { while (waitpid(_ep2, &_es2, 0) < 0 && errno == EINTR) { } _es2 = WIFEXITED(_es2) ? WEXITSTATUS(_es2) : 1; } _es2; });
    globfree(&_gg0_0);
    (sh_status == 0);
    char *_av1[1024]; int _an1 = 0;
    _av1[_an1++] = "rm";
    glob_t _gg1_0 = {0}; const char *_pg1_0 = "file_*.txt"; int _gm1_0 = _an1;
    if (glob(_pg1_0, 0, 0, &_gg1_0) == 0) { for (size_t _gi = 0; _gi < _gg1_0.gl_pathc && _an1 < 1022; _gi++) _av1[_an1++] = _gg1_0.gl_pathv[_gi]; }
    if (_an1 == _gm1_0) _av1[_an1++] = (char*)_pg1_0;
    _av1[_an1] = 0;
    sh_status = ({ fflush(stdout); fflush(stderr); pid_t _ep3; for (int _t = 0; (_ep3 = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); int _es3 = 127; if (_ep3 == 0) { execvp((_av1)[0], (_av1)); _exit(127); } else if (_ep3 > 0) { while (waitpid(_ep3, &_es3, 0) < 0 && errno == EINTR) { } _es3 = WIFEXITED(_es3) ? WEXITSTATUS(_es3) : 1; } _es3; });
    globfree(&_gg1_0);
    (sh_status == 0);
    printf("brace_expand: %s\n", "1 2 3 4 5"), sh_status = 0;
    return sh_status;
}