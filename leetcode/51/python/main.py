#!/usr/bin/env python3

from typing import List


class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        if n == 1:
            return [["Q"]]
        elif n < 4:
            return [[]]
        queen_pos = self._solve_n_queens(n, 0, [])

        res = []

        for pos in queen_pos:
            tmp = ["." for _ in range(n)]
            tmp[pos] = "Q"
            res.append(tmp)

        return res

    def _solve_n_queens(self, n: int, curr_idx: int, track: List[int]) -> List[int]:
        if track[n] == None:
            candidates = [i for i in range(n) if i not in track]

        for cand in candidates:
            track
