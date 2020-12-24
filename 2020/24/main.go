package main
import (
  "log"
  "io/ioutil"
  "strings"
  "fmt"
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

func getStringFromCoords(coords []int) string {
  return fmt.Sprintf("%d:%d:%d", coords[0], coords[1], coords[2])
}

func getCoordsFromString(input string) []int {
  res := []int{0,0,0}
  for i, v := range strings.Split(input, ":") {
    value, _ := strconv.Atoi(v)
    res[i] = value
  }
  return res
}

func coordFromDirection(input []int, direction int) []int {
  res := []int{0,0,0}
  for i, v := range input { res[i] = v }
  switch direction {
  case 0: //ne
    res[0]++; res[2]--
  case 1: //nw
    res[1]++; res[2]--
  case 2: //w
    res[0]--; res[1]++
  case 3: //sw
    res[0]--; res[2]++
  case 4: //se
    res[1]--; res[2]++
  case 5: //e
    res[0]++; res[1]--
  }
  return res 
}

func countBlackTiles(tileMap map[string]bool) int {
  total := 0
  for _, tile := range tileMap {
    if tile { total++ }
  }
  return total
}

func main() {
  input := readFile("./input.txt")

  input = strings.Replace(input, "ne", "0", -1)
  input = strings.Replace(input, "nw", "1", -1)
  input = strings.Replace(input, "sw", "3", -1)
  input = strings.Replace(input, "se", "4", -1)
  input = strings.Replace(input, "w", "2", -1)
  input = strings.Replace(input, "e", "5", -1)

  rawTiles := strings.Split(strings.TrimRight(input, "\n"), "\n")

  tiles := make([][]int, len(rawTiles))
  for i, t := range rawTiles {
    tiles[i] = make([]int, len(t))
    for j, f := range t {
      tiles[i][j] = int(f) - 48
    }
  }

  // Challenge 1
  tileColors := map[string]bool{}
  {
    for _, tile := range tiles {
      coords := []int{0,0,0}
      for _, dir := range tile {
        coords = coordFromDirection(coords, dir)
      }
      tileCoords := getStringFromCoords(coords)
      if color, ok := tileColors[tileCoords]; ok {
        tileColors[tileCoords] = !color
      } else {
        tileColors[tileCoords] = true
      }
    }
    log.Print(countBlackTiles(tileColors))
  }

  // Challenge 2
  {
    for i := 0; i < 100; i++ {
      // add white tiles around the black tiles, they might be flipped in this round
      blackTiles := map[string]bool{}
      for k, v := range tileColors {
        if !v { continue }
        coords := getCoordsFromString(k)
        if tileColors[k] { blackTiles[k] = true }
        for j := 0; j < 6; j++ {
          adjacent := getStringFromCoords(coordFromDirection(coords, j))
          if _, ok := tileColors[adjacent]; !ok { tileColors[adjacent] = false }
        }
      }

      // The new map for later
      res := map[string]bool{}
      for k, v := range tileColors {
        coords := getCoordsFromString(k)

        // count the surrounding colors
        adjacentBlack := 0
        for j := 0; j < 6; j++ {
          adjacent := coordFromDirection(coords, j)
          if blackTiles[getStringFromCoords(adjacent)] {
            adjacentBlack++
          }
        }
        //only store the black tiles, white tiles are added at the beginning of the round
        if (v && (adjacentBlack == 1 || adjacentBlack == 2)) || (!v && adjacentBlack == 2) {
          res[k] = true
        }
      }
      tileColors = res
    }
    log.Print(countBlackTiles(tileColors))
  }
}
