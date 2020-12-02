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

	p1, err := part1(f)
	check(err)
	p2, err := part2(f)
	check(err)
	p1b, err := part1b(f)
	check(err)
	p2b, err := part2b(f)
	check(err)

	fmt.Println("Part 1:", p1)
	fmt.Println("Part 2:", p2)
	fmt.Println("Part 1b:", p1b)
	fmt.Println("Part 2b:", p2b)
}

func part1(f io.ReadSeeker) (ans int, err error) {
	var ipos int64
	for {
		ipos, err = f.Seek(ipos, io.SeekStart)
		check(err)

		iscnr := bufio.NewScanner(f)
		iscnf := func(data []byte, atEOF bool) (adv int, token []byte, err error) {
			adv, token, err = bufio.ScanLines(data, atEOF)
			ipos += int64(adv)
			return
		}
		iscnr.Split(iscnf)

		iscnd := iscnr.Scan()
		if !iscnd {
			break
		}
		istr := iscnr.Text()

		i, err := strconv.Atoi(strings.TrimSpace(istr))
		check(err)

		_, err = f.Seek(ipos, io.SeekStart)
		check(err)

		jscnr := bufio.NewScanner(f)
		for {
			jscnd := jscnr.Scan()
			if !jscnd {
				break
			}

			j, err := strconv.Atoi(strings.TrimSpace(jscnr.Text()))
			check(err)

			if i+j == 2020 {
				return i * j, nil
			}
		}
	}
	return 0, fmt.Errorf("No solution found")
}

func part2(f io.ReadSeeker) (ans int, err error) {
	var ipos int64
	for {
		var inum int
		ipos, inum, err = scan(f, ipos)
		if err == io.EOF {
			break
		}
		if err != nil {
			return 0, err
		}
		jpos := ipos
		for {
			var jnum int
			jpos, jnum, err = scan(f, jpos)
			if err == io.EOF {
				break
			}
			if err != nil {
				return 0, err
			}
			kpos := jpos
			for {
				var knum int
				kpos, knum, err = scan(f, kpos)
				if err == io.EOF {
					break
				}
				if err != nil {
					return 0, err
				}
				if inum+jnum+knum == 2020 {
					return inum*jnum*knum, nil
				}
			}
		}
	}
	return 0, fmt.Errorf("No solution found")
}

func part1b(f io.ReadSeeker) (ans int, err error) {
	return scanR(f, 0, []int{}, 2020, 2)
}
func part2b(f io.ReadSeeker) (ans int, err error) {
	return scanR(f, 0, []int{}, 2020, 3)
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

func scan(r io.ReadSeeker, start int64) (pos int64, num int, err error) {
	pos, err = r.Seek(start, io.SeekStart)
	if err != nil {
		return
	}

	scnr := bufio.NewScanner(r)
    scnf := func(data []byte, eof bool) (adv int, tkn []byte, err error) {
		adv, tkn, err = bufio.ScanLines(data, eof)
	    pos += int64(adv)
		return
	}
	scnr.Split(scnf)
	scnd := scnr.Scan()
	if !scnd {
		if scnr.Err() == nil {
			return pos, num, io.EOF
		}
		return pos, num, scnr.Err()
	}
	
	str := scnr.Text()
	num, err = strconv.Atoi(strings.TrimSpace(str))
	return
}
