* Signature: 0x885e71c046639a0f
NAME quad_obj_and_constr
OBJSENSE MAX
ROWS
 N  OBJ
 L  lc1     
 G  lc2     
 L  qc1     
COLUMNS
    x         lc1       1
    MARKER    'MARKER'                 'INTORG'
    y         OBJ       -1
    y         lc1       1
    y         lc2       1
    z         OBJ       4
    z         lc1       1
    z         lc2       -1
    MARKER    'MARKER'                 'INTEND'
RHS
    RHS1      lc1       10
    RHS1      lc2       -1
    RHS1      qc1       20
BOUNDS
 UP BND1      x         5
 LO BND1      y         -2
 UP BND1      y         8
 BV BND1      z       
QUADOBJ
    x         x         2
    x         y         2
QCMATRIX   qc1     
    x         x         1
    y         y         1
ENDATA
