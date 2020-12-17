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

type xAxis = map[int]rune
type yAxis = map[int]xAxis
type zAxis = map[int]yAxis

type Board struct {
  xExtent []int
  yExtent []int
  zExtent []int
  data zAxis
}
type countFunc = func(board Board, x int, y int, z int) int

func countOccupiedAdjacent(board Board, x int, y int, z int) int {
  occupied := 0
  for i := 0; i < 27; i++ {
    if (i == 13) { continue }
    posZ := z + (i / 9) - 1
    posY := y + ((i % 9) / 3) - 1
    posX := x + (i % 3) - 1
    if board.data[posZ][posY][posX] == '#' {
      occupied++
    }
  }
  return occupied
}

func stepBoard(board Board, countFunction countFunc) Board {
  output := Board{
    xExtent: []int{board.xExtent[0]-1, board.xExtent[1]+1},
    yExtent: []int{board.yExtent[0]-1, board.yExtent[1]+1},
    zExtent: []int{board.zExtent[0]-1, board.zExtent[1]+1},
    data: zAxis{},
  }

  for z := output.zExtent[0]; z < output.zExtent[1]; z++ {
    output.data[z] = yAxis{}
    for y := output.yExtent[0]; y < output.yExtent[1]; y++ {
      output.data[z][y] = xAxis{}
      for x := output.xExtent[0]; x < output.xExtent[1]; x++ {
        current := board.data[z][y][x]
        occupied := countFunction(board, x, y, z)
        if current == '#' && (occupied == 2 || occupied == 3) {
          output.data[z][y][x] = '#'
        } else if (current == '.' || current == 0) && occupied == 3 {
          output.data[z][y][x] = '#'
        } else {
          output.data[z][y][x] = '.'
        }
      }
    }
  }
  return output
}

func calculateBoard(board Board) int {
  total := 0
  for z := board.zExtent[0]; z < board.zExtent[1]; z++ {
    for y := board.yExtent[0]; y < board.yExtent[1]; y++ {
      for x := board.xExtent[0]; x < board.xExtent[1]; x++ {
        if board.data[z][y][x] == '#' { total++ }
      }
    }
  }
  return total
}

func printBoard(board Board) {
  for z := board.zExtent[0]; z < board.zExtent[1]; z++ {
    for y := board.yExtent[0]; y < board.yExtent[1]; y++ {
      row := []rune{}
      for x := board.xExtent[0]; x < board.xExtent[1]; x++ {
        row = append(row, board.data[z][y][x])
      }
      log.Print(string(row))
    }
    log.Print("")
  }
}

func main() {
  input := readFile("./input.txt")

  rawBoard := strings.Split(strings.TrimRight(input, "\n"), "\n")
  board := Board{
    zExtent: []int{0,1},
    yExtent: []int{0, len(rawBoard)},
    xExtent: []int{0, len(rawBoard[0])},
    data: zAxis{},
  }

  board.data[0] = yAxis{}
  for y, row := range rawBoard {
    board.data[0][y] = xAxis{}
    for x, col := range row {
      board.data[0][y][x] = col
    }
  }

  //Challenge 1
  {
    printBoard(board)
    for i := 0; i < 6; i++ {
      board = stepBoard(board, countOccupiedAdjacent)
      printBoard(board)
    }
    log.Print(calculateBoard(board))
  }
}
