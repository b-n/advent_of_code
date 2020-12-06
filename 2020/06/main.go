package main

import (
  "log"
  "io/ioutil"
  "strings"
  //"strconv"
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

func answerToInt(input string) int {
  res := 0
  for _, c := range input {
    res |= 1 << (c - 97)
  }
  return res
}

func main() {
  input := readFile("./input.txt")

  rawGroups := strings.Split(input, "\n\n")

  groups := make([][]string, len(rawGroups))

  for i, g := range rawGroups {
    groups[i] = strings.Split(strings.TrimRight(g, "\n"), "\n")
  }

  // Challenge 1
  {
    total := 0
    for _, g := range groups {
      groupRes := 0
      for _, person := range g {
        groupRes |= answerToInt(person)
      }

      answeredGroups := 0
      for i := 0; i < 26; i++ {
        if ((1 << i) & groupRes) > 0 {
          answeredGroups++
        }
      }

      total += answeredGroups
    }
    log.Print(total)
  }

  // Challenge 2
  {
    total := 0
    for _, g := range groups {
      groupRes := (1 << 27) - 1

      for _, person := range g {
        groupRes &= answerToInt(person)
      }

      answeredGroups := 0
      for i := 0; i < 26; i++ {
        if ((1 << i) & groupRes) > 0 {
          answeredGroups++
        }
      }

      total += answeredGroups
    }
    log.Print(total)
  }
}
