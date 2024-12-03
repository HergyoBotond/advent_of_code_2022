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

// Part 1
func bubbleSort(arr []int64) {
	for i := 0; i < len(arr); i++ {
		for j := 0; j < len(arr)-1-i; j++ {
			if arr[j] > arr[j+1] {
				arr[j], arr[j+1] = arr[j+1], arr[j]
			}
		}
	}
}

func quickSort(arr []int64, lo int64, hi int64) {
    if lo >= hi {
        return
    }

    var pivotIdx = partition(arr, lo, hi)

    quickSort(arr, lo, pivotIdx-1)
    quickSort(arr, pivotIdx+1, hi)
}

func partition(arr []int64, lo int64, hi int64) int64 {
    var pivot = arr[hi]
    var idx = lo - 1

    for i := lo; i < hi; i++ {
        if arr[i] <= pivot {
            idx++
            arr[i], arr[idx] = arr[idx], arr[i]
        }
    }

    idx++
    arr[hi], arr[idx] = arr[idx], pivot

    return idx
}

func getDiffs(arr1 []int64, arr2 []int64) int64 {
	allDiff := int64(0)
	if len(arr1) == len(arr2) {
		for i := 0; i < len(arr1); i++ {
			allDiff += int64(math.Abs(float64(arr2[i] - arr1[i])))
		}
	}
	return allDiff
}

// Part 2
func calcApp(arr1 []int64, arr2 []int64) int64 {
    nums := make(map[int64]int64)
    var score int64 = 0

    for i := 0; i < len(arr1); i++ {
        nums[arr1[i]] = 0
        for j := 0; j < len(arr2); j++ {
            if arr2[j] == arr1[i] {
                nums[arr1[i]]++
            }
        }
        score += nums[arr1[i]] * arr1[i]
    }
    return score
}


func main() {
	var col1 []int64
	var col2 []int64

	// open file
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	// close file at the end of reding it
	defer f.Close()

	// read file line by line using scanner
	scanner := bufio.NewScanner(f)

	for scanner.Scan() {
		// do something with the line here
		nums := strings.Fields(scanner.Text())

		if len(nums) >= 2 {
			num1, _ := strconv.ParseInt(nums[0], 10, 64)
			num2, _ := strconv.ParseInt(nums[1], 10, 64)
			col1 = append(col1, num1)
			col2 = append(col2, num2)
		}
	}

    // Part 1
	bubbleSort(col1)
    quickSort(col2, 0, int64(len(col2)-1))

	solution1 := getDiffs(col1, col2)
	solution2 := calcApp(col1, col2)

	fmt.Println(solution1)
	fmt.Println("---------Part 1------------")
	fmt.Println(solution2)
	fmt.Println("---------Part 2------------")

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
