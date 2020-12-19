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

func safeAtoi(input string) int {
  res, err := strconv.Atoi(input)
  check(err)
  return res
}

type Rule struct {
  raw              [][]int
  compiled_options []string
}

func parseRules(input []string) map[int]Rule {
  rules := map[int]Rule{}
  for _, v := range input {
    parts := strings.Split(v, ": ")
    index := safeAtoi(parts[0])

    if parts[1] == "\"a\"" || parts[1] == "\"b\"" {
      rules[index] = Rule{
        compiled_options: []string{string(parts[1][1])},
      }
      continue
    }

    rule := Rule{
      raw: [][]int{},
      compiled_options: []string{},
    }
    for i, section := range strings.Split(parts[1], " | ") {
      rule.raw = append(rule.raw, []int{})
      for _, num := range strings.Split(section, " ") {
        rule.raw[i] = append(rule.raw[i], safeAtoi(num))
      }
    }
    rules[index] = rule
  }
  return rules
}

func rule_options(ruleNumber int, rules map[int]Rule) []string {
  if len(rules[ruleNumber].compiled_options) > 0 {
    return rules[ruleNumber].compiled_options
  }

  subRules := rules[ruleNumber].raw
  compiled_options := rules[ruleNumber].compiled_options
  for _, subRule := range subRules {
    if len(subRule) == 1 {
      for _, opt := range rule_options(subRule[0], rules) {
        compiled_options = append(compiled_options, opt)
      }
      continue
    }
    for _, opt1 := range rule_options(subRule[0], rules) {
      for _, opt2 := range rule_options(subRule[1], rules) {
        compiled_options = append(compiled_options, opt1 + opt2)
      }
    }
  }
  return compiled_options
}


func main() {
  input := readFile("./input.txt")

  parts := strings.Split(strings.TrimRight(input, "\n"), "\n\n")
  rawRules := strings.Split(parts[0], "\n")
  messages := strings.Split(parts[1], "\n")

  rules := parseRules(rawRules)

  // Challenge 1
  {
    valid_options := rule_options(0, rules)
    valid := 0
    for _, m := range messages {
      good := false
      for _, opt := range valid_options {
        if m == opt { good = true; break }
      }
      if good { valid++ }
    }
    log.Print(valid)
  }

  // Challenge 2
  {
  }
}
