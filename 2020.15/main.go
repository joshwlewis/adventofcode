package main

import "fmt"

func main() {
	ns := []int{16, 1, 0, 18, 12, 14, 19}
	p2 := Say(2020, ns)
	fmt.Println("2020th number", p2)
	p3 := Say(30000000, ns)
	fmt.Println("30000000th number", p3)
}

func Say(i int, nums []int) (num int) {
	cache := make(map[int]int)
	curNum := nums[0]
	var prevNum int
	for j := 2; j <= i; j++ {
		prevNum = curNum
		if j <= len(nums) {
			curNum = nums[j-1]
		} else {
			if jPrev, ok := cache[prevNum]; ok {
				curNum = j - jPrev - 1
			} else {
				curNum = 0
			}
		}
		cache[prevNum] = j - 1
	}
	return curNum
}
