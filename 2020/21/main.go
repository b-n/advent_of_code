package main
import (
  "log"
  "io/ioutil"
  "strings"
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

type Food struct {
  Ingredients []string
  Allergens []string
}

func main() {
  input := readFile("./input.txt")

  rawFoods := strings.Split(strings.TrimRight(input, "\n"), "\n")
  foods := make([]Food, len(rawFoods))
  for i, f := range rawFoods {
    parts := strings.Split(f, " (contains ")
    foods[i] = Food{
      Ingredients: strings.Split(parts[0], " "),
      Allergens: strings.Split(strings.TrimRight(parts[1],")\n"), ", "),
    }
  }

  // Challenge 1
  {
    ingredientCounts := map[string]int{}
    allergenIngredients := map[string][]string{}

    for _, f := range foods {
      for _, i := range f.Ingredients {
        ingredientCounts[i]++
      }

      for _, a := range f.Allergens {
        if _, ok := allergenIngredients[a]; ok {
          // we only want to keep ingredients that are in both lists
          intersectionIngredients := []string{}
          for _, fi := range f.Ingredients {
            for _, ai := range allergenIngredients[a] {
              if fi == ai {
                intersectionIngredients = append(intersectionIngredients, fi)
              }
            }
          }
          allergenIngredients[a] = intersectionIngredients
        } else {
          allergenIngredients[a] = f.Ingredients
        }
      }
    }

    suspectIngredients := map[string]bool{}
    for _, ingredients := range allergenIngredients {
      for _, i := range ingredients {
        suspectIngredients[i] = true
      }
    }

    total := 0
    for k, v := range ingredientCounts {
      if !suspectIngredients[k] { total += v }
    }
    log.Print(total)
  }

  // Challenge 2
  {
  }
}



