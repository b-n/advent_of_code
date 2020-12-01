# Day 1

Why so complex? Why not. I went for optimization rather than "it works"

So, here's the thought:

* If we sort the list, and we start going over the list, our current value always goes up
* In theory the search space is reducing from the other end. Why? If the search space is
  sorted, then the higher in input, the lower the output
* So for each combination we check, we can go back from the end. We also know we can
  search from the max of the last execution.
* We're effectively moving towards the middle

E.g.

input:
1 2 3 4 5 6 7 8 9 14 15

search for numbers adding to 12

1. Start at 1
2. Is 1 + 15 = 16 ? max = end, next
2. Is 1 + 14 = 15 ? max = end - 1, next
3. IS 1 + 9 = 10 ? break
4. Start at 2
5. Is 2 + 14 = 16 ? max = end - 1, next
6. Is 2 + 9 = 11 ? break
7. Start at 3
8. 3 + 14 = 17 ? max = end - 1, next
9. 3 + 9 = 12 = WIN

By reducing the search space, we skipped out on one above after step 4, before step 5.

On a small data set this is somewhat pointless. On a bigger data set, much efficiency++?
