package main

import "fmt"

func main() {
	ns := []int{16, 1, 0, 18, 12, 14, 19}
	// ns := []int{3,1,2}
	// p1 := Say(10, ns)
	// fmt.Println("10th number", p1)
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
		cache[prevNum] = j-1
	}
	return curNum
}

func say(i int, nums []int, cache map[int]int) (int) {
	if i == 1 {
		return nums[i-1]
	}
	var curNum, prevNum int
	prevNum = say(i-1, nums, cache)
	if i <= len(nums) {
		curNum = nums[i-1]
	} else {
		if iPrev, ok := cache[prevNum]; ok {
			curNum = i - iPrev - 1
		} else {
			curNum = 0
		}
	}
	cache[prevNum] = i-1
	// fmt.Println("i/cur/cache", i, curNum, cache)
	return curNum
}
