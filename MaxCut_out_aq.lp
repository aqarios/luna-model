\ Model unnamed
\ Problem name: unnamed

Minimize
 obj: -6 x_0 - 6 x_1 - 6 x_2 - 4 x_3 - 6 x_4 + quadobjvar + 7
Subject To
 quadobj: [ 8 x_0 * x_1 + 8 x_0 * x_2 + 8 x_0 * x_4 + 8 x_1 * x_2 + 8 x_1 * x_4 + 8 x_2 * x_3 + 8 x_3 * x_4 ] <= 0
Bounds
 quadobjvar free
Binaries
 x_0 x_1 x_2 x_3 x_4
Semi-Continuous
 quadobjvar
End