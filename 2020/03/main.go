package main

import (
  "log"
  "io/ioutil"
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

func wrappedCoordinate(x, y, maxX, maxY int) (int, int) {
  return x % maxX, y % maxY
}

type xyGetter func(int, int) byte
type treeCounter func(int, int) int

type Forest struct {
  xMax  int
  yMax  int
  field []byte
  get   xyGetter
  trees treeCounter
}

func generateForest(input string) Forest {
  rows := strings.Split(input, "\n")

  res := Forest{}

  res.xMax = len(rows[0])
  res.yMax = len(rows) - 1

  result := make([]byte, res.xMax * res.yMax)

  for i, y := range rows {
    if len(y) == 0 { continue }
    for j := 0; j < len(y); j++ {
      result[i*res.xMax + j] = y[j]
    }
  }

  res.field = result
  res.get = func(x, y int) byte {
    realX, realY := wrappedCoordinate(x, y, res.xMax, res.yMax)
    if realX == -1 || realY == -1 { return 0 }
    return res.field[realY*res.xMax + realX]
  }
  res.trees = func(x, y int) int {
    count := 0
    for i := 0; i*y < res.yMax; i++ {
      if res.get(i*x, i*y) == 35 { count++ }
    }
    return count
  }
  return res
}


func main() {
  input := readFile("./input.txt")

  forest := generateForest(input)

  log.Print("Challenge 1: ", forest.trees(3, 1))
  log.Print("Challenge 2: ", forest.trees(1, 1) * forest.trees(3,1) * forest.trees(5, 1) * forest.trees(7, 1) * forest.trees(1, 2))
}
