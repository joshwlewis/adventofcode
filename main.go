package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	d, err := ParseData(f)
	check(err)

	invNum, err := d.FindInvalidNumber(25)
	check(err)

	fmt.Println("Invalid Number: ", invNum)
}

type Data []int

func ParseData(r io.Reader) (Data, error) {
	d := Data{}

	scnr := bufio.NewScanner(r)
	for scnr.Scan() {
		num, err := strconv.Atoi(strings.TrimSpace(scnr.Text()))
		if err != nil {
			return d, err
		}
		d = append(d, num)
	}
	err := scnr.Err()
	return d, err
}

func (d Data) FindInvalidNumber(preamble int) (int, error) {
i:
	for i, in := range d {
		if i <= preamble {
			continue i
		}
		for j, jn := range d[i-preamble : i] {
		k:
			for k, kn := range d[i-preamble: i] {
				if j == k {
					continue k
				}
				if jn+kn == in {
					continue i
				}
			}
		}
		return in, nil
	}
	return 0, fmt.Errorf("No invalid number found")
}

func check(err error) {
	if err != nil {
		log.Fatalf("Unexpected error: %+v", err)
	}
}
