package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"math"
	"sort"
	"strconv"
	"strings"
)

func part1(input string) int {
	dsum := 0
	lines := strings.Split(input, "\n")

	left := []int{}
	right := []int{}

	for i, line := range lines {
		tokens := strings.SplitN(line, "   ", 2)
		if len(tokens) != 2 {
			continue
		}

		num, err := strconv.Atoi(tokens[0])
		if err != nil {
			log.Fatalf("Invalid number at line %d: %s", i+1, tokens[0])
			continue
		}
		left = append(left, num)

		num, err = strconv.Atoi(tokens[1])
		if err != nil {
			log.Fatalf("Invalid number at line %d: %s", i+1, tokens[1])
			continue
		}
		right = append(right, num)
	}

	sort.Ints(left)
	sort.Ints(right)

	for i := range left {
		diff := math.Abs(float64(left[i] - right[i]))
		dsum += int(diff)
	}

	return dsum
}

func part2(input string) int {
	lines := strings.Split(input, "\n")
	left := []int{}
	right := make(map[int]int)
	score := 0

	for i, line := range lines {
		tokens := strings.SplitN(line, "   ", 2)
		if len(tokens) != 2 {
			continue
		}

		num, err := strconv.Atoi(tokens[0])
		if err != nil {
			log.Fatalf("Invalid number at line %d: %s", i+1, tokens[0])
		}
		left = append(left, num)

		num, err = strconv.Atoi(tokens[1])
		if err != nil {
			log.Fatalf("Invalid number at line %d: %s", i+1, tokens[1])
		}
		right[num]++
	}

	for _, num := range left {
		score += num * right[num]
	}

	return score
}

func main() {
	filePath := "sample.txt"

	data, err := ioutil.ReadFile(filePath)
	if err != nil {
		log.Fatalf("Error reading file: %v", err)
	}

	part1 := part1(string(data))
	fmt.Printf("Part 1: %d\n", part1)

	part2 := part2(string(data))
	fmt.Printf("Part 2: %d\n", part2)
}
