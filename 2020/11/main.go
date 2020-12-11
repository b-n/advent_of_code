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

func inBounds(board [][]rune, x int, y int) bool {
  if y < 0 || y > len(board) - 1 { return false }
  if x < 0 || x > len(board[0]) - 1 { return false }
  return true
}

type countFunc = func(board [][]rune, x int, y int) int

func countOccupiedAdjacent(board [][]rune, x int, y int) int {
  occupied := 0
  for i := 0; i < 9; i++ {
    if (i == 4) { continue }
    posX, posY := x + ((i % 3) - 1), y + ((i / 3) - 1)
    if inBounds(board, posX, posY) && board[posY][posX] == '#' {
      occupied++
    }
  }
  return occupied
}

func countOccupiedAngles(board [][]rune, x int, y int) int {
  occupied := 0
  for i := 0; i < 9; i++ {
    if (i == 4) { continue }
    dirX, dirY := ((i % 3) - 1), ((i / 3) - 1)
    posX, posY := x, y
    found := false
    for true {
      posX += dirX
      posY += dirY
      if !inBounds(board, posX, posY) { break }
      if board[posY][posX] == 'L' { break }
      if board[posY][posX] == '#' {
        found = true
        break
      }
    }
    if found { occupied++ }
  }

  return occupied
}

func stepBoard(board [][]rune, birth int, die int, countFunction countFunc) [][]rune {
  rowLength := len(board[0])
  output := make([][]rune, len(board))
  for y, row := range board {
    output[y] = make([]rune, rowLength)
    for x, v := range row {
      if v == '.' {
        output[y][x] = '.'
        continue
      }

      occupiedAdjacent := countFunction(board, x, y)
      if (v == 'L' && occupiedAdjacent == birth) {
        output[y][x] = '#'
        continue
      }
      if (v == '#' && occupiedAdjacent >= die) {
        output[y][x] = 'L'
        continue
      }
      output[y][x] = v
    }
  }
  return output
}

func calculateBoard(board [][]rune) int {
  total := 0
  for _, row := range board {
    for _, v := range row {
      if (v == '#') { total ++ }
    }
  }
  return total
}

func printBoard(board [][]rune) {
  for _, row := range board {
    log.Print(string(row))
  }
}

func main() {
  input := readFile("./input.txt")

  rawBoard := strings.Split(strings.TrimRight(input, "\n"), "\n")
  board := make([][]rune, len(rawBoard))
  for i, row := range rawBoard {
    board[i] = []rune(row)
  }

  //Challenge 1
  {
    lastOccupied := 0
    printBoard(board)
    for true {
      copy(board, stepBoard(board, 0, 4, countOccupiedAdjacent))
      printBoard(board)
      occupied := calculateBoard(board)
      log.Print(occupied)
      if (lastOccupied == occupied) { break }
      lastOccupied = occupied
    }
  }

  //Challenge 2
  {
    lastOccupied := 0
    printBoard(board)
    for true {
      copy(board, stepBoard(board, 0, 5, countOccupiedAngles))
      printBoard(board)
      occupied := calculateBoard(board)
      log.Print(occupied)
      if (lastOccupied == occupied) { break }
      lastOccupied = occupied
    }
  }
}
