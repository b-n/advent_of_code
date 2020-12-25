package main
import (
  "log"
)

func transform(subjectMatter int, res int) int {
  res = res * subjectMatter
  res = res % 20201227
  return res
}

func solve(target int) int {
  i := 1
  res := 1
  subjectMatter := 7
  for true {
    res = transform(subjectMatter, res)
    if res == target { break }
    i++
  }

  return i
}

func encryptionKey(publicKey int, loops int) int {
  res := 1
  for i := 0; i < loops; i++ {
    res = transform(publicKey, res)
  }
  return res
}

func main() {

  // input
  cardPublic := 6929599
  doorPublic := 2448427

  // test
  //cardPublic := 5764801
  //doorPublic := 17807724

  // Challenge 1
  {
    cardLoops := solve(cardPublic)
    doorLoops := solve(doorPublic)

    // start with the one with the lowest loops (faster)
    start := cardPublic
    loops := doorLoops
    if (cardLoops < loops) {
      start = doorPublic
      loops = cardLoops
    }

    log.Print(encryptionKey(start, loops))
  }
}
