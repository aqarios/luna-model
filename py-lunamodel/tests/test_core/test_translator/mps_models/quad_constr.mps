* Signature: 0xaef9cad60bb7680f
NAME quad_constr
ROWS
 N  OBJ
 L  lc1     
 L  qc1     
 G  qc2     
COLUMNS
    x         OBJ       1
    x         lc1       1
    y         OBJ       1
    y         lc1       1
    z         OBJ       1
    z         lc1       1
RHS
    RHS1      lc1       15
    RHS1      qc1       25
    RHS1      qc2       1
BOUNDS
 UP BND1      x         10
 UP BND1      y         10
 UP BND1      z         10
QCMATRIX   qc1     
    x         x         1
    x         y         0.5
    y         x         0.5
    y         y         1
QCMATRIX   qc2     
    x         z         -1
    z         x         -1
    z         z         1
ENDATA
