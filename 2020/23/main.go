package main
import (
  "log"
)

func check(e error) {
  if e != nil {
    log.Fatal(e)
    panic(e)
  }
}

func wrappedPick(circle []int, from int, quantity int) []int {
  res := circle
  res = append(res, circle...)
  return res[from:from+quantity]
}

func removeElements(source []int, toRemove []int) []int {
  rMap := map[int]bool{}
  for _, r := range toRemove { rMap[r] = true }
  res := []int{}
  for _, v := range source {
    if !rMap[v] { res = append(res, v) }
  }
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

func play(input []int, rounds int, maxValue int) []int {
  circle := input
  for i := 0; i < rounds; i++ {
    if i % 10000 == 0 { log.Print("Round ", i) }
    destination := circle[0] - 1
    if (destination == 0) { destination = maxValue }
    pickUps := circle[1:4]
    destination = checkValue(pickUps, destination, maxValue)

    res := make([]int, len(circle))
    res[len(circle)-1] = circle[0]
    pos := 4
    for j := 0; j < len(circle) - 1; j++ {
      res[j] = circle[pos]
      if res[j] == destination {
        res[j+1] = pickUps[0]
        res[j+2] = pickUps[1]
        res[j+3] = pickUps[2]
        j += 3
      }
      pos++
    }
    circle = res
  }
  return circle
}

func main() {
  //test
  //circle := []int{3, 8, 9, 1, 2, 5, 4, 6, 7}
  //input
  circle := []int{6, 2, 4, 3, 9, 7, 1, 5, 8}

  // Challenge 1
  {
    smallCircle := make([]int, len(circle))
    copy(smallCircle, circle)
    smallCircle = play(smallCircle, 100, 9)
    res := []int{}
    i := 0
    for ; i < 9; i++ { if smallCircle[i] == 1 { break } }
    i++
    for len(res) < 8 {
      res = append(res, smallCircle[i])
      i++
      i = i % 9
    }
    log.Print(res)
  }

  // Challenge 2
  {
    biggerCircle := make([]int, 1000000)
    copy(biggerCircle, circle)
    for i := 10; i <= 1000000; i++ {
      biggerCircle[i-1] = i
    }
    log.Print(biggerCircle[:10], biggerCircle[999990:])
    res := play(biggerCircle, 10000000, 1000000)
    for i, v := range res {
      if v == 1 {
        log.Print(res[i], res[i+1], res[i+2])
        break
      }
    }
    log.Print(len(res))
  }
}
