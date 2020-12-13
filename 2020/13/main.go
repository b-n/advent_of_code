package main

import (
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func check(e error) {
  if e != nil {
    log.Fatal(e)
    panic(e)
  }
}

func readFile(path string) (string) {
  dat, err := ioutil.ReadFile(path)
  check(err)
  return string(dat)
}

func safeAtoi(input string) int {
  output, e := strconv.Atoi(input)
  if e != nil { return 0 }
  return output
}

func next(time int, bus int) int {
  return time + (bus - time % bus)
}

func main() {
  input := readFile("./input.txt")

  data := strings.Split(strings.TrimRight(input, "\n"), "\n")

  //Challenge 1
  {
    time := safeAtoi(data[0])
    rawBuses := strings.Split(data[1], ",")
    //buses := make([]int, len(rawBuses))
    nextBus := -1
    for _, b := range rawBuses {
      bus := safeAtoi(b)
      if bus > 0 {
        if nextBus == -1 || next(time, bus) < next(time, nextBus) {
          nextBus = bus
        }
      }
    }

    log.Print((next(time, nextBus) - time) * nextBus)
  }

  //Challenge 2
  {
  }
}
