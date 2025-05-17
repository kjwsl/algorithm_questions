/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
int* shortestToChar(char* s, char c, int* returnSize) {
    ssize_t size = strlen(s);
    int* res = malloc(sizeof(int) * size);
    int res_size = 0;
    int* idxs = malloc(sizeof(int) * size);
    int idxs_size = 0;

    *returnSize = size;

    for (int i = 0; i < size; i++) {
        if (s[i] == c) {
            idxs[idxs_size++] = i;
        }
    }

    for (int i = 0; i < size; i++) {
        int min = INT32_MAX;
        for (int j = 0; j < idxs_size; j++) {
            int distance = abs(idxs[j] - i);
            if (distance < min) {
                min = distance;
            }
        }

        res[res_size++] = min;
    }

    free(idxs);

    return res;
}

