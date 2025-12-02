\ Model TestModel
\ Problem name: TestModel

Minimize
 obj: [ 11.213 x0 * x1 - 3 x0 * x2 + 1848482 x0 * x3 + 0.5 x1 * x2 + x1 * x4 ] /       2 + x0 + 1
Subject To
 c0: x0 + x2 <= 1
 my_constraint: x0 + x2 <= 1
Bounds
 x1 >= 0
 0 <= x3 <= 30
Semi-Continuous
 x1
Binaries
 x0 x2 x4
Generals
 x3
End
