#include <stdio.h>

char IFS[256] = " \t\n"; /* bash's startup IFS */
char* lang = NULL;

int main(void) {
    static const char* _for_lang_0[] = {"cs", "de", "es", "fi", "fr"};
    for (size_t _i_lang = 0; _i_lang < 5; _i_lang++) {
        lang = (char*)_for_lang_0[_i_lang];
        printf("%s\n", (char*)((lang ? lang : "")));
    }
    return 0;
}