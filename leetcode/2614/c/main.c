#include <math.h>
#include <stdbool.h>

#define max(a, b) (a) > (b) ? (a) : (b)
bool is_prime(int num) {
    if (num < 2) {
        return false;
    } else if (num == 2) {
        return true;
    }

    int upto = (int)(sqrt((double)num));

    for (int i = 2; i <= upto; i++) {
        if (num % i == 0) {
            return false;
        }
    }

    return true;
}

int diagonalPrime(int** nums, int numsSize, int* numsColSize) {
    int max = 0;

    for (int i = 0; i < numsSize; i++) {
        int idx1 = (numsSize + 1) * i;
        int idx2 = (numsSize * numsSize - 1) - (numsSize - 1) * (i + 1);
        int num1 = nums[idx1 / 3][idx1 % 3];
        int num2 = nums[idx2 / 3][idx2 % 3];

        if (is_prime(num1)) {
            max = max(num1, max);
        }

        if (is_prime(num2)) {
            max = max(num2, max);
        }
    }

    return max;
}
