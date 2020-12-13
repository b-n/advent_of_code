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
  return bus - time % bus
}

func main() {
  input := readFile("./input.txt")

  data := strings.Split(strings.TrimRight(input, "\n"), "\n")

  //Challenge 1
  {
    time := safeAtoi(data[0])
    rawBuses := strings.Split(data[1], ",")
    nextBus := -1
    for _, b := range rawBuses {
      bus := safeAtoi(b)
      if bus > 0 {
        if nextBus == -1 || next(time, bus) < next(time, nextBus) {
          nextBus = bus
        }
      }
    }

    log.Print((next(time, nextBus)) * nextBus)
  }

  //Challenge 2 - attemp 3/4
  if false {
    rawBuses := strings.Split(data[1], ",")
    buses := make([]int, len(rawBuses))
    busIndex := map[int]int{}
    for i, b := range rawBuses {
      buses[i] = safeAtoi(b)
      if buses[i] > 0 && i > 0 { busIndex[buses[i]] = i }
    }

    timeStamp := 0
    firstBus := buses[0]
    // This is bruteforce, and will pin your CPU for a long time
    // At least it gives you an output every x cycles so you know it's running
    for true {
      i := 0
      for bus, index := range busIndex {
        comes := next(timeStamp + index, bus)
        if bus != comes { break }
        i++
      }
      if (i == len(busIndex)) { break }
      if (timeStamp % (firstBus * 10000000) == 0) { log.Print(timeStamp) }
      timeStamp += firstBus
    }
    log.Print(timeStamp)
  }


  // Challenge 2 - attempt last final v2.0 [1] .docx
  {
    // Idea/implementation from 
    // https://www.reddit.com/r/adventofcode/comments/kc4njx/2020_day_13_solutions/gfo4b1z?utm_source=share&utm_medium=web2x&context=3
    // Understanding: (assume buses 67, 7, 59, 61)
    // - We are checking for busses that work well together, not searching the timespace
    // - We *= the jump each time, because there will be no busses in that range that are common factors
    // - We search until we hit a % 0, because we're looking for the lowest common value
    rawBuses := strings.Split(data[1], ",")
    jump := 1
    busIndex := map[int]int{}
    for i, b := range rawBuses {
      bus := safeAtoi(b)
      if bus > 0 { busIndex[bus] = i }
    }

    timeStamp := 0
    for bus, index := range busIndex {
      for (timeStamp + index) % bus != 0 {
        timeStamp += jump
      }
      jump *= bus
    }
    log.Print(timeStamp)
  }
}
