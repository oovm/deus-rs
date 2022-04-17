# Lights Out Solver

## Basic

```rust
```


## Todo list


- https://mathworld.wolfram.com/LightsOutPuzzle.html

- https://zhuanlan.zhihu.com/p/21265602
- https://zhuanlan.zhihu.com/p/53646257
- http://yanhaijing.com/inverter/

```wolfram
t = {
  {1, 1, 0},
  {1, 1, 1},
  {0, 1, 1}
  }
i = IdentityMatrix[3]
m = ArrayFlatten[{
   {t, i, 0},
   {i, t, i},
   {0, i, t}
   }]
Inverse[m, Modulus -> 2] . Table[Subscript[x, i], {i, 9}] // TableForm
```
