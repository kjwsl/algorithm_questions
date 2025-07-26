package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

type Instruction struct {
	direction byte
	distance  int
}

func parse_input(input string) []Instruction {
	i := 0

	var instructions []Instruction

	for i < len(input) {
		direction := input[i]

		i += 1

		comma_idx := i

		for comma_idx < len(input) && input[comma_idx] != ',' {
			comma_idx += 1
		}

		distance, err := strconv.Atoi(input[i:comma_idx])
		if err != nil {
			log.Fatal(err)
		}

		instructions = append(instructions, Instruction{direction, distance})
		i = comma_idx + 2
	}

	return instructions
}

func part1(input string) int {
	instructions := parse_input(input)
	x := 0
	y := 0

	directions := [4]byte{'N', 'E', 'S', 'W'}
	current_direction := 0

	for i := 0; i < len(instructions); i++ {
		inst := &instructions[i]

		switch inst.direction {
		case 'L':
			current_direction = (current_direction + 3) % 4
		case 'R':
			current_direction = (current_direction + 1) % 4
		default:
			log.Fatalf("Unknown direction: %v", inst.direction)
		}

		switch directions[current_direction] {
		case 'N':
			y += inst.distance
		case 'S':
			y -= inst.distance
		case 'E':
			x += inst.distance
		case 'W':
			x -= inst.distance
		default:
			log.Fatalf("Unknown direction: %v", inst.direction)
		}

	}

	return int(math.Abs(float64(x)) + math.Abs(float64(y)))
}

func main() {
	file, err := os.Open("../sample.txt")
	if err != nil {
		log.Fatalf("Error opening file: %v", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var sample string
	for scanner.Scan() {
		sample = scanner.Text()
		fmt.Println(sample)
	}

	fmt.Println("Part 1:", part1(sample))

}
