#include <stdbool.h>
#include <stdio.h>
#include <string.h>
bool checkDistances(char* s, int* distance, int distanceSize) {
    int size = strlen(s);
    bool checked[26];
    for (int i = 0; i < size; i++) {
        char ch       = s[i];
        int idx       = ch - 'a';
        int curr_dist = distance[idx];
        int next_idx  = i + curr_dist + 1;

        if (checked[idx] || (next_idx < size && s[next_idx] == ch)) {
            checked[idx] = true;
        } else {
            return false;
        }
    }
    return true;
}

int main() {
    const char* str  = "zz";
    int distance[26] = { 0 };
    bool res         = checkDistances((char*)str, distance, 26);

    printf("%s", res ? "true" : "false");

    return 0;
}
