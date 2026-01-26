#include <stdio.h>
#include <stdlib.h>

void panic(const char* msg) {
    if (msg) {
        fprintf(stderr, "Runtime Panic: %s\n", msg);
    } else {
        fprintf(stderr, "Runtime Panic\n");
    }
    abort();
}
