package main
import (
  "log"
  "io/ioutil"
  "strings"
  "strconv"
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
  data, err := strconv.Atoi(input)
  check(err)
  return data
}

func copyPiles(input [][]int) [][]int {
  res := make([][]int, len(input))
  for i, in := range input {
    res[i] = make([]int, len(in))
    for j, v := range in {
      res[i][j] = v
    }
  }
  return res
}

func getHandString(piles [][]int) string {
  res := ""
  for i, pile := range piles {
    res = res + fmt.Sprintf("%d:%s", i, fmt.Sprint(pile))
  }
  return res
}

func playNormal(piles [][]int) [][]int {
  totalSize := 0
  for _, pile := range piles { totalSize += len(pile) }

  for true {
    // take top cards
    current := []int{
      piles[0][0],
      piles[1][0],
    }

    piles[0] = piles[0][1:]
    piles[1] = piles[1][1:]

    if current[0] > current[1] {
      piles[0] = append(append(piles[0], current[0]), current[1])
    } else {
      piles[1] = append(append(piles[1], current[1]), current[0])
    }

    if len(piles[0]) == totalSize || len(piles[1]) == totalSize { break }
  }

  return piles
}

func playRecursive(piles [][]int) [][]int {
  uniqueHands := map[string]bool{}

  totalSize := 0
  for _, pile := range piles { totalSize += len(pile) }

  i := 0
  for true {
    if len(piles[0]) == 0 || len(piles[1]) == 0 { break }

    // take top cards
    current := []int{
      piles[0][0],
      piles[1][0],
    }

    piles[0] = piles[0][1:]
    piles[1] = piles[1][1:]

    winner := 0

    if current[0] <= len(piles[0]) && current[1] <= len(piles[1]) {
      newPiles := copyPiles(piles)
      newPiles[0] = newPiles[0][:current[0]]
      newPiles[1] = newPiles[1][:current[1]]
      outcome := playRecursive(newPiles)
      if len(outcome[0]) == 0 {
        winner = 1
      }
    } else {
      if current[0] < current[1] {
        winner = 1
      }
    }

    if winner == 0 {
      piles[0] = append(append(piles[0], current[0]), current[1])
    } else {
      piles[1] = append(append(piles[1], current[1]), current[0])
    }

    // check for hand uniqueness in this round, or fail with player1 winning
    handsString := getHandString(piles)
    if uniqueHands[handsString] {
      return [][]int{
        []int{1},
        []int{},
      }
    }
    uniqueHands[handsString] = true
    i++
  }

  return piles
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
    result := playNormal(copyPiles(piles))

    winningPile := result[0]
    if len(winningPile) == 0 {
      winningPile = result[1]
    }

    topScore := 0
    for i := 0; i < totalSize; i++ {
      topScore += winningPile[totalSize - i - 1] * (i + 1)
    }
    log.Print(topScore)
  }

  // Challenge 2
  {
    result := playRecursive(copyPiles(piles))

    winningPile := result[0]
    if len(winningPile) == 0 {
      winningPile = result[1]
    }

    topScore := 0
    for i := 0; i < totalSize; i++ {
      topScore += winningPile[totalSize - i - 1] * (i + 1)
    }
    log.Print(topScore)
  }
}
