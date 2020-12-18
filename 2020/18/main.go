package main
import (
  "log"
  "io/ioutil"
  "strings"
  "strconv"
  "regexp"
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

func safeAtoi(input string) int {
  res, err := strconv.Atoi(input)
  check(err)
  return res
}

func solveL2R(input string) int {
  tokens := strings.Fields(input)
  res := safeAtoi(tokens[0])
  op := ""
  for i, v := range tokens {
    if i % 2 == 1 { op = tokens[i]; continue }
    val := safeAtoi(v)
    switch op {
    case "*":
      res *= val
    case "+":
      res += val
    }
  }
  return res
}

func solveReverse(input string) int {
  plusRegex := regexp.MustCompile("\\d*\\ \\+\\ \\d*")

  eq := input
  matched := plusRegex.MatchString(eq)
  for matched {
    subEqs := plusRegex.FindAllString(eq, 1)
    if len(subEqs) == 0 { break }
    sub := subEqs[0]
    vals := strings.Fields(sub)
    res := fmt.Sprintf("%d", safeAtoi(vals[0]) + safeAtoi(vals[2]))
    eq = strings.ReplaceAll(eq, sub, res)
    matched = plusRegex.MatchString(eq)
  }

  tokens := strings.Fields(eq)
  total := safeAtoi(tokens[0])
  for i := 1; i < len(tokens); i += 2 { total *= safeAtoi(tokens[i+1]) }

  return total
}

func main() {
  input := readFile("./test2.txt")

  rawEquations := strings.Split(strings.TrimRight(input, "\n"), "\n")

  // Challenge 1
  {
    parenRegex := regexp.MustCompilePOSIX("\\([^\\(\\)]*\\)")
    total := 0

    for _, eq := range rawEquations {
      matched := parenRegex.MatchString(eq)
      for matched {
        subEqs := parenRegex.FindAllString(eq, -1)
        for _, sub := range subEqs {
          res := fmt.Sprintf("%d", solveL2R(sub[1:len(sub)-1]))
          eq = strings.ReplaceAll(eq, sub, res)
        }
        matched = parenRegex.MatchString(eq)
      }

      val := solveL2R(eq)
      total += val
    }
    log.Print(total)
  }

  // Challenge 2
  {
    parenRegex := regexp.MustCompilePOSIX("\\([^\\(\\)]*\\)")
    total := 0

    for _, eq := range rawEquations {
      log.Print(eq)
      matched := parenRegex.MatchString(eq)
      for matched {
        subEqs := parenRegex.FindAllString(eq, -1)
        for _, sub := range subEqs {
          res := fmt.Sprintf("%d", solveReverse(sub[1:len(sub)-1]))
          eq = strings.ReplaceAll(eq, sub, res)
        }
        log.Print(eq)
        matched = parenRegex.MatchString(eq)
      }

      val := solveReverse(eq)
      log.Print(val)
      total += val
    }
    log.Print(total)
  }
}
