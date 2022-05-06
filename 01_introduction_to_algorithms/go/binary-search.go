package main

import "fmt"

var (
	searchList = []int {1, 2, 3, 4, 5, 6, 7, 8}
	searchValue = 7
)

func BinarySearch(list []int, value int) int {
	var (
		low = 0
		high = len(list) - 1
	)

	for low <= high {
		var (
			mid = (low + high) / 2
			guess = list[mid]
		)

		if guess == value {
			return mid
		}

		if guess < value {
			low = mid + 1
		} else {
			high = mid - 1
		}
	}

	return -1
}

func main() {
	fmt.Println("Binary Search")

	fmt.Println("List: ", searchList)
	fmt.Println("Search value: ", searchValue)

	result := BinarySearch(searchList, searchValue)
	wrong := BinarySearch(searchList, 100000)

	fmt.Println("Result: ", result) // 6
	fmt.Println("Wrong: ", wrong) // -1
}
