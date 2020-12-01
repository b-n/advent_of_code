package main

import (
  "log"
  "io/ioutil"
  "net/http"
  "strings"
  "strconv"
  "sort"
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

func arrToI(arr []string) []int {
  ret := []int{}
  for _, s := range arr {
    v, _ := strconv.Atoi(s)
    ret = append(ret, v)
  }
  return ret
}

func pairedSearch(toFind int, arr []int) (int, int) {
  zMax := len(arr)
  for i, v := range arr {
    coupledValue := toFind - v

    j := 0
    for j = zMax - 1; j > 0 && arr[j] > coupledValue; j-- {}

    if arr[j] == coupledValue {
      return i, j
    }

    zMax = j + 1
  }
  return -1, -1
}

func trippledSearch(toFind int, arr []int) (int, int, int) {
  zMax := len(arr)
  for i, v := range arr {
    coupledValue := toFind - v

    j := 0
    for j = zMax - 1; j > 0 && arr[j] > coupledValue; j-- {}

    a, b := pairedSearch(coupledValue, arr[0:j])

    if a + b > 0 {
      return i, a, b
    }

    zMax = j + 1
  }
  return -1, -1, -1
}

func main() {
  text := readFile("./input.txt")

  entries := arrToI(strings.Fields(text))
  sort.Ints(entries)

  i, j := pairedSearch(2020, entries)
  log.Print(entries[i], entries[j], entries[i] * entries[j])

  a,b,c := trippledSearch(2020, entries)
  log.Print(entries[a], entries[b], entries[c], entries[a] * entries[b] * entries[c])
}

