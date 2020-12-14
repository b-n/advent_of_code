package main

import (
	"io/ioutil"
	"log"
	"strconv"
	"strings"
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

func safeAtoi(input string) uint64 {
  output, e := strconv.Atoi(input)
  if e != nil { return 0 }
  return uint64(output)
}

func applyLogicMask(input uint64, mask string) uint64 {
  orMask, _ := strconv.ParseUint(strings.ReplaceAll(mask, "X", "0"), 2, 64)
  andMask, _ := strconv.ParseUint(strings.ReplaceAll(mask, "X", "1"), 2, 64)
  return (input | orMask) & andMask
}

func floatingMaskFromAddr(addr uint64, mask string) string {
  strAddr := fmt.Sprintf("%036b", addr)[0:36]
  maskChars := make([]byte, len(strAddr))
  for i, c := range mask {
    switch c {
    case '0':
      maskChars[i] = strAddr[i]
    case '1':
      maskChars[i] = '1'
    case 'X':
      maskChars[i] = 'X'
    }
  }
  return string(maskChars)
}

func writeFloatingMemory(value uint64, memory map[uint64]uint64, mask string) {
  if !strings.Contains(mask, "X") {
    loc, _ := strconv.ParseUint(strings.ReplaceAll(mask, "X", "0"), 2, 64)
    memory[loc] = value
    return
  }
  writeFloatingMemory(value, memory, strings.Replace(mask, "X", "0", 1))
  writeFloatingMemory(value, memory, strings.Replace(mask, "X", "1", 1))
}

func main() {
  input := readFile("./input.txt")

  data := strings.Split(strings.TrimRight(input, "\n"), "\n")

  //Challenge 1
  {
    currentMask := ""
    memory := map[uint64]uint64{}
    for _, d := range data {
      switch key := d[0:3]; key {
      case "mas":
        currentMask = strings.Fields(d)[2]
      case "mem":
        parts := strings.Fields(d)
        addr := safeAtoi(parts[0][4:len(parts[0])-1])
        value := safeAtoi(parts[2])
        masked := applyLogicMask(value, currentMask)
        memory[addr] = masked
      }
    }
    var total uint64 = 0
    for _, v := range memory {
      total += v
    }
    log.Print(total)
  }

  //Challenge 2
  {
    currentMask := ""
    memory := map[uint64]uint64{}
    for _, d := range data {
      switch key := d[0:3]; key {
      case "mas":
        currentMask = strings.Fields(d)[2]
      case "mem":
        parts := strings.Fields(d)
        addr := safeAtoi(parts[0][4:len(parts[0])-1])
        value := safeAtoi(parts[2])
        mask := floatingMaskFromAddr(addr, currentMask)
        writeFloatingMemory(value, memory, mask)
      }
    }
    var total uint64 = 0
    for _, v := range memory {
      total += v
    }
    log.Print(total)
  }
}
