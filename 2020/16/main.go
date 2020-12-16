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
  return int(output)
}

func safeArrayAtoi(input []string) []int {
  res := make([]int, len(input))
  for i, v := range input { res[i] = safeAtoi(v) }
  return res
}

type Condition struct {
  name string
  ranges [][]int
}

func stringToCondition(input string) Condition {
  parts := strings.Split(input, ": ")
  ranges := strings.Split(parts[1], " or ")
  return Condition{
    name: parts[0],
    ranges: [][]int{
      safeArrayAtoi(strings.Split(ranges[0], "-")),
      safeArrayAtoi(strings.Split(ranges[1], "-")), 
    },
  }
}

func validateSegment(v int, ranges [][]int) int {
  for _, r := range ranges {
    if v >= r[0] && v <= r[1] { return 0 }
  }
  return v
}

func validateTicket(ticket []int, cons []Condition) int {
  res := 0
  ranges := make([][]int, len(cons) * 2)
  for i, c := range cons { ranges[i*2],ranges[i*2+1] = c.ranges[0],c.ranges[1] }
  for _, v := range ticket {
    res += validateSegment(v, ranges)
  }
  return res
}

func main() {
  input := readFile("./input.txt")

  sections := strings.Split(strings.TrimRight(input, "\n"), "\n\n")

  rawConditions := strings.Split(sections[0], "\n")
  conditions := make([]Condition, len(rawConditions))
  for i, v := range rawConditions { conditions[i] = stringToCondition(v) }

  //myTicket := strings.Split(strings.Split(sections[1], "\n")[1], ",")

  rawNearbyTickets := strings.Split(sections[2], "\n")
  nearbyTickets := make([][]int, len(rawNearbyTickets) - 1)
  for i, v := range rawNearbyTickets[1:] { nearbyTickets[i] = safeArrayAtoi(strings.Split(v, ",")) }

  //Challenge 1
  {
    invalidValues := 0
    for _, t := range nearbyTickets {
      invalidValues += validateTicket(t, conditions)
    }
    log.Print(invalidValues)
  }

  //Challenge 2
  {
  }
}
