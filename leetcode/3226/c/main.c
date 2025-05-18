#include <stdint.h>

int minChanges(int n, int k) {
    if (n == k) {
        return 0;
    } else if (n < k) {
        return -1;
    }

    int cnt = 0;

    for (int i = 0; i < 31; i++) {
        int bit = 1 << i;
        if ((n & bit) != 0 && (k & bit) == 0) {
            cnt++;
        } else if ((n & bit) == 0 && (k & bit) != 0) {
            return -1;
        }
    }

    return cnt;
}
