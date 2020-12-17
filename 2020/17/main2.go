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
type aAxis = map[int]zAxis

type Board struct {
  xExtent []int
  yExtent []int
  zExtent []int
  aExtent []int
  data aAxis
}
type countFunc = func(board Board, x int, y int, z int, a int) int

func countOccupiedAdjacent(board Board, x int, y int, z int, a int) int {
  occupied := 0
  for i := 0; i < 81; i++ {
    if (i == 40) { continue }
    posA := a + (i / 27) - 1
    posZ := z + ((i % 27) / 9) - 1
    posY := y + ((i % 9) / 3) - 1
    posX := x + (i % 3) - 1
    if board.data[posA][posZ][posY][posX] == '#' {
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
    aExtent: []int{board.aExtent[0]-1, board.aExtent[1]+1},
    data: aAxis{},
  }

  for a := output.aExtent[0]; a < output.aExtent[1]; a++ {
    output.data[a] = zAxis{}
    for z := output.zExtent[0]; z < output.zExtent[1]; z++ {
      output.data[a][z] = yAxis{}
      for y := output.yExtent[0]; y < output.yExtent[1]; y++ {
        output.data[a][z][y] = xAxis{}
        for x := output.xExtent[0]; x < output.xExtent[1]; x++ {
          current := board.data[a][z][y][x]
          occupied := countFunction(board, x, y, z, a)
          if current == '#' && (occupied == 2 || occupied == 3) {
            output.data[a][z][y][x] = '#'
          } else if (current == '.' || current == 0) && occupied == 3 {
            output.data[a][z][y][x] = '#'
          } else {
            output.data[a][z][y][x] = '.'
          }
        }
      }
    }
  }
  return output
}

func calculateBoard(board Board) int {
  total := 0
  for a := board.aExtent[0]; a < board.aExtent[1]; a++ {
    for z := board.zExtent[0]; z < board.zExtent[1]; z++ {
      for y := board.yExtent[0]; y < board.yExtent[1]; y++ {
        for x := board.xExtent[0]; x < board.xExtent[1]; x++ {
          if board.data[a][z][y][x] == '#' { total++ }
        }
      }
    }
  }
  return total
}

func printBoard(board Board) {
  for a := board.aExtent[0]; a < board.aExtent[1]; a++ {
    for z := board.zExtent[0]; z < board.zExtent[1]; z++ {
      for y := board.yExtent[0]; y < board.yExtent[1]; y++ {
        row := []rune{}
        for x := board.xExtent[0]; x < board.xExtent[1]; x++ {
          row = append(row, board.data[a][z][y][x])
        }
        log.Print(string(row))
      }
      log.Print("")
    }
    log.Print("")
  }
}

func main() {
  input := readFile("./input.txt")

  rawBoard := strings.Split(strings.TrimRight(input, "\n"), "\n")
  board := Board{
    aExtent: []int{0,1},
    zExtent: []int{0,1},
    yExtent: []int{0, len(rawBoard)},
    xExtent: []int{0, len(rawBoard[0])},
    data: aAxis{},
  }

  board.data[0] = zAxis{}
  board.data[0][0] = yAxis{}
  for y, row := range rawBoard {
    board.data[0][0][y] = xAxis{}
    for x, col := range row {
      board.data[0][0][y][x] = col
    }
  }

  //Challenge 2
  {
    printBoard(board)
    for i := 0; i < 6; i++ {
      board = stepBoard(board, countOccupiedAdjacent)
      printBoard(board)
    }
    log.Print(calculateBoard(board))
  }
}
