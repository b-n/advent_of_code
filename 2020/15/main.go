package main

import (
	"log"
)

func solve(input []int, until int) int {
  foundMap := map[int][]int{}
  for i, v := range input {
    foundMap[v] = []int{i + 1, -1}
  }

  for len(input) < until {
    last := input[len(input) - 1]

    next := 0
    // if we have 2 last positions, use those
    if (foundMap[last][0] > -1 && foundMap[last][1] > -1) {
      next = foundMap[last][0] - foundMap[last][1]
    }

    // if we're not tracking this number yet, track it
    if _, ok := foundMap[next]; !ok { foundMap[next] = []int{-1, -1} }
    
    // shuffle the last found positions
    foundMap[next][1] = foundMap[next][0]
    foundMap[next][0] = len(input) + 1

    input = append(input, next)
  }
  return input[len(input)-1]
}

func main() {
  //Challenge 1
  {
    input := []int{8,13,1,0,18,9}
    log.Print(solve(input, 2020))
  }

  //Challenge 2
  {
    input := []int{8,13,1,0,18,9}
    log.Print(solve(input, 30000000))
  }
}
