package main
import (
  "log"
  "io/ioutil"
  "strings"
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

func getStringFromCoords(coords []int) string {
  return fmt.Sprintf("%d:%d:%d", coords[0], coords[1], coords[2])
}

func getCoordsFromString(input string) []int {
  res := []int{0,0,0}
  for i, v := range strings.Split(input, ":") {
    res[i] = int(v[0]) - 48
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

func tileColor(tileMap map[string]bool, coords []int) int {
  if color, ok := tileMap[getStringFromCoords(coords)]; ok {
    if color { return 1 }
    return 0
  }
  return -1
}

func countBlackTiles(tileMap map[string]bool) int {
  total := 0
  for _, tile := range tileMap {
    if tile { total++ }
  }
  return total
}

func main() {
  input := readFile("./test.txt")

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
      log.Print("Round ", i)

      // expand the borders for the black tiles, they default white, but could be flipped later
      for k, v := range tileColors {
        if !v { continue }
        coords := getCoordsFromString(k)
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
        colors := []int{0,0}
        for j := 0; j < 6; j++ {
          adjacent := coordFromDirection(coords, j)
          color := tileColor(tileColors, adjacent)
          if color == -1 { colors[0]++; continue }
          colors[color]++
        }
        //only store the black tiles, we expand for white next round
        if (v && (colors[1] == 1 || colors[1] == 2)) || (!v && colors[1] == 2) {
          res[k] = true
        }
      }
      tileColors = res
      log.Print(countBlackTiles(tileColors))
    }
    log.Print(countBlackTiles(tileColors))
  }
}
