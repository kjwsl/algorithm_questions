#include <stdlib.h>

#define max(a, b) ((a) > (b) ? (a) : (b))

int findPoisonedDuration(int* timeSeries, int timeSeriesSize, int duration) {
    if (timeSeriesSize == 1) {
        return duration;
    }

    int elapsed = duration;
    int last_time = timeSeries[0] + duration;

    for (int i = 1; i < timeSeriesSize; i++) {
        int t = timeSeries[i];
        int next_last_time = t + duration;
        if (next_last_time <= last_time) {
            continue;
        }
        elapsed += next_last_time - max(last_time, t);
        last_time = next_last_time;
    }

    return elapsed;
}
