package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

const PATH = "../input/011.txt"

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)
	line, _, err := reader.ReadLine()

	if line == nil || err != nil {
		panic("ouch")
	}

	fields := strings.Fields(string(line))
	var numbers = []int{}

	for _, f := range fields {
		n, _ := strconv.Atoi(f)
		numbers = append(numbers, n)
	}

	max_i := 75

	var cache = make(map[[2]int]int)
	count := 0

	t := time.Now()
	for _, n := range numbers {
		count += get_count(n, 0, max_i, &cache)
	}

	fmt.Println(count, time.Since(t))
}

func get_count(n int, level int, max_level int, cache *map[[2]int]int) int {
	if level == max_level {
		return 1
	}
	level_d := max_level - level
	cached, ok := (*cache)[[2]int{n, level_d}]
	if ok {
		return cached
	}

	ch := get_next(n)
	out := 0
	for _, c := range ch {
		out += get_count(c, level+1, max_level, cache)
	}
	(*cache)[[2]int{n, level_d}] = out
	return out
}

func get_next(n int) []int {
	if n == 0 {
		return []int{1}
	}
	l := digits(n)
	if l%2 == 1 {
		return []int{2024 * n}
	}

	d := int(math.Pow10(l / 2))
	return []int{n / d, n % d}
}

func digits(n int) int {
	return int(math.Floor(math.Log10(float64(n)) + 1.))
}
