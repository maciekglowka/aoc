package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const PATH = "../input/013.txt"

type Rule struct {
	A     [2]int
	B     [2]int
	prize [2]int
}

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	// reader := bufio.NewReader(file)
	scanner := bufio.NewScanner(file)
	var rules = []Rule{}

	for {
		if !scanner.Scan() {
			break
		}
		line_A := scanner.Text()
		if len(line_A) < 2 {
			continue
		}

		scanner.Scan()
		line_B := scanner.Text()
		scanner.Scan()
		line_P := scanner.Text()

		r := Rule{
			get_numbers(&line_A, "+"),
			get_numbers(&line_B, "+"),
			get_numbers(&line_P, "="),
		}
		rules = append(rules, r)
	}

	total := 0
	for _, r := range rules {
		total += find_cost(&r, 0, 100)
	}
	fmt.Println("First: ", total)

	total_b := 0
	for _, r := range rules {
		total_b += find_cost(&r, 10000000000000, -1)
	}
	fmt.Println("Second: ", total_b)
}

func get_numbers(s *string, deli string) [2]int {
	parts := strings.Split(*s, ",")
	lp := strings.Split(parts[0], deli)
	l, _ := strconv.Atoi(lp[1])
	rp := strings.Split(parts[1], deli)
	r, _ := strconv.Atoi(rp[1])
	return [2]int{l, r}
}

func find_cost(rule *Rule, prize_offset int, max_moves int) int {
	i := prize_offset*(rule.B[1]-rule.B[0]) + rule.prize[0]*rule.B[1] - rule.prize[1]*rule.B[0]
	ii := rule.A[0]*rule.B[1] - rule.A[1]*rule.B[0]

	if i%ii != 0 {
		return 0
	}

	a := i / ii
	iii := prize_offset + rule.prize[1] - rule.A[1]*a
	if iii%rule.B[1] != 0 {
		return 0
	}
	b := iii / rule.B[1]
	if max_moves > 0 && (a > max_moves || b > max_moves) {
		return 0
	}

	return 3*a + b
}
