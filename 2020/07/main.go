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

type Constraint struct {
  quantity int
  bagColor string
}

type Rule struct {
  bagColor string
  constraints []Constraint
}

func getRule(input string) Rule {
  parts := strings.Split(input, " bags contain ")
  rawConstraints := strings.Split(strings.TrimRight(parts[1], "."), ", ")
  res := Rule{bagColor: parts[0], constraints: make([]Constraint, len(rawConstraints))}
  for i, c := range rawConstraints {
    vals := strings.Fields(c)
    quantity, _ := strconv.Atoi(vals[0])
    res.constraints[i] = Constraint{
      quantity: quantity,
      bagColor:  strings.Join(vals[1:len(vals)-1], " "),
    }
  }

  return res
}

func getColorCapacity(searchColor string, rule Rule, dict map[string]Rule) int {
  capacity := 0

  for _, c := range rule.constraints {
    if c.bagColor == searchColor {
      capacity += c.quantity
    } else {
      capacity += getColorCapacity(searchColor, dict[c.bagColor], dict)
    }
  }

  return capacity
}

func getTotalCapacity(rule Rule, dict map[string]Rule) int {
  capacity := 1
  for _, c := range rule.constraints {
    if _, ok := dict[c.bagColor]; ok { 
      capacity += (c.quantity * getTotalCapacity(dict[c.bagColor], dict))
    }
  }
  return capacity
}

func main() {
  input := readFile("./input.txt")

  rawRules := strings.Split(input, "\n")

  rules := make(map[string]Rule, len(rawRules))

  for _, r := range rawRules {
    if r == "" { continue }
    rule := getRule(r)
    rules[rule.bagColor] = rule
  }

  // Challenge 1
  {
    bags := 0
    for _, r := range rules {
      capacity := getColorCapacity("shiny gold", r, rules)
      if capacity > 0 { bags++ }
    }
    log.Print(bags)
  }

  // Challenge 2
  {
    totalCapacity := getTotalCapacity(rules["shiny gold"], rules)
    // - 1, because the totalCapacity assumes you're counting the value of the bag too
    // so lets remove that
    log.Print(totalCapacity - 1)
  }
}
