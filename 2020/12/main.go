package main

import (
	"io/ioutil"
	"log"
	"strconv"
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

type Ship struct {
  posX, posY int
  dir int
}

func moveNorth(s *Ship, distance int) { s.posY -= distance }
func moveSouth(s *Ship, distance int) { s.posY += distance }
func moveEast(s *Ship, distance int) { s.posX += distance }
func moveWest(s *Ship, distance int) { s.posX -= distance }
func turnLeft(s *Ship, distance int) { s.dir = (s.dir + 360 - distance) % 360 }
func turnRight(s *Ship, distance int) { s.dir = (s.dir + distance) % 360 }
func moveForward(s *Ship, distance int) {
  if s.dir == 0 { moveNorth(s, distance) }
  if s.dir == 90 { moveEast(s, distance) }
  if s.dir == 180 { moveSouth(s, distance) }
  if s.dir == 270 { moveWest(s, distance) }
}

type movementFunction = func(s *Ship, distance int)

var commands = map[byte]movementFunction{
  'N': moveNorth,
  'S': moveSouth,
  'E': moveEast,
  'W': moveWest,
  'L': turnLeft,
  'R': turnRight,
  'F': moveForward,
}

func main() {
  input := readFile("./input.txt")

  instructions := strings.Split(strings.TrimRight(input, "\n"), "\n")

  //Challenge 1
  {
    boatyMcBoatFace := Ship{dir: 90}
    for _, i := range instructions {
      command := i[0]
      distance,_ := strconv.Atoi(i[1:])
      commands[command](&boatyMcBoatFace, distance)
    }
    log.Print(boatyMcBoatFace.posX + boatyMcBoatFace.posY)
  }

  //Challenge 2
  {
  }
}
