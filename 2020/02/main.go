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

func charAt(input string, pos int) string {
  if (pos >= len(input)) { return "" }
  return string(input[pos])
}

type Policy struct {
  char string
  int1 int
  int2 int
}

func getPolicyFromString(input string) Policy {
  sections := strings.Fields(input)
  prange := strings.Split(sections[0], "-")
  int1, _ := strconv.Atoi(prange[0])
  int2, _ := strconv.Atoi(prange[1])
  return Policy{ char: sections[1], int1: int1, int2: int2 }
}

func hasValidCount(input string, p Policy) bool {
  count := strings.Count(input, p.char)
  return (count >= p.int1) && (count <= p.int2)
}

func hasValidPosition(input string, p Policy) bool {
  pos1 := charAt(input, p.int1 - 1)
  pos2 := charAt(input, p.int2 - 1)
  // Go has no XOR?
  return (pos1 == p.char) != (pos2 == p.char)
}

func main() {
  input := readFile("./input.txt")

  entries := strings.Split(input, "\n")

  validCounts := 0
  validPositions := 0
  for _, entry := range entries {
    sections := strings.Split(entry, ":")
    if len(sections) != 2 { continue }

    policy := getPolicyFromString(sections[0])
    password := strings.TrimSpace(sections[1])

    if hasValidCount(password, policy) { validCounts++ }
    if hasValidPosition(password, policy) { validPositions++ }
  }
  log.Print("Valid Count Passwords: ", validCounts)
  log.Print("Valid Position Passwords: ", validPositions)
}

