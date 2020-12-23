package main

import (
	"container/ring"
  "log"
)

func getPickups(r *ring.Ring) []int {
  res := make([]int, r.Len())
  i := 0
  r.Do(func(p interface{}) {
    res[i] = p.(int)
    i++
  })
  return res
}

func checkValue(arr []int, value int, maxValue int) int {
  res := value
  vMap := map[int]bool{}
  for _, v := range arr { vMap[v] = true }
  for i := 0; i < 4; i++ {
    if !vMap[res] { break }
    res--
    if res == 0 { res = maxValue }
  }
  return res
}

func goToInt(r *ring.Ring, value int) *ring.Ring {
  for true {
    if r.Value.(int) == value { break }
    r = r.Next()
  }
  return r
}

func play(r *ring.Ring, positions map[int]*ring.Ring, rounds int, maxValue int) *ring.Ring {
  for i := 0; i < rounds; i++ {
    // get our safe destination value
    destination := r.Value.(int) - 1
    if destination == 0 { destination = maxValue }

    //get those pickups (unlink) and fix the destination value
    pickUpRing := r.Unlink(3)
    pickUps := getPickups(pickUpRing)
    destination = checkValue(pickUps, destination, maxValue)

    // where to after this?
    nextHead := r.Next()

    // find the destination number from above, and link the pickups in
    r = positions[destination]
    r = r.Link(pickUpRing)

    // Go to the next head
    r = nextHead
  }
  return r
}

func main() {
  // test
  //circle := []int{3, 8, 9, 1, 2, 5, 4, 6, 7}
  // input
  circle := []int{6, 2, 4, 3, 9, 7, 1, 5, 8}

  // Challenge 1
  {
    positions := map[int]*ring.Ring{}
    r := ring.New(len(circle))
    for _, v := range circle {
      positions[v] = r
      r.Value = v
      r = r.Next()
    }

    r = play(r, positions, 100, 9)

    r = positions[1].Next()
    for i := 0; i < len(circle) - 1; i++ {
      log.Print(r.Value.(int))
      r = r.Next()
    }
  }
  // challenge 2
  {
    biggerCircle := make([]int, 1000000)
    copy(biggerCircle, circle)
    for i := 10; i <= 1000000; i++ {
      biggerCircle[i-1] = i
    }

    positions := map[int]*ring.Ring{}
    r := ring.New(len(biggerCircle))
    for _, v := range biggerCircle {
      positions[v] = r
      r.Value = v
      r = r.Next()
    }

    r = play(r, positions, 10000000, 1000000)

    r = positions[1].Next()
    total := 1
    total *= r.Value.(int)
    r = r.Next()
    total *= r.Value.(int)
    log.Print(total)
  }
}
