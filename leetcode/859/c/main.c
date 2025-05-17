#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
bool buddyStrings(char* s, char* goal) {
    int s_size    = strlen(s);
    int goal_size = strlen(goal);

    if (s_size != goal_size) {
        return false;
    }
    if (strcmp(s, goal) == 0) {
        int ch_cnt[26] = { 0 };
        for (int i = 0; i < s_size; i++) {
            int idx = s[i] - 'a';
            if (ch_cnt[idx] == 1) {
                return true;
            }
            ch_cnt[idx]++;
        }
    }

    int* diffs = (int*)calloc(s_size, sizeof(int));
    int diff_cnt = 0;
    for (int i = 0; i < s_size; i++) {
        if (s[i] != goal[i]) {
            diffs[diff_cnt++] = i;
            if (diff_cnt > 2) {
                goto clean;
            }
        }
    }

    if (diff_cnt == 2) {
        if (s[diffs[0]] == goal[diffs[1]] && s[diffs[1]] == goal[diffs[0]]) {
            return true;
        }
    }

clean:
    free(diffs);
    return false;
}
