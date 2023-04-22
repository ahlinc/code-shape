#include <stdlib.h>

static void *ts_malloc_default(size_t size);
static void *ts_calloc_default(size_t count, size_t size);
static void *ts_realloc_default(void *buffer, size_t size);

void *(*ts_current_malloc)(size_t) = ts_malloc_default;
void *(*ts_current_calloc)(size_t, size_t) = ts_calloc_default;
void *(*ts_current_realloc)(void *, size_t) = ts_realloc_default;
void (*ts_current_free)(void *) = free;

void foo() {}
int bar(int a) {}
