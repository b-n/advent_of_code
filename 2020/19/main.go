package main
import (
  "log"
  "io/ioutil"
  "strings"
  "strconv"
  "regexp"
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
  raw               [][]int
  compiled_options  []string
}

func parseRules(input []string) map[int]*Rule {
  rules := map[int]*Rule{}
  for _, v := range input {
    parts := strings.Split(v, ": ")
    index := safeAtoi(parts[0])

    rule := Rule{
      raw: [][]int{},
      compiled_options: []string{},
    }
    if parts[1] == "\"a\"" || parts[1] == "\"b\"" {
      rule.compiled_options = append(rule.compiled_options, string(parts[1][1]))
      rules[index] = &rule
      continue
    }

    for i, section := range strings.Split(parts[1], " | ") {
      rule.raw = append(rule.raw, []int{})
      for _, num := range strings.Split(section, " ") {
        rule.raw[i] = append(rule.raw[i], safeAtoi(num))
      }
    }
    rules[index] = &rule
  }
  return rules
}

func combineArrays(input [][]string) []string {
  res := []string{""}
  for _, arr := range input {
    prod :=  []string{}
    for _, i := range arr {
      for _, r := range res {
        prod = append(prod, r + i)
      }
    }
    res = prod
  }
  return res
}

func rule_options(ruleNumber int, rules map[int]*Rule) []string {
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
    rule_outputs := [][]string{}
    infinite := false
    for _, sub := range subRule {
      if sub == ruleNumber { 
        infinite = true;
        continue
      }
      rule_outputs = append(rule_outputs, rule_options(sub, rules))
    }
    if infinite {
      for i, o := range rule_outputs {
        for j, v := range o {
          rule_outputs[i][j] = "(" + v + ")+"
        }
      }
    }

    for _, o := range combineArrays(rule_outputs) {
      compiled_options = append(compiled_options, o)
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
  if false {
    valid_options := rule_options(0, rules)
    log.Print(len(valid_options))
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
  if true {
    rules[8].raw[0] = []int{42, 8}
    rules[11].raw[0] = []int{42,11,31}

    valid_options := rule_options(0, rules)
    log.Print(len(valid_options))
    regexs := make([]*regexp.Regexp, len(valid_options))
    log.Print("Compiling Regexs")
    for i, opt := range valid_options {
      regexs[i] = regexp.MustCompile("^" + opt + "$")
      if i % 100000 == 0 { log.Print(i) }
    }

    valid := 0
    for c, m := range messages {
      log.Print(c, " ", m)
      good := false
      for _, r := range regexs {
        if r.MatchString(m) {
          log.Print(m, r.String())
          good = true
          break
        }
      }
      if good { valid++ }
    }
    log.Print(valid)
  }
}
