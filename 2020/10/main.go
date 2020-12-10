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

func walk(input map[int][]int, given int, cache map[int]int) int {
  if _, ok := cache[given]; ok { return cache[given] }

  total := 0
  for _, option := range input[given] {
    total += walk(input, option, cache)
  }
  // if we have no total, we're probably with no optins, e.g. end of the line
  if total == 0 { total = 1 }
  cache[given] = total

  return total
}

func main() {
  input := readFile("./input.txt")

  rawNumbers := strings.Split(strings.TrimRight(input, "\n"), "\n")

  stack := arrayAtoi(rawNumbers)

  adapters := map[int][]int{}

  for _, v := range stack { adapters[v] = []int{} }
  adapters[0] = []int{}

  //make a map of adapter to it's possible options
  for v, _ := range adapters {
    for j := 1; j < 4; j++ {
      searchKey := v + j
      if _, ok := adapters[searchKey]; ok {
        adapters[v] = append(adapters[v], searchKey) 
      }
    }
  }

  // Challenge 1
  {
    // Start with a adapter of 3
    differences := map[int]int{
      1: 0,
      2: 0,
      3: 1,
    }

    next := 0
    for true {
      if len(adapters[next]) == 0 { break }

      adapter := adapters[next][0]
      difference := adapter - next
      differences[difference]++
      next = adapter
    }
    log.Print(differences[1] * differences[3])
  }

  // Challenge 2
  {
    cache := map[int]int{}
    paths := walk(adapters, 0, cache)
    log.Print(paths)
  }
}
