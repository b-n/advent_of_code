package main
import (
  "log"
  "io/ioutil"
  "strings"
  "sort"
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

type Ingredient struct {
  Name string
  Allergen string
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

  allergenIngredients := map[string][]string{}
  // Challenge 1
  {
    ingredientCounts := map[string]int{}

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
    assignedIngredients := map[string]string{}
    assignedAllergens := map[string]bool{}
    for len(assignedIngredients) < len(allergenIngredients) {
      for k, v := range allergenIngredients {
        if assignedAllergens[k] { continue }
        total := 0
        ingredient := ""
        for _, i := range v {
          if _, ok := assignedIngredients[i]; !ok {
            ingredient = i
            total++
          }
        }
        if total == 1 {
          assignedIngredients[ingredient] = k
          assignedAllergens[k] = true
        }
      }
    }

    ingredients := make([]Ingredient, len(assignedIngredients))
    i := 0
    for k, v := range assignedIngredients {
      ingredients[i] = Ingredient{
        Name: k,
        Allergen: v,
      }
      i++
    }

    sort.Slice(ingredients, func(i, j int) bool {
      return ingredients[i].Allergen < ingredients[j].Allergen
    })

    output := make([]string, len(assignedIngredients))
    for i, k := range ingredients { output[i] = k.Name }
    log.Print(strings.Join(output,","))
  }
}
