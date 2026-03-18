* Signature: 0x991fc0f1ceb540f4
NAME mixed_vtypes
ROWS
 N  OBJ
 L  mix_c1  
 G  mix_c2  
 E  mix_c3  
COLUMNS
    MARKER    'MARKER'                 'INTORG'
    b1        OBJ       1
    b1        mix_c1    1
    b2        OBJ       2
    b2        mix_c1    1
    i1        OBJ       3
    i1        mix_c1    1
    i1        mix_c3    1
    MARKER    'MARKER'                 'INTEND'
    r1        OBJ       0.5
    r1        mix_c2    1
    r1        mix_c3    -1
    r2        OBJ       -1
    r2        mix_c2    1
RHS
    RHS1      mix_c1    8
    RHS1      mix_c2    5
BOUNDS
 BV BND1      b1      
 BV BND1      b2      
 UP BND1      i1        10
 UP BND1      r1        100
 FR BND1      r2      
ENDATA
