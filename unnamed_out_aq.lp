\ Model unnamed
\ Problem name: unnamed

Minimize
 obj: 3 x_0 + x_1 - 2 x_2 + 2 x_3 + 4 x_4 - 6 x_5 + 7 x_6 - 4 x_7 + quadobjvar
Subject To
 quadobj: [ -16 x_0 * x_3 + 8 x_0 * x_5 + 20 x_0 * x_7 + 28 x_1 * x_2 - 12 x_1 * x_4 + 16 x_1 * x_6 + 24 x_2 * x_4 - 20 x_2 * x_6 + 32 x_3 * x_5 - 4 x_3 * x_7 - 8 x_4 * x_6 + 12 x_5 * x_7 ] <= 0
Bounds
 quadobjvar free
Binaries
 x_0 x_1 x_2 x_3 x_4 x_5 x_6 x_7
Semi-Continuous
 quadobjvar
End