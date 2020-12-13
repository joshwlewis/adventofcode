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

	time, busIDs, busOffs, err := ReadSchedule(f)
	check(err)
	fmt.Println("Bus Input Data", time, busIDs, busOffs)

	bID, wait := NextBus(time, busIDs)
	fmt.Printf("Wait %d minutes for Bus %d. Prod: %d\n", wait, bID, wait*bID)

	contestTime := ContestTime(busIDs, busOffs)
	fmt.Println("Contest Time:", contestTime)
}

func NextBus(dTime int, bIDs []int) (int, int) {
	var minWait, minID int
	if len(bIDs) > 0 {
		minID = bIDs[0]
		minWait = GetWait(dTime, bIDs[0])
	}
	for _, bID := range bIDs {
		if GetWait(dTime, bID) < minWait {
			minID = bID
			minWait = GetWait(dTime, bID)
		}
	}
	return minID, minWait
}

func ContestTime(busIDs []int, busOffs []int) (t int) {
	T: for {
		tStep := 1
		for i, busOff := range busOffs {
			if (t + busOff) % busIDs[i] != 0 {
				t+=tStep
				continue T
			}
			tStep *= busIDs[i]
		}
		return
	}
}

func GetWait(dTime, bID int) int {
	busCount := (dTime / bID) + 1
	nextBusTime := bID * busCount
	return nextBusTime - dTime
}

func ReadSchedule(r io.Reader) (dTime int, busIds []int, busOffsets []int, err error) {
	scnr := bufio.NewScanner(r)
	for scnr.Scan() {
		if dTime == 0 {
			dTime, err = strconv.Atoi(strings.TrimSpace(scnr.Text()))
			if err != nil {
				return
			}
			continue
		}
		buses := strings.Split(strings.TrimSpace(scnr.Text()), ",")
		for i, bus := range buses {
			if bus == "x" {
				continue
			}
			var busId int
			busId, err = strconv.Atoi(bus)
			if err != nil {
				return
			}
			busIds = append(busIds, busId)
			busOffsets = append(busOffsets, i)
		}
	}
	err = scnr.Err()
	return
}

func check(err error) {
	if err != nil {
		log.Fatalf("unexpected error: %+v", err)
	}
}
