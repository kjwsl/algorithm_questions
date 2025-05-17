

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DEFAULT_CAPACITY 4

typedef struct {
    int* pings;
    int size;
    int capacity;
} RecentCounter;

RecentCounter* recentCounterCreate() {
    RecentCounter* counter = malloc(sizeof(RecentCounter));
    counter->pings         = malloc(sizeof(int) * DEFAULT_CAPACITY);
    counter->capacity      = DEFAULT_CAPACITY;
    counter->size          = 0;

    return counter;
}

int recentCounterPing(RecentCounter* obj, int t) {
    // Resize if needed
    if (obj->size == obj->capacity) {
        int* tmp = realloc(obj->pings, sizeof(int) * obj->capacity * 2);
        if (tmp == NULL) {
            printf("Failed to resize counter\n");
            exit(-1);
        }
        obj->pings = tmp;
        obj->capacity *= 2;
    }

    obj->pings[obj->size++] = t;

    int cnt                 = 0;

    for (int i = obj->size - 1; i >= 0; i--) {
        // If a certain ping is recent
        if (obj->pings[i] >= t - 3000) {
            cnt++;
        } else {
            break;
        }
    }

    return cnt;
}

void recentCounterFree(RecentCounter* obj) {
    free(obj->pings);
    free(obj);
}

/**
 * Your RecentCounter struct will be instantiated and called as such:
 * RecentCounter* obj = recentCounterCreate();
 * int param_1 = recentCounterPing(obj, t);

 * recentCounterFree(obj);
*/
