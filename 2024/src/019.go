package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const PATH = "../input/019.txt"

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)

	var towels = []string{}

	line, _, _ := reader.ReadLine()

	for _, s := range strings.Split(string(line), ",") {
		s = strings.ReplaceAll(s, " ", "")
		towels = append(towels, s)
	}

	_, _, _ = reader.ReadLine()
	count := 0
	count_p2 := 0

	var found = map[string]int{}
	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}
		// patterns = append(patterns, string(line))
		pattern := string(line)
		res := is_possible(&pattern, &towels, &found)
		if res > 0 {
			count++
		}
		count_p2 += res
	}
	fmt.Println(count, count_p2)
}

func is_possible(pattern *string, towels *[]string, found *map[string]int) int {
	if _, exists := (*found)[*pattern]; exists {
		return (*found)[*pattern]
	}

	count := 0

	for _, towel := range *towels {
		if !strings.HasPrefix(*pattern, towel) {
			continue
		}
		if len(towel) == len(*pattern) {
			count++
			continue
		}
		subpattern := (*pattern)[len(towel):]
		count += is_possible(&subpattern, towels, found)
	}
	if count > 0 {
		(*found)[*pattern] = count
	}
	return count
}
