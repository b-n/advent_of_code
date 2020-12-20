package main
import (
  "log"
  "io/ioutil"
  "strings"
  "strconv"
  "math"
  "fmt"
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
  Neighbours map[int]int
}

// get an edge in all possible directions as a string for comparing
func edge(d [][]string, side int) string {
  res := make([]string, SIDE_LENGTH)
  for i := 0; i < SIDE_LENGTH; i++ {
    switch side {
    //0-3 = clockwise
    //4-7 = anticlockwise
    case 0:
      res[i] = d[0][i] // Y = 0, X = i
    case 1:
      res[i] = d[i][9] // Y = i
    case 2:
      res[i] = d[9][9-i] // Y = 9, X = -i
    case 3:
      res[i] = d[9-i][0] // Y = -i, X = 0
    case 4:
      res[i] = d[0][9-i] // Y = 0, X = -i
    case 5:
      res[i] = d[9-i][9] // Y = -i, X = 9
    case 6:
      res[i] = d[9][i] //Y = 9, X = i
    case 7:
      res[i] = d[i][0] //Y = -i, X = 0
    }
  }
  return strings.Join(res, "")
}

func matchingSides(t, ot *Tile) (int, int) {
  for i := 0; i < 8; i++ {
    e := edge(t.Data, i)
    for j := 0; j < 8; j++ {
      oe := edge(ot.Data, j)
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
    Neighbours: map[int]int{},
  }
  for y, row := range rows {
    res.Data[y] = make([]string, SIDE_LENGTH)
    for x, v := range row {
      res.Data[y][x] = string(v)
    }
  }
  return &res
}

func rotate2dClockwise(input [][]string) [][]string {
  n := len(input)
  ret := make([][]string, n)
  for i := 0; i < n; i++ {
    ret[i] = make([]string, n)
    for j := 0; j < n; j++ {
      ret[i][j] = input[n -j -1][i]
    }
  }
  return ret
}

func flip(input [][]string) [][]string {
  n := len(input)
  ret := make([][]string, n)
  for i := 0; i < n; i++ {
    ret[i] = make([]string, n)
    for j := 0; j < n; j++ {
      ret[i][j] = input[n-1-i][j]
    }
  }
  return ret
}

func printTileMap(tileMap [][]*Tile) {
  for y := 0; y < len(tileMap) * 10; y++ {
    for x := 0; x < len(tileMap) * 10; x++ {
      fmt.Printf("%s", tileMap[y/10][x/10].Data[y%10][x%10])
      if x % 10 == 9 { fmt.Printf(" ") }
    }
    fmt.Print("\n")
    if y % 10 == 9 { fmt.Printf("\n") }
  }
}

func printPicture(picture [][]string) {
  for _, row := range picture {
    log.Print(strings.Join(row, ""))
  }
}

func main() {
  input := readFile("./test.txt")

  rawTiles := strings.Split(strings.TrimRight(input, "\n"), "\n\n")

  tiles := map[int]*Tile{}
  for _, t := range rawTiles {
    tile := getTile(t)
    tiles[tile.Id] = tile
  }

  log.Print(len(rawTiles))

  // Challenge 1
  {
    // for each tile, check each edge, see if the edges from any other tiles match
    for _, t := range tiles {
      for _, ot := range tiles {
        // Don't check yourself
        if t.Id == ot.Id { continue }
        // Don't check if we already match!
        if _, ok := t.Neighbours[ot.Id]; ok { continue }

        // check the rest
        s, os := matchingSides(t, ot)
        if s >= 0 && os >= 0 {
          t.Neighbours[ot.Id] = s
          ot.Neighbours[t.Id] = os
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
    // Lets make a grid of tiles! (initilaze it with nils to make life easier)
    gridSize := int(math.Sqrt(float64(len(rawTiles))))
    tileMap := make([][]*Tile, gridSize)
    for i := 0; i < gridSize; i++ {
      tileMap[i] = make([]*Tile, gridSize)
    }

    // pick a random top left
    for _, t := range tiles {
      if len(t.Neighbours) == 2 {
        tileMap[0][0] = t
        break
      }
    }

    // fill in grid adjacent cells (randomly)
    i := 0
    for k := range tileMap[0][0].Neighbours {
      t := tiles[k]
      if (i == 0) { tileMap[0][1] = t }
      if (i == 1) { tileMap[1][0] = t }
      i++
    }

    // fill the rest of the grid
    for y := 0; y < gridSize; y++ {
      for x := 0; x < gridSize; x++ {
        // already filled? ignore you
        if tileMap[y][x] != nil { continue }

        // first row? we're only looking for edge pieces (2/3 neighbours)
        if y == 0 {
          for k := range tileMap[y][x-1].Neighbours {
            if k != tileMap[0][x-2].Id && len(tiles[k].Neighbours) < 4 {
              tileMap[y][x] = tiles[k]
            }
          }
        } else {
          // row x onwards.
          // If it's the first column (0), we're looking for an edge piece that hasn't been used
          // Otherwise we're finding the one piece that will fit that hasn't been used 
          upTile := tileMap[y-1][x]

          for k1 := range upTile.Neighbours {
            if x == 0 {
              if k1 != tileMap[y-2][x].Id && len(tiles[k1].Neighbours) < 4 {
                tileMap[y][x] = tiles[k1]
              }
            } else {
              for k2 := range tileMap[y][x-1].Neighbours {
                if k1 == k2 && k1 != tileMap[y-1][x-1].Id {
                  tileMap[y][x] = tiles[k1]
                }
              }
            }
          }
        }
      }
    }
    for y := 0; y < gridSize; y++ {
      for x := 0; x < gridSize; x++ {
        fmt.Printf("%d ", tileMap[y][x].Id)
      }
      fmt.Printf("\n")
    }

    log.Print("Orientate!")
    //orientate everything
    for y := 0; y < gridSize; y++ {
      for x := 0; x < gridSize; x++ {
        t1 := tileMap[y][x]
        if y == 0 {
          if x == 0 {
            // orient relative to right. can't flip, because we're following this one's direction
            t2 := tileMap[y][x+1]
            s1, _ := matchingSides(t1, t2)
            baseRotation := 1
            rotations := (baseRotation - s1 + 4) % 4
            for i := rotations; i > 0; i-- {
              t1.Data = rotate2dClockwise(t1.Data)
            }
          } else {
            // orient relative to left
            // matchingSides always assumes the first argument is correctly oriented, so we rely on that
            t2 := tileMap[y][x-1]
            baseRotation := 3
            s1, s2 := matchingSides(t2, t1)
            if s1 != 1 { log.Print("t2 has bad orientation") }
            if s2 < 4 {
              // our data should mirror t1. If we have a direct match, that means we flip!
              t1.Data = flip(t1.Data)
            }
            _, s2 = matchingSides(t2, t1)
            rotations := (baseRotation - s2 + 8) % 4
            for i := rotations; i > 0; i-- {
              t1.Data = rotate2dClockwise(t1.Data)
            }
          }
        } else {
          t2 := tileMap[y-1][x]
          if x == 0 && y == 1 {
            //make sure top row is flipped correctly
            s1, _ := matchingSides(t2, t1)
            if s1 != 2 {
              log.Print("Flipping top row")
              for i := 0; i < gridSize; i++ {
                tileMap[0][i].Data = flip(tileMap[0][i].Data)
              }
            }
          }
          // orient relative to up
          baseRotation := 0
          s1, s2 := matchingSides(t2, t1)
          if s1 != 2 { log.Print("t2 has bad orientation") }
          if s2 < 4 {
            // our data should mirror t1. If we have a direct match, that means we flip!
            t1.Data = flip(t1.Data)
          }
          _, s2 = matchingSides(t2, t1)
          rotations := (baseRotation - s2 + 8) % 4
          for i := rotations; i > 0; i-- {
            t1.Data = rotate2dClockwise(t1.Data)
          }
        }
      }
    }

    printTileMap(tileMap)

    // let's make the picture!
    pictureSize := gridSize * 8
    picture := make([][]string, pictureSize)
    row := 0
    for y := 0; y < len(tileMap) * 10; y++ {
      if y % 10 == 0 || y % 10 == 9 { continue }
      picture[row] = make([]string, pictureSize)
      col := 0
      for x := 0; x < len(tileMap) * 10; x++ {
        if x % 10 == 0 || x % 10 == 9 { continue }
        tile := tileMap[y/10][x/10]
        picture[row][col] = tile.Data[y%10][x%10]
        col++
      }
      row++
    }

    monster := []string{
      "                  # ",
      "#    ##    ##    ###",
      " #  #  #  #  #  #   ",
    }

    for i := 0; i < 8; i++ {
      log.Print("Iteration ", i)
      printPicture(picture)
      for y := 0; y < pictureSize - len(monster); y++ {
        for x := 0; x < pictureSize - len(monster[0]); x++ {
          if picture[y][x+18] == "#" {
            //the first # begins on the 18th character, just small optimization, but will break for different monsters
            ifound := true
            for my := 1; my < len(monster) && ifound; my++ {
              for mx := 0; mx < len(monster[0]) && ifound; mx++ {
                if string(monster[my][mx]) == "#" && picture[y+my][x+mx] != "#" {
                  ifound = false
                }
              }
            }
            if ifound { log.Print("GOT EM ", x, y) }
          }
        }
      }

      if i == 3 { picture = flip(picture) }
      picture = rotate2dClockwise(picture)
    }
  }
}
