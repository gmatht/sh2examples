#include <stdio.h>
#include <stdlib.h>

char* maybe = NULL;

int main(void) {
    fputs("== Case modification in parameter expansion ==\n", stdout);
    fputs("WORLD\n", stdout);
    fputs("world\n", stdout);
    fputs("World\n", stdout);
    fputs("== Advanced parameter expansion ==\n", stdout);
    fputs("013_param_expansion_file.txt\n", stdout);
    fputs("/tmp\n", stdout);
    fputs("aXXa\n", stdout);
    fputs("== More parameter expansion ==\n", stdout);
    fputs(" world\n", stdout);
    fputs("hello \n", stdout);
    fputs("hell0 w0rld\n", stdout);
    fputs("== Default values ==\n", stdout);
    maybe = NULL;
    printf("%s\n", ((char*)(((((maybe ? maybe : "")) && ((maybe ? maybe : ""))[0]) ? ((maybe ? maybe : "")) : ("default"))) ? (char*)(((((maybe ? maybe : "")) && ((maybe ? maybe : ""))[0]) ? ((maybe ? maybe : "")) : ("default"))) : ""));
    printf("%s\n", ((char*)((maybe = ((((maybe ? maybe : "")) && ((maybe ? maybe : ""))[0]) ? ((maybe ? maybe : "")) : ("default")), ((((maybe ? maybe : "")) && ((maybe ? maybe : ""))[0]) ? ((maybe ? maybe : "")) : ("default")))) ? (char*)((maybe = ((((maybe ? maybe : "")) && ((maybe ? maybe : ""))[0]) ? ((maybe ? maybe : "")) : ("default")), ((((maybe ? maybe : "")) && ((maybe ? maybe : ""))[0]) ? ((maybe ? maybe : "")) : ("default")))) : ""));
    printf("%s\n", ((char*)(((((maybe ? maybe : "")) && ((maybe ? maybe : ""))[0]) ? ((maybe ? maybe : "")) : (fprintf(stderr, "%s\n", "error"), exit(1), (char*)0))) ? (char*)(((((maybe ? maybe : "")) && ((maybe ? maybe : ""))[0]) ? ((maybe ? maybe : "")) : (fprintf(stderr, "%s\n", "error"), exit(1), (char*)0))) : ""));
    return 0;
}