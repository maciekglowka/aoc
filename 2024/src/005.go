package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

const PATH = "../input/005.txt"

var after = map[int][]int{}

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)
	// after := make(map[int][]int)

	for {
		line, _, _ := reader.ReadLine()

		if len(line) == 0 {
			break
		}

		parts := strings.Split(string(line), "|")

		l, _ := strconv.Atoi(parts[0])
		r, _ := strconv.Atoi(parts[1])
		a, ok := after[l]
		if !ok {
			after[l] = []int{r}
		} else {
			after[l] = append(a, r)
		}
	}

	total := 0
	total_sorted := 0
	for {
		line, _, _ := reader.ReadLine()

		if line == nil {
			break
		}
		parts := strings.Split(string(line), ",")
		seq := make([]int, len(parts))
		for i, p := range parts {
			v, _ := strconv.Atoi(p)
			seq[i] = v
		}

		valid := true
		for i := 0; i < len(seq); i++ {
			if !validate_sequence(&seq, &after, i) {
				valid = false
				break
			}
		}
		if valid {
			total += seq[len(seq)/2]
		} else {
			slices.SortStableFunc(seq, cmp)
			total_sorted += seq[len(seq)/2]
		}
	}

	fmt.Println(total)
	fmt.Println(total_sorted)
}

func validate_sequence(seq *[]int, after *map[int][]int, start int) bool {
	key := (*seq)[start]
	a, ok := (*after)[key]
	if !ok {
		return true
	}
	for i := 0; i < start; i++ {
		if slices.Contains(a, (*seq)[i]) {
			return false
		}
	}
	return true
}

func cmp(a, b int) int {
	a_after, ok := after[a]
	if ok {
		if slices.Contains(a_after, b) {
			return -1
		}
	}
	return 0
}
