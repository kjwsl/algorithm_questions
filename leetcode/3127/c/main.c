#include <stdbool.h>

bool canMakeSquare(char** grid, int gridSize, int* gridColSize) {
    for (int i = 0; i < 2; i++) {
        for (int j = 0; j < 2; j++) {
            char chs[4] = {
                grid[i][j],
                grid[i][j + 1],
                grid[i + 1][j],
                grid[i + 1][j + 1],
            };

            int blk_cnt = 0;

            for (int k = 0; k < 4; k++) {
                if (chs[k] == 'B') {
                    blk_cnt++;
                }
            }

            if (blk_cnt != 2) {
                return true;
            }
        }
    }

    return false;
}
