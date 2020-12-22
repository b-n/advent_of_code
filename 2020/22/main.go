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
  data, err := strconv.Atoi(input)
  check(err)
  return data
}

func main() {
  input := readFile("./input.txt")

  rawPiles := strings.Split(strings.TrimRight(input, "\n"), "\n\n")

  piles := make([][]int, len(rawPiles))
  totalSize := 0
  for i, p := range rawPiles {
    rows := strings.Split(p, "\n")
    totalSize += len(rows) - 1
    piles[i] = make([]int, len(rows) - 1)
    for j, r := range rows {
      if j == 0 { continue }
      piles[i][j - 1] = safeAtoi(r)
    }
  }

  // Challenge 1
  {
    i := 0
    for true {
    //for i := 0; i < 10; i++ {
      log.Print("Round ", i, piles)
      // take top cards
      current := []int{
        piles[0][0],
        piles[1][0],
      }

      //shorten existing arrays

      piles[0] = piles[0][1:]
      piles[1] = piles[1][1:]
      log.Print(current, piles)

      if current[0] > current[1] {
        piles[0] = append(append(piles[0], current[0]), current[1])
      } else {
        piles[1] = append(append(piles[1], current[1]), current[0])
      }

      if len(piles[0]) == totalSize || len(piles[1]) == totalSize { break }
      i++
    }

    winningPile := piles[0]
    if len(winningPile) == 0 {
      winningPile = piles[1]
    }

    log.Print(winningPile, totalSize)
    topScore := 0
    for i := 0; i < totalSize; i++ {
      topScore += winningPile[totalSize - i - 1] * (i + 1)
    }
    log.Print(topScore)
  }

  // Challenge 2
  {
  }
}
