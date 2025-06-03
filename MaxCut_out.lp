\ SCIP STATISTICS
\   Problem name     : /Users/jflxb/Documents/aq-models-rs/MaxCut.lp
\   Variables        : 6 (5 binary, 0 integer, 0 implicit integer, 1 continuous)
\   Constraints      : 1
Minimize
 Obj: -6 x_0 -6 x_1 -6 x_2 -4 x_3 -6 x_4 +1 quadobjvar +7
Subject to
 quadobj: -1 quadobjvar + [ +4 x_0 * x_1 +4 x_0 * x_2 +4 x_0 * x_4 +4 x_1 * x_2 +4 x_1 * x_4 +4 x_2 * x_3
  +4 x_3 * x_4 ] <= +0
Bounds
 0 <= x_0 <= 1
 0 <= x_1 <= 1
 0 <= x_2 <= 1
 0 <= x_3 <= 1
 0 <= x_4 <= 1
 quadobjvar free
Binaries
 x_0 x_1 x_2 x_3 x_4
End
