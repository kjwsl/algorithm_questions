/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * int guess(int num);
 */

#include <stdint.h>
int guess(int num);

int guessNumber(int n) {
    const int TIMEOUT = 1000;
    int try           = 0;
    uint64_t lo = 0, hi = n;
    while (try < TIMEOUT) {
        int mid    = (hi + lo) / 2;
        int status = guess(mid);
        if (status < 0) {
            hi = mid - 1;
        } else if (status > 0) {
            lo = mid + 1;
        } else {
            return mid;
        }
        try++;
    }
    return -1;
}
