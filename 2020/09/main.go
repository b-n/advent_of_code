package main

import (
  "log"
  "io/ioutil"
  "strings"
  "strconv"
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

func arrayAtoi(input []string) []int {
  res := make([]int, len(input))
  for i, v := range input {
    n, _ := strconv.Atoi(v)
    res[i] = n
  }
  return res
}

func extent(input []int) (int, int) {
  min, max := int(^uint(0) >> 1), 0
  for _, val := range input {
    if (val < min) { min = val }
    if (val > max) { max = val }
  }
  return min, max
}

func numInArraySum2(input int, arr []int) bool {
  for i, n := range arr {
    for _, m := range arr[i+1:] {
      if n + m == input { return true }
    }
  }
  return false
}

func findContiguous(input int, stack []int) []int {
  res := []int{}
  for i := 0; i < len(stack); i++ {
    total := stack[i]
    res = []int{stack[i]}
    for j := i + 1; j < len(stack) && total < input; j++ {
      total += stack[j]
      res = append(res, stack[j])
      if total == input {
        return res
      }
    }
  }
  return []int{}
}

func main() {
  input := readFile("./input.txt")

  rawNumbers := strings.Split(strings.TrimRight(input, "\n"), "\n")

  stack := arrayAtoi(rawNumbers)

  weakXmasNumber := 0
  // Challenge 1
  {
    preambleLength := 25
    for i := 0; i < len(stack) - preambleLength; i++ {
      v := stack[i + preambleLength]
      searchSpace := stack[i:i+preambleLength]
      if !numInArraySum2(v, searchSpace) {
        weakXmasNumber = v
        break
      }
    }
    log.Print(weakXmasNumber)
  }

  // Challenge 2
 {
    res := findContiguous(weakXmasNumber, stack)
    min, max := extent(res)
    log.Print(min + max)
  }
}
