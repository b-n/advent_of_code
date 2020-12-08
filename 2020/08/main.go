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

type Instruction struct {
  operation string
  offset int
}

type StateMachine struct {
  accumulator int
  counter int
  stack []Instruction
}

func nop(machine *StateMachine, _ int) {
  machine.counter++
}

func acc(machine *StateMachine, offset int) {
  machine.accumulator += offset
  machine.counter++
}

func jmp(machine *StateMachine, offset int) {
  machine.counter += offset
}

var commands = map[string](func(machine *StateMachine, offset int)){
  "nop": nop,
  "acc": acc,
  "jmp": jmp,
}

func runMachine(machine *StateMachine) {
  instructionCount := make([]int, len(machine.stack))

  for true {
    instruction := machine.stack[machine.counter]

    //increment instruction count, and break if same line is hit more than once
    instructionCount[machine.counter]++
    if (instructionCount[machine.counter] > 1) {
      break
    }

    command := commands[instruction.operation]
    command(machine, instruction.offset)

    // check for end condition
    if (machine.counter >= len(machine.stack)) { break }
  }
}

func main() {
  input := readFile("./input.txt")

  rawStack := strings.Split(strings.TrimRight(input, "\n"), "\n")

  stack := make([]Instruction, len(rawStack))

  for i, r := range rawStack {
    parts := strings.Fields(r)
    offset, _ := strconv.Atoi(parts[1][1:len(parts[1])])
    if (parts[1][0] == '-') { offset *= -1 }
    stack[i] = Instruction{
      operation: parts[0],
      offset: offset,
    }
  }


  // Challenge 1
  {
    machine := StateMachine{
      accumulator: 0,
      counter: 0,
      stack: stack,
    }

    runMachine(&machine)

    log.Print(machine.accumulator)
  }

  // Challenge 2
  {
    machine := StateMachine{
      accumulator: 0,
      counter: 0,
      stack: stack,
    }

    originalStack := make([]Instruction, len(stack))
    copy(originalStack, stack)

    for i, instruction := range machine.stack {

      switch op := instruction.operation; op {
      case "nop":
        machine.stack[i].operation = "jmp"
      case "jmp":
        machine.stack[i].operation = "nop"
      default:
        continue
      }

      // run the machine
      runMachine(&machine)

      // Check if machine completed successfully
      if (machine.counter >= len(machine.stack)) {
        log.Print("Accumulator at end of corrected instruction: ", machine.accumulator)
        break
      }

      //reset the machine
      machine.counter = 0
      machine.accumulator = 0
      copy(machine.stack, originalStack)
    }
  }
}
