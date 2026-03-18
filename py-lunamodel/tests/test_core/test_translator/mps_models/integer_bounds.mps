* Signature: 0x8e9c4b6d7b8ea636
NAME integer_bounds
OBJSENSE MAX
ROWS
 N  OBJ
 L  con_le  
 G  con_ge  
 E  con_eq  
 L  con_free_neg
 G  con_free_ge
COLUMNS
    MARKER    'MARKER'                 'INTORG'
    i_lb_only  OBJ       1
    i_lb_only  con_le    1
    i_lb_only  con_ge    1
    i_lb_only  con_free_ge  -1
    i_ub_only  OBJ       2
    i_ub_only  con_le    1
    i_ub_only  con_eq    1
    i_both    OBJ       -1
    i_both    con_le    1
    i_both    con_ge    -1
    i_free    OBJ       0.5
    i_free    con_free_neg  1
    i_free    con_free_ge  1
    i_neg     OBJ       -1
    i_neg     con_free_neg  1
    MARKER    'MARKER'                 'INTEND'
RHS
    RHS1      con_le    15
    RHS1      con_ge    1
    RHS1      con_eq    5
    RHS1      con_free_ge  -10
BOUNDS
 LI BND1      i_lb_only  2
 MI BND1      i_ub_only
 UP BND1      i_ub_only  10
 LO BND1      i_both    -3
 UP BND1      i_both    7
 FR BND1      i_free  
 LO BND1      i_neg     -8
 UP BND1      i_neg     -1
ENDATA
