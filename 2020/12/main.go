package main

import (
	"io/ioutil"
	"log"
	"strconv"
	"strings"
  "math"

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

func deg2rad(deg int) float64 {
  return float64(deg) * math.Pi / 180
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
func rotate(s *Ship, distance int) {
  rad := deg2rad(distance)
  sin, cos := math.Sincos(rad)
  cPosX, cPosY := float64(s.posX), float64(s.posY)
  s.posX = int(math.Round((cPosX * cos) - (cPosY * sin)))
  s.posY = int(math.Round((cPosX * sin) + (cPosY * cos)))
}
func rotateLeft(s *Ship, distance int) { rotate(s, distance * -1) }
func rotateRight(s *Ship, distance int) { rotate(s, distance) }


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
    wayPoint := Ship{posX: 10, posY: -1}
    boatyMcBoatFace := Ship{}
    for _, i := range instructions {
      command := i[0]
      distance, _ := strconv.Atoi(i[1:])
      // move Waypoint
      if command == 'N' || command == 'S' || command == 'E' || command == 'W' {
        commands[command](&wayPoint, distance)
      }
      // move boaty
      if command == 'F' {
        boatyMcBoatFace.posX += wayPoint.posX * distance
        boatyMcBoatFace.posY += wayPoint.posY * distance
      }
      // rotate
      if command == 'L' { rotateLeft(&wayPoint, distance) }
      if command == 'R' { rotateRight(&wayPoint, distance) }
    }
    log.Print(boatyMcBoatFace.posX + boatyMcBoatFace.posY)
  }
}
