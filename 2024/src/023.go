package main

import (
	"bufio"
	"fmt"
	"maps"
	"os"
	"slices"
	"sort"
)

const PATH = "../input/023.txt"

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)

	net := make(map[[2]rune]map[[2]rune]bool)
	ts := make(map[[2]rune]bool)

	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}

		s := []rune(string(line))
		a := [2]rune{s[0], s[1]}
		b := [2]rune{s[3], s[4]}
		link(a, b, &net)
		link(b, a, &net)

		if a[0] == rune('t') {
			ts[a] = true
		}
		if b[0] == rune('t') {
			ts[b] = true
		}
	}

	// first
	tris := make(map[[3][2]rune]bool)

	for k, _ := range ts {
		for link, _ := range net[k] {
			for sublink, _ := range net[link] {
				if net[sublink][k] {
					v := [][2]rune{k, link, sublink}
					node_sort(&v)
					tris[[3][2]rune{v[0], v[1], v[2]}] = true
				}
			}
		}
	}
	fmt.Println(len(tris))

	// second

	var sets = [][][2]rune{}

	for k, v := range net {
		occurences := make(map[[2]rune]int)

		for kk, _ := range v {
			for kkk, _ := range net[kk] {
				if _, exists := occurences[kkk]; exists {
					occurences[kkk]++
				} else {
					occurences[kkk] = 1
				}
			}
		}

		var occ = [][3]byte{}
		for k, v := range occurences {
			occ = append(occ, [3]byte{byte(k[0]), byte(k[1]), byte(v)})
		}
		// that's unneccessary?
		sort.Slice(occ, func(i, ii int) bool {
			return occ[i][2] > occ[ii][2]
		})

		set := maps.Clone(v)
		set[k] = true

		for _, o := range occ {
			kkk := [2]rune{rune(o[0]), rune(o[1])}
			if !set[kkk] {
				continue
			}
			ss := net[kkk]
			ss[kkk] = true
			set = intersect(&set, &ss)
		}

		s := slices.Collect(maps.Keys(set))
		sets = append(sets, s)
	}

	sort.Slice(sets, func(i, ii int) bool {
		return len(sets[i]) < len(sets[ii])
	})

	for _, set := range sets {
		node_sort(&set)
		for _, v := range set {
			fmt.Printf("%c%c,", v[0], v[1])
		}
		fmt.Println("")
	}
}

func link(a, b [2]rune, net *map[[2]rune]map[[2]rune]bool) {
	if _, exists := (*net)[a]; !exists {
		(*net)[a] = map[[2]rune]bool{}
	}
	(*net)[a][b] = true
}

func node_sort(v *[][2]rune) {
	sort.Slice(*v, func(i, ii int) bool {
		if (*v)[i][0] == (*v)[ii][0] {
			return (*v)[i][1] < (*v)[ii][1]
		}
		return (*v)[i][0] < (*v)[ii][0]
	})
}

func intersect(a, b *map[[2]rune]bool) map[[2]rune]bool {
	// go == no hashsets LOL
	output := make(map[[2]rune]bool)
	for k, _ := range *a {
		if (*b)[k] {
			output[k] = true
		}
	}
	return output
}
