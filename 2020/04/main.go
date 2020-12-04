package main

import (
  "log"
  "io/ioutil"
  "strconv"
  "strings"
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

type Passport struct {
  fields map[string]string
}

func getPassports(input string) []Passport {
  lines := strings.Split(input, "\n")

  totalPassports := 0
  for _, v := range lines {
    if v == "" { totalPassports++ }
  }

  passports := make([]Passport, totalPassports)
  totalPassports = 0
  currentPassport := Passport{}
  currentPassport.fields = make(map[string]string)

  for _, line := range lines {
    if line == "" {
      passports[totalPassports] = currentPassport
      currentPassport = Passport{}
      currentPassport.fields = make(map[string]string)
      totalPassports++
      continue
    }
    for _, field := range strings.Fields(line) {
      values := strings.Split(field, ":")
      currentPassport.fields[values[0]] = values[1]
    }
  }
  return passports
}

func validateNumber(input string, min int, max int) bool {
  reg := regexp.MustCompile("[^0-9]+")

  num, _ := strconv.Atoi(reg.ReplaceAllString(input, ""))

  return (num <= max && num >= min)
}


func main() {
  input := readFile("./input.txt")

  passports := getPassports(input)

  requiredFields := []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

  // Challenge 1
  {
    validPassports := 0
    for _, p := range passports {
      validFields := 0
      for _, requiredField := range requiredFields {
        if p.fields[requiredField] != "" { validFields++ }
      }
      if validFields >= 7 { validPassports++ }
    }
    log.Print(validPassports)
  }


  // Challenge 2
  {
    hclReg := regexp.MustCompile("#[a-f0-9]{6}")
    eyeColReg := regexp.MustCompile("amb|blu|brn|gry|grn|hzl|oth")
    pidReg := regexp.MustCompile("[0-9]{9}")

    validPassports := 0
    for _, p := range passports {
      validFields := 0
      for _, requiredField := range requiredFields {
        value := p.fields[requiredField]
        if requiredField == "byr" {
          if validateNumber(value, 1920, 2002) {
            validFields++
          }
        } else if requiredField == "iyr" {
          if validateNumber(value, 2010, 2020) {
            validFields++
          }
        } else if requiredField == "eyr" {
          if validateNumber(value, 2020, 2030) {
            validFields++
          }
        } else if requiredField == "hgt" {
          if strings.Contains(value, "in") {
            if validateNumber(value, 59, 76) {
              validFields++
            }
          } else if strings.Contains(value, "cm") {
            if validateNumber(value, 150, 193) {
              validFields++
            }
          }
        } else if requiredField == "hcl" {
          match := hclReg.MatchString(value)
          if match {
            validFields++
          }
        } else if requiredField == "ecl" {
          match := eyeColReg.MatchString(value)
          if match {
            validFields++
          }
        } else if requiredField == "pid" {
          match := pidReg.MatchString(value)
          if match {
            validFields++
          }
        } else {
          if value != "" { validFields++ }
        }
      }
      if validFields >= 7 { validPassports++ }
    }
    log.Print(validPassports)
  }
}
