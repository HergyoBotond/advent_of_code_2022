package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func safety(arr []int) bool {
    for i := 0; i < len(arr); i++ {
        if i + 1 < len(arr)-1 {
            fmt.Println(arr[i])
            if math.Abs(float64(arr[i] - arr[i+1])) >= 3 && math.Abs(float64(arr[i] - arr[i+1])) > 0 {
                continue
            } else {
                return false
            }
        }
    }
    return true
}

func main() {
    f, err := os.Open("test.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()

    safe := 0

    scanner := bufio.NewScanner(f)

    for scanner.Scan() {
        strings := strings.Fields(scanner.Text())
        nums := []int{}

        for i := 0; i < len(strings); i++ {
            num, _ := strconv.Atoi(strings[i])
            nums = append(nums, num)
        }

        if safety(nums) {
            safe++
        }
    }
        fmt.Println(safe)
}
