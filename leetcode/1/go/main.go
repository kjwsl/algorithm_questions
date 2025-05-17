package main

import (
	"fmt"
)

func twoSum(nums []int, target int) []int {
	m := make(map[int]int)

	for i, num := range nums {
		m[num] = i
	}

	for i, num := range nums {
		needed := target - num
		if idx, ok := m[needed]; ok {
			if i != idx {
				return []int{i, idx}
			}
		}
	}
	return nil
}

func main() {
	res := twoSum([]int{3, 2, 4}, 6)
	fmt.Printf("res: %v\n", res)
}
