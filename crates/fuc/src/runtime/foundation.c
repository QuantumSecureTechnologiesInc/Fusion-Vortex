#include <stdio.h>
#include <stdlib.h>

/**
 * Fusion Core Runtime
 * The bedrock for all Fusion applications.
 */

void* fu_malloc(size_t size) {
    void* p = malloc(size);
    if (!p) {
        fprintf(stderr, "FATAL: Fusion Runtime exhausted heap memory.\n");
        exit(1);
    }
    return p;
}

void fu_free(void* ptr) {
    if (ptr) free(ptr);
}

void fu_panic(const char* message) {
    fprintf(stderr, "FUSION PANIC: %s\n", message);
    exit(1);
}