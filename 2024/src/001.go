package main

import (
        "bufio"
        "fmt"
        "math"
        "sort"
        "strconv"
        "strings"
        "os"
)

const PATH = "../input/001.txt"

func main() {
        file, err := os.Open(PATH)
        if err != nil {
                fmt.Println("Error", err)
                return
        }

        defer file.Close()

        scanner := bufio.NewScanner(file)

        var left []int
        var right []int
        count := make(map[int]int)

        for scanner.Scan() {
                line := scanner.Text()
                parts := strings.Fields(line)

                l, _ := strconv.Atoi(parts[0])
                r, _ := strconv.Atoi(parts[1])

                left = append(left, l)
                right = append(right, r)

                _, ok := count[r]
                if !ok {
                        count[r] = 1
                } else {
                        count[r] += 1
                }
                
        }
        sort.Sort(sort.IntSlice(left))
        sort.Sort(sort.IntSlice(right))

        // fmt.Println(left)
        // fmt.Println(right)
        // fmt.Println(count)

        dist := 0
        for i := 0; i < len(left); i++ {
                dist += int(math.Abs(float64(right[i]) - float64(left[i])))
        }
        // 1
        fmt.Println(dist)

        v := 0
        for i := 0; i < len(left); i++ {
                c := count[left[i]]  
                v += left[i] * c
        }
        fmt.Println(v)
}
