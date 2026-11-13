#include <stdio.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>


static int _stg0_0(void) {
  fputs("alpha beta\n", stdout);
  return 0;
}

int main(void) {
    int sh_status = 0;
    fputs("== Subshell ==\n", stdout);
    {
        fputs("inside-subshell\n", stdout);
    }
    fputs("== Simple pipeline ==\n", stdout), sh_status = 0;
    char *_a0_1[] = {"grep", "beta", 0};
    ({ int _pf0[2]; int _ok0 = 1; for (int _k = 0; _k < 1; _k++) if (pipe(_pf0 + 2*_k) != 0) { _ok0 = 0; break; } if (!_ok0) { sh_status = 127; } else { fflush(stdout); fflush(stderr); pid_t _cp0[2]; for (int _t = 0; (_cp0[0] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp0[0] == 0) { dup2(_pf0[1], 1); for (int _k = 0; _k < 1; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } int _tr = _stg0_0(); fflush(stdout); _exit(_tr); } for (int _t = 0; (_cp0[1] = fork()) < 0 && (errno == EAGAIN || errno == ENOMEM || errno == EINTR) && _t < 1000; _t++) usleep(1000); if (_cp0[1] == 0) { dup2(_pf0[0], 0); for (int _k = 0; _k < 1; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } execvp(_a0_1[0], _a0_1); _exit(127); } for (int _k = 0; _k < 1; _k++) { close(_pf0[2*_k]); close(_pf0[2*_k+1]); } int _st0 = 0; for (int _k = 0; _k < 2; _k++) { int _s = 0; while (waitpid(_cp0[_k], &_s, 0) < 0 && errno == EINTR) { } if (_k == 1) _st0 = _s; } sh_status = WIFEXITED(_st0) ? WEXITSTATUS(_st0) : 1; } (sh_status == 0); });
    printf("exit: %d\n", sh_status), sh_status = 0;
    return sh_status;
}