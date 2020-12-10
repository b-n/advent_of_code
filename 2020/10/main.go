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

func hasKey(input map[int]int, key int) bool {
  _, ok := input[key]
  return ok
}

func checkValue(input map[int]int, given int, collected []bool) int {
  options := options(input, given)

  if (len(options) == 0) { return 1 }

  extraOptions := len(options)
  log.Print(given, options)
  for _, option := range options {
    if (collected[option]) { continue }
    log.Print(given, option)
    extraOptions *= checkValue(input, option, collected)
    collected[option] = true
  }
  log.Print("eo", given, extraOptions)


  return extraOptions
}

func options(input map[int]int, given int) []int {
  res := []int{}
  for j := 1; j < 4; j++ {
    searchKey := given - j
    if hasKey(input, searchKey) {
      res = append(res, searchKey)
    }
  }

  return res
}

func main() {
  input := readFile("./test.txt")

  rawNumbers := strings.Split(strings.TrimRight(input, "\n"), "\n")

  stack := arrayAtoi(rawNumbers)

  adapters := make(map[int]int, len(stack))
  for _, v := range stack { adapters[v] = 0 }

  // Challenge 1
  {
    // Start with a adapter of 3
    differences := map[int]int{
      1: 0,
      2: 0,
      3: 1,
    }

    device := 0
    for device >= 0 {
      adapter := 0
      for j := 1; j < 4; j++ {
        searchAdapter := device + j
        if hasKey(adapters, searchAdapter) {
          adapter = searchAdapter;
          break
        }
      }
      if adapter == 0 { break }

      difference := adapter - device
      differences[difference]++
      device = adapter
    }
    log.Print(differences[1] * differences[3])
  }

  // Challenge 2
  {
    maxAdapter := 22

    collected := make([]bool, maxAdapter)
    totalOptions := checkValue(adapters, maxAdapter, collected)
    log.Print(totalOptions)
    // starting at the max value, how many adapters could get here?
    // 

    //totalOptions := checkValue(adapters, 0, 19)
    //log.Print(totalOptions)
  }
}
