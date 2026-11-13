#include <stdio.h>
#include <stdint.h>

const uint8_t LIMIT = 10;

int main(void) {
    long long i = 0;
    {
        i = 0;
        while ((i < LIMIT)) {
            static char _s1[32];
            snprintf(_s1, sizeof _s1, "%lld", (long long)(i));
            printf("%s", _s1);
            i++;
        }
    }
    fputs("\n", stdout);
    return 0;
}