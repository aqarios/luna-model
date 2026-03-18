* Signature: 0xb0499d53abf810c
NAME continuous_bounds
ROWS
 N  OBJ
 L  con_le  
 G  con_ge  
 E  con_eq  
 L  con_free_neg
 G  con_free_ge
COLUMNS
    c_lb_only  OBJ       3.5
    c_lb_only  con_le    1
    c_lb_only  con_ge    1
    c_lb_only  con_free_ge  -1
    c_ub_only  OBJ       -2
    c_ub_only  con_le    1
    c_ub_only  con_eq    1
    c_both    OBJ       1
    c_both    con_le    1
    c_both    con_ge    -1
    c_both    con_eq    1
    c_free    OBJ       0.5
    c_free    con_free_neg  1
    c_free    con_free_ge  1
    c_neg     OBJ       -1
    c_neg     con_free_neg  1
RHS
    RHS1      con_le    50
    RHS1      con_ge    0.5
    RHS1      con_eq    10
    RHS1      con_free_ge  -5
BOUNDS
 LO BND1      c_lb_only  1.5
 MI BND1      c_ub_only
 UP BND1      c_ub_only  100
 LO BND1      c_both    -2.5
 UP BND1      c_both    8.5
 FR BND1      c_free  
 LO BND1      c_neg     -10
 UP BND1      c_neg     -1
ENDATA
