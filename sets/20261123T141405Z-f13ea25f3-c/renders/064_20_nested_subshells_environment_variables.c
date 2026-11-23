#include <stdio.h>
#include <string.h>
#include <assert.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */
char DEBUG[2] = "";
char LOG_LEVEL[8] = "";

int main(void) {
    assert(strlen(DEBUG) <= 1);
    assert(strlen(LOG_LEVEL) <= 7);

    {
        char _sv_DEBUG[2];
        strcpy(_sv_DEBUG, DEBUG);
        char _sv_LOG_LEVEL[8];
        strcpy(_sv_LOG_LEVEL, LOG_LEVEL);

        strncpy(DEBUG, (char*)("1"), 2);
        strncpy(LOG_LEVEL, (char*)("verbose"), 8);
        {
            char _sv_DEBUG[2];
            strcpy(_sv_DEBUG, DEBUG);

            DEBUG[0] = '\0';
            printf("Inner: LOG_LEVEL=%s, DEBUG=%s\n", ((char*)(LOG_LEVEL) ? (char*)(LOG_LEVEL) : ""), ((char*)((((DEBUG) && (DEBUG)[0]) ? (DEBUG) : ("unset"))) ? (char*)((((DEBUG) && (DEBUG)[0]) ? (DEBUG) : ("unset"))) : ""));

            strcpy(DEBUG, _sv_DEBUG);
        }
        printf("Outer: LOG_LEVEL=%s, DEBUG=%s\n", ((char*)(LOG_LEVEL) ? (char*)(LOG_LEVEL) : ""), ((char*)(DEBUG) ? (char*)(DEBUG) : ""));

        strcpy(DEBUG, _sv_DEBUG);
        strcpy(LOG_LEVEL, _sv_LOG_LEVEL);
    }
    return 0;
}