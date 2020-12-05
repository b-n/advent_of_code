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

func extent(input []int) (int, int) {
  min, max := 9999, 0
  for _, val := range input {
    if (val < min) { min = val }
    if (val > max) { max = val }
  }
  return min, max
}

func seatFromString(input string) (int, int) {
  replacer := strings.NewReplacer("F", "0", "B", "1", "R", "1", "L", "0")

  binaryString := replacer.Replace(input)

  row, _ := strconv.ParseInt(binaryString[0:7], 2, 8)
  col, _ := strconv.ParseInt(binaryString[7:10], 2, 8)

  return int(row), int(col)
}

func seatId(row, col int) int {
  return (row * 8 + col)
}

func main() {
  input := readFile("./input.txt")

  rows := strings.Fields(input)

  // Challenge 1
  {
    seats := make([]int, len(rows))
    for i, seat := range rows {
      row, col := seatFromString(seat)
      seats[i] = seatId(row, col)
    }
    _, maxSeat := extent(seats) 
    log.Print("Max Seat: ", maxSeat)
  }

  // Challenge 2
  {
    seats := make([]int, len(rows))
    for i, seat := range rows {
      seats[i] = seatId(seatFromString(seat))
    }
    _, maxSeat := extent(seats)
    allSeats := make([]int, maxSeat + 1)
    for _, v := range seats { allSeats[v] = 1 }

    log.Print("Missing Seat(s): (unknown number of missing front row seats)")
    for i := 0; i < maxSeat; i++ {
      if (allSeats[i] == 0) { log.Print(i)}
    }
  }
}
