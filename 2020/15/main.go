package main

import (
	"log"
)

func main() {
  //Challenge 1
  {
    input := []int{8,13,1,0,18,9}

    for len(input) < 2020 {
      last := input[len(input) - 1]

      lastTurn := -1
      lastLastTurn := -1
      // searching from right, find the last 2 times it was used
      for i := len(input) - 1; i >= 0; i-- {
        if lastTurn >= 0 && input[i] == last { lastLastTurn = i + 1; break }
        if lastTurn == -1 && input[i] == last { lastTurn = i + 1; }
      }

      next := 0
      // if we found both last times, next = the difference in those turns
      if lastLastTurn >= 0 {
        next = lastTurn - lastLastTurn
      }

      input = append(input, next)
    }
    log.Print(input[len(input)-1])
  }

  //Challenge 2
  {
  }
}
