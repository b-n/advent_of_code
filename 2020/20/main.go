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

func safeAtoi(input string) int {
  res, err := strconv.Atoi(input)
  check(err)
  return res
}

func contains(arr []int, v int) bool {
  for _, i := range arr {
    if i == v { return true }
  }
  return false
}

var SIDE_LENGTH = 10

type Side int

type Tile struct {
  Id int
  Data [][]string
  Neighbours []int
}

// get an edge in all possible directions as a string for comparing
func edge(t *Tile, side int) string {
  res := make([]string, SIDE_LENGTH)
  for i := 0; i < SIDE_LENGTH; i++ {
    switch side {
    //0-3 = clockwise
    //4-7 = anticlockwise
    case 0:
      res[i] = t.Data[0][i] // Y = 0, X = i
    case 1:
      res[i] = t.Data[i][9] // Y = i
    case 2:
      res[i] = t.Data[9][9-i] // Y = 9, X = -i
    case 3:
      res[i] = t.Data[9-i][0] // Y = -i, X = 0
    case 4:
      res[i] = t.Data[0][9-i] // Y = 0, X = -i
    case 5:
      res[i] = t.Data[9-i][9] // Y = -i, X = 9
    case 6:
      res[i] = t.Data[9][i] //Y = 9, X = i
    case 7:
      res[i] = t.Data[i][0] //Y = -i, X = 0
    }
  }
  return strings.Join(res, "")
}

func matchingSides(t, ot *Tile) (int, int) {
  for i := 0; i < 8; i++ {
    e := edge(t, i)
    for j := 0; j < 8; j++ {
      oe := edge(ot, j)
      //log.Print(i, j, " ", e, " ", oe)
      if e == oe { return i, j }
    }
  }
  return -1, -1
}

func getTile(input string) *Tile {
  parts := strings.SplitN(input, "\n", 2)
  rows := strings.Split(parts[1], "\n")
  res := Tile{
    Id: safeAtoi(parts[0][5:9]),
    Data: make([][]string, SIDE_LENGTH),
    Neighbours: []int{},
  }
  for y, row := range rows {
    res.Data[y] = make([]string, SIDE_LENGTH)
    for x, v := range row {
      res.Data[y][x] = string(v)
    }
  }
  return &res
}


func main() {
  input := readFile("./input.txt")

  rawTiles := strings.Split(strings.TrimRight(input, "\n"), "\n\n")

  tiles := make([]*Tile, len(rawTiles))
  for i, t := range rawTiles {
    tiles[i] = getTile(t)
  }

  // Challenge 1
  {
    // for each tile, check each edge, see if the edges from any other tiles match
    for _, t := range tiles {
      for _, ot := range tiles {
        // Don't check yourself
        if t.Id == ot.Id { continue }
        // Don't check if we already match!
        if contains(t.Neighbours, ot.Id) { continue }

        // check the rest
        s, os := matchingSides(t, ot)
        if s >= 0 && os >= 0 {
          t.Neighbours = append(t.Neighbours, ot.Id)
          ot.Neighbours = append(ot.Neighbours, t.Id)
        }
      }
    }

    total := 1
    for _, t := range tiles {
      if len(t.Neighbours) == 2 { total *= t.Id }
    }
    log.Print(total)
  }

  // Challenge 2
  {
  }
}
