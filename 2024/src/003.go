package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

const MUL_REG = `mul\((\d{1,3}),(\d{1,3})\)`
const DO_REG = `mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)`
const PATH = "../input/003.txt"

func main() {
	b, _ := os.ReadFile(PATH)
	input := string(b)

	re := regexp.MustCompile(MUL_REG)
	matches := re.FindAllStringSubmatch(input, -1)

	total := 0
	for _, m := range matches {
		a, _ := strconv.Atoi(m[1])
		b, _ := strconv.Atoi(m[2])
		total += a * b
	}
	fmt.Println(total)

	re_do := regexp.MustCompile(DO_REG)
	matches_do := re_do.FindAllStringSubmatch(input, -1)
	active := true
	total_do := 0

	for _, m := range matches_do {
		if m[0] == "do()" {
			active = true
			continue
		}
		if m[0] == "don't()" {
			active = false
			continue
		}

		if active {
			a, _ := strconv.Atoi(m[1])
			b, _ := strconv.Atoi(m[2])
			total_do += a * b
		}
	}
	fmt.Println(total_do)
}
