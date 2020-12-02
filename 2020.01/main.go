package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	dir, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(dir, "input.txt"))
	check(err)

	p1, err := scanR(f, 0, []int{}, 2020, 2)
	check(err)

	p2, err := scanR(f, 0, []int{}, 2020, 3)
	check(err)

	fmt.Println("Part 1:", p1)
	fmt.Println("Part 2:", p2)
}

func scanR(r io.ReadSeeker, pos int64, nums []int, tgt int, max int) (prod int, err error) {
	pos, err = r.Seek(pos, io.SeekStart)
	if err != nil {
		return 0, err
	}

	scnr := bufio.NewScanner(r)
    scnf := func(data []byte, eof bool) (adv int, tkn []byte, err error) {
		adv, tkn, err = bufio.ScanLines(data, eof)
	    pos += int64(adv)
		return
	}

	scnr.Split(scnf)

	for {
		scnd := scnr.Scan()
		if !scnd {
			if scnr.Err() != nil {
				return 0, scnr.Err()
			}
	        break
		}

		str := scnr.Text()
		num, err := strconv.Atoi(strings.TrimSpace(str))
		if err != nil {
			return num, err
		}

		lnums := append(nums, num)
		if len(lnums) == max {
			sum := 0
			prod = 1
			for _, num := range lnums {
				sum += num
				prod *= num
			}
			if sum == tgt {
				return prod, nil
			}
		} else {
			prod, err = scanR(r, pos, lnums, tgt, max)
			if err != nil || prod != 0 {
				return prod, err
			}
		}
	}
	return 0, nil
}
