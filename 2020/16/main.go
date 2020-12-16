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

// returns -1 if in one of the ranges, otherwise the value
func validateSegment(v int, ranges [][]int) int {
  for _, r := range ranges {
    if v >= r[0] && v <= r[1] { return -1 }
  }
  return v
}

//returns -1 if the ticket values in total match at least one condition
func validateTicket(ticket []int, cons []Condition) int {
  res := 0
  ranges := make([][]int, len(cons) * 2)
  for i, c := range cons { ranges[i*2],ranges[i*2+1] = c.ranges[0],c.ranges[1] }
  matchedConditions := 0
  for _, v := range ticket {
    validation := validateSegment(v, ranges) 
    if validation == -1 {
      matchedConditions++
    } else {
      res += validateSegment(v, ranges)
    }
  }
  if matchedConditions == len(cons) { return -1 }
  return res
}

func validConditionsForField(tickets [][]int, conditions []Condition) [][]Condition {
  res := make([][]Condition, len(conditions))
  for _, c := range conditions {
    r1 := c.ranges[0]
    r2 := c.ranges[1]
    for i := range tickets[0] {
      matches := true
      for _, t := range tickets {
        if (t[i] < r1[0] || t[i] > r1[1]) && (t[i] < r2[0] || t[i] > r2[1]) {
          matches = false
          break
        }
      }
      if matches {
        res[i] = append(res[i], c);
      }
    }
  }
  return res
}


func main() {
  input := readFile("./input.txt")

  sections := strings.Split(strings.TrimRight(input, "\n"), "\n\n")

  rawConditions := strings.Split(sections[0], "\n")
  conditions := make([]Condition, len(rawConditions))
  for i, v := range rawConditions { conditions[i] = stringToCondition(v) }

  myTicket := safeArrayAtoi(strings.Split(strings.Split(sections[1], "\n")[1], ","))

  rawNearbyTickets := strings.Split(sections[2], "\n")
  nearbyTickets := make([][]int, len(rawNearbyTickets) - 1)
  for i, v := range rawNearbyTickets[1:] { nearbyTickets[i] = safeArrayAtoi(strings.Split(v, ",")) }

  //Challenge 1
  validTickets := [][]int{}
  {
    invalidValues := 0
    for _, t := range nearbyTickets {
      validationResult := validateTicket(t, conditions)
      if validationResult == -1 {
        validTickets = append(validTickets, t)
      } else {
        invalidValues += validationResult
      }
    }
    log.Print(invalidValues)
  }

  //Challenge 2
  {
    fields := validConditionsForField(validTickets, conditions)

    for i, f := range fields {
      log.Print(i, f)
    }

    mappedConditions := map[string]int{}

    // fields = an array of ticket field to an array of possible conditions
    // we want to map a single condition to each field, so we start mappedConditins being empty
    // for each field, we want to see how many possible conditions there are
    //   it's possible that one of the conditions in our conditions array is already taken, so we don't count those
    // if we have 0 matching conditions (i.e. they already assigned to another field), then we mark it as missing
    // if there is only 1 possible condition (e.g. only one ever matched, or the conditions are all used elsewhere),
    //    then that must be the correct one
    // keep going until we have a value for all mapped conditions. This could iterate through the fields multiple times
    // since each round we are likely removing one from the pool by mapping it
    for len(mappedConditions) < len(conditions) {
      for i, f := range fields {
        possibleValues := []string{}
        for _, c := range f {
          if _, ok := mappedConditions[c.name]; !ok {
            possibleValues = append(possibleValues, c.name)
          }
        }
        if len(possibleValues) == 0 {
          mappedConditions[strconv.Itoa(i)] = i
        }
        if len(possibleValues) == 1 {
          mappedConditions[possibleValues[0]] = i
        }
      }
    }
    // Not all fields map weirdly enough, but it mapped all the departure fields, and that's all we need

    // calculate the output for the puzzle
    total := 1
    for k, v := range mappedConditions {
      if strings.Split(k, " ")[0] == "departure" {
        total *= myTicket[v]
      }
    }
    log.Print(total)
  }
}
