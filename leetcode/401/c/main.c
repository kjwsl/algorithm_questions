#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TIME_STR_LEN 6 // "h:mm\0"

void dfs(int leds, int pos, int bitmask, int turnedOn, int* count, char** res) {
    if (leds == turnedOn) {
        int hour = (bitmask >> 6) & 0xF; // upper 4 bits
        int min  = bitmask & 0x3F;       // lower 6 bits
        if (hour < 12 && min < 60) {
            res[*count] = malloc(TIME_STR_LEN);
            snprintf(res[*count], TIME_STR_LEN, "%d:%02d", hour, min);
            (*count)++;
        }
        return;
    }
    for (int i = pos; i < 10; i++) {
        dfs(leds + 1, i + 1, bitmask | (1 << i), turnedOn, count, res);
    }
}

char** readBinaryWatch(int turnedOn, int* returnSize) {
    // Maximum possible valid times is less than 720
    char** res = malloc(720 * sizeof(char*));
    int count = 0;
    dfs(0, 0, 0, turnedOn, &count, res);
    *returnSize = count;
    return res;
}

// Utility functions for testing
void print_strings(char** strings, int n) {
    printf("[");
    for (int i = 0; i < n; i++) {
        printf("%s", strings[i]);
        if (i < n - 1) printf(", ");
    }
    printf("]\n");
}

void free_combinations(char** combs, int n) {
    for (int i = 0; i < n; i++) free(combs[i]);
    free(combs);
}

int main() {
    int return_size;
    char** combs = readBinaryWatch(1, &return_size);
    print_strings(combs, return_size);
    free_combinations(combs, return_size);

    combs = readBinaryWatch(2, &return_size);
    print_strings(combs, return_size);
    free_combinations(combs, return_size);

    return 0;
}

