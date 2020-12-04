package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	validKeyCount, validValCount, err := CountValidIds(f)
	check(err)

	fmt.Printf("Found %d valid ids by keys.\n", validKeyCount)
	fmt.Printf("Found %d valid ids by keys and vals.\n", validValCount)
}

func check(err error) {
	if err != nil {
		log.Fatalln(err)
	}
}

type ID map[string]string

func CountValidIds(r io.Reader) (int, int, error) {
	scnr := bufio.NewScanner(r)
	var keycount, valcount int
	var entry string
	for scnr.Scan() {
		text := scnr.Text()
		entry = fmt.Sprintf("%s\n%s", entry, text)
		if len(strings.TrimSpace(text)) == 0 {
			id, err := NewIdFromEntry(entry)
			if err != nil {
				return 0, 0, err
			}
			if id.IsValidByKeys() {
				keycount++
			}
			if id.IsValidByVals() {
				valcount++
			}
			entry = ""
		}
	}
	return keycount, valcount, nil
}

func NewIdFromEntry(entry string) (ID, error) {
	pairs := strings.Fields(entry)
	id := make(map[string]string)
	for _, pair := range pairs {
		sides := strings.Split(pair, ":")
		if len(sides) != 2 {
			return nil, fmt.Errorf("unexpected entry: %s", pair)
		}
		id[sides[0]] = sides[1]
	}
	return id, nil
}

var requiredKeys = []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

// var allowedKeys = append(requiredKeys, "cid")

func (id ID) IsValidByKeys() bool {
	for _, key := range requiredKeys {
		if id[key] == "" {
			return false
		}
	}
	return true
}

func (id ID) IsValidByVals() bool {
	if !id.IsValidByKeys() {
		return false
	}
	for k, v := range id {
		switch k {
		case "byr":
			if !validY(v, 1920, 2002) {
				return false
			}
		case "iyr":
			if !validY(v, 2010, 2020) {
				return false
			}
		case "eyr":
			if !validY(v, 2020, 2030) {
				return false
			}
		case "hgt":
			if !validHgt(v) {
				return false
			}
		case "hcl":
			if !validHcl(v) {
				return false
			}
		case "ecl":
			if !validEcl(v) {
			    return false
			}
		case "pid":
			if !validPid(v) {
				return false
			}
		}
	}
	return true
}

func validY(v string, min, max int) bool {
	year, err := strconv.Atoi(v)
	if err != nil {
		return false
	}
	if year < min || year > max {
		return false
	}
	return true
}

func validHgt(s string) bool {
	re := regexp.MustCompile(`^(\d+)([a-z]+)$`)
	matches := re.FindStringSubmatch(s)

	if len(matches) != 3 {
		return false
	}

	val, _ := strconv.Atoi(matches[1])
	unit := matches[2]
	switch unit {
	case "cm":
		if val >= 150 && val <= 193 {
			return true
		}
	case "in":
		if val >= 59 && val <= 76 {
			return true
		}
	}
	return false
}

func validEcl(s string) bool {
	for  _, vc := range []string{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"} {
		if s == vc {
			return true
		}
	}
	return false
}

func validHcl(s string) bool {
	re := regexp.MustCompile(`^#[a-f0-9]{6}$`)
	return re.Match([]byte(s))
}

func validPid(s string) bool {
	re := regexp.MustCompile(`^\d{9}$`)
	return re.Match([]byte(s))
}
