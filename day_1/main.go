package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func sum(array []int) int {
	result := 0
	for _, v := range array {
		result += v
	}
	return result
}

type Output struct {
	Index         int
	SumOfCalories int
}

func FindMaxAndMin(products []Output) (min Output, max Output) {
	min = products[0]
	max = products[0]
	for _, product := range products {
		if product.SumOfCalories > max.SumOfCalories {
			max = product
		}
		if product.SumOfCalories < min.SumOfCalories {
			min = product
		}
	}
	return min, max
}

func main() {
	data, err := os.ReadFile("./data/input.txt")
	check(err)

	items := strings.Split(string(data), "\n\n")

	var groups [][]int

	for _, item := range items {
		str := strings.Split(item, " ")
		var sliceOfIntegers []int

		for _, groupString := range str {
			for _, stringNumber := range strings.Split(groupString, "\n") {
				if stringNumber == "" {
					continue
				}

				intFromString, err := strconv.Atoi(stringNumber)
				check(err)

				sliceOfIntegers = append(sliceOfIntegers, intFromString)
			}
		}

		groups = append(groups, sliceOfIntegers)
	}

	var outputs []Output

	for index, group := range groups {
		sumOfCalories := sum(group)
		output := Output{
			Index:         index,
			SumOfCalories: sumOfCalories,
		}

		outputs = append(outputs, output)
	}

	_, max := FindMaxAndMin(outputs)

	fmt.Printf("Elf number %d has to most calories, %d\n", max.Index, max.SumOfCalories)
}
