import gurobipy as grb
from pathlib import Path

OUT = Path(".")

# What is tested
#
# | # | File                        | Vtypes                          | Bounds                              | Constraints                                  | Objective                | Sense |
# |---|-----------------------------|---------------------------------|-------------------------------------|----------------------------------------------|--------------------------|-------|
# | 1 | binary_only.mps             | binary                          | default [0,1]                       | <=, >=, ==                                   | linear                   | min   |
# | 2 | integer_bounds.mps          | integer                         | lb-only, ub-only, both, free, neg   | <=, >=, ==                                   | linear                   | max   |
# | 3 | continuous_bounds.mps       | continuous                      | lb-only, ub-only, both, free, neg   | <=, >=, ==                                   | linear                   | min   |
# | 4 | mixed_vtypes.mps            | binary + integer + continuous   | mixed (incl. free)                  | <=, >=, ==                                   | linear                   | min   |
# | 5 | quad_obj.mps                | continuous + integer            | both                                | linear <=, >=                                | quadratic (QUADOBJ)      | min   |
# | 6 | quad_constr.mps             | continuous                      | both                                | quadratic <=, >= + linear <= (QCMATRIX)      | linear                   | min   |
# | 7 | quad_obj_and_constr.mps     | binary + integer + continuous   | mixed                               | quadratic <= + linear <=, >=                 | quadratic (QUADOBJ)      | max   |
# | 8a| maximize_with_offset.mps    | continuous + integer            | both                                | <=, >=                                       | linear + offset          | max   |
# | 8b| minimize_with_offset.mps    | continuous                      | both                                | <=, >=                                       | linear + offset          | min   |


## 1. Binary variables only


m = grb.Model("binary_only")
x1 = m.addVar(vtype=grb.GRB.BINARY, name="x1")
x2 = m.addVar(vtype=grb.GRB.BINARY, name="x2")
x3 = m.addVar(vtype=grb.GRB.BINARY, name="x3")
m.setObjective(2 * x1 + 3 * x2 - x3, grb.GRB.MINIMIZE)
# <= constraint
m.addConstr(x1 + x2 + x3 <= 2, name="c_le")
# >= constraint
m.addConstr(x1 - x2 >= 0, name="c_ge")
# == constraint
m.addConstr(x1 + x3 == 1, name="c_eq")
m.update()
m.write(str(OUT / "binary_only.mps"))


## 2. Integer variables — only lb, only ub, both, free, and negative bounds


m = grb.Model("integer_bounds")
# only lb
i_lb = m.addVar(vtype=grb.GRB.INTEGER, lb=2, ub=grb.GRB.INFINITY, name="i_lb_only")
# only ub
i_ub = m.addVar(vtype=grb.GRB.INTEGER, lb=-grb.GRB.INFINITY, ub=10, name="i_ub_only")
# both lb and ub
i_both = m.addVar(vtype=grb.GRB.INTEGER, lb=-3, ub=7, name="i_both")
# free integer
i_free = m.addVar(vtype=grb.GRB.INTEGER, lb=-grb.GRB.INFINITY, ub=grb.GRB.INFINITY, name="i_free")
# negative bounds
i_neg = m.addVar(vtype=grb.GRB.INTEGER, lb=-8, ub=-1, name="i_neg")

m.setObjective(i_lb + 2 * i_ub - i_both + 0.5 * i_free - i_neg, grb.GRB.MAXIMIZE)
# <= constraint
m.addConstr(i_lb + i_ub + i_both <= 15, name="con_le")
# >= constraint
m.addConstr(i_lb - i_both >= 1, name="con_ge")
# == constraint
m.addConstr(i_ub == 5, name="con_eq")
# constraints with free and negative vars
m.addConstr(i_free + i_neg <= 0, name="con_free_neg")
m.addConstr(i_free - i_lb >= -10, name="con_free_ge")
m.update()
m.write(str(OUT / "integer_bounds.mps"))



## 2. Integer variables — only lb, only ub, both, free, and negative bounds


m = grb.Model("continuous_bounds")
# only lb
c_lb = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=1.5, ub=grb.GRB.INFINITY, name="c_lb_only")
# only ub
c_ub = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=-grb.GRB.INFINITY, ub=100.0, name="c_ub_only")
# both lb and ub
c_both = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=-2.5, ub=8.5, name="c_both")
# free variable (no bounds)
c_free = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=-grb.GRB.INFINITY, ub=grb.GRB.INFINITY, name="c_free")
# negative bounds
c_neg = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=-10.0, ub=-1.0, name="c_neg")

m.setObjective(3.5 * c_lb - 2.0 * c_ub + c_both + 0.5 * c_free - c_neg, grb.GRB.MINIMIZE)
# <= constraint
m.addConstr(c_lb + c_ub + c_both <= 50, name="con_le")
# >= constraint
m.addConstr(c_lb - c_both >= 0.5, name="con_ge")
# == constraint
m.addConstr(c_ub + c_both == 10, name="con_eq")
# constraint involving free and negative-bounded vars
m.addConstr(c_free + c_neg <= 0, name="con_free_neg")
m.addConstr(c_free - c_lb >= -5, name="con_free_ge")
m.update()
m.write(str(OUT / "continuous_bounds.mps"))



## 4. Mixed variable types (binary + integer + continuous)

m = grb.Model("mixed_vtypes")
b1 = m.addVar(vtype=grb.GRB.BINARY, name="b1")
b2 = m.addVar(vtype=grb.GRB.BINARY, name="b2")
i1 = m.addVar(vtype=grb.GRB.INTEGER, lb=0, ub=10, name="i1")
r1 = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0.0, ub=100.0, name="r1")
r2 = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=-grb.GRB.INFINITY, ub=grb.GRB.INFINITY, name="r2")
m.setObjective(b1 + 2 * b2 + 3 * i1 + 0.5 * r1 - r2, grb.GRB.MINIMIZE)
m.addConstr(b1 + b2 + i1 <= 8, name="mix_c1")
m.addConstr(r1 + r2 >= 5, name="mix_c2")
m.addConstr(i1 - r1 == 0, name="mix_c3")
m.update()
m.write(str(OUT / "mixed_vtypes.mps"))


## 5. Quadratic objective (QUADOBJ section in MPS)
m = grb.Model("quad_obj")
x = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0, ub=10, name="x")
y = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0, ub=10, name="y")
z = m.addVar(vtype=grb.GRB.INTEGER, lb=0, ub=5, name="z")
# quadratic objective: x^2 + 2*x*y + 3*y^2 + x - 2*y + z
m.setObjective(x * x + 2 * x * y + 3 * y * y + x - 2 * y + z, grb.GRB.MINIMIZE)
m.addConstr(x + y + z <= 12, name="qo_c1")
m.addConstr(x - y >= -1, name="qo_c2")
m.update()
m.write(str(OUT / "quad_obj.mps"))


## 6. Quadratic constraints (QCMATRIX section in MPS)

m = grb.Model("quad_constr")
x = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0, ub=10, name="x")
y = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0, ub=10, name="y")
z = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0, ub=10, name="z")
m.setObjective(x + y + z, grb.GRB.MINIMIZE)
# quadratic constraint: x^2 + y^2 + x*y <= 25
m.addConstr(x * x + y * y + x * y <= 25, name="qc1")
# quadratic constraint: z^2 - 2*x*z >= 1
m.addConstr(z * z - 2 * x * z >= 1, name="qc2")
# linear constraint
m.addConstr(x + y + z <= 15, name="lc1")
m.update()
m.write(str(OUT / "quad_constr.mps"))


## 7. Quadratic objective + quadm = grb.Model("quad_obj_and_constr")

x = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0.0, ub=5.0, name="x")
y = m.addVar(vtype=grb.GRB.INTEGER, lb=-2, ub=8, name="y")
z = m.addVar(vtype=grb.GRB.BINARY, name="z")
# quadratic objective
m.setObjective(x * x + 2 * x * y - y + 4 * z, grb.GRB.MAXIMIZE)
# quadratic constraint
m.addConstr(x * x + y * y <= 20, name="qc1")
# linear constraints
m.addConstr(x + y + z <= 10, name="lc1")
m.addConstr(y - z >= -1, name="lc2")
m.update()
m.write(str(OUT / "quad_obj_and_constr.mps"))

## 8. Objective with constant offset (minimize and maximize)

# maximize with offset
m = grb.Model("maximize_with_offset")
a = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0, ub=20, name="a")
b = m.addVar(vtype=grb.GRB.INTEGER, lb=1, ub=10, name="b")
m.setObjective(5 * a - 3 * b + 42, grb.GRB.MAXIMIZE)
m.addConstr(a + b <= 15, name="c1")
m.addConstr(a >= 2, name="c2")
m.update()
m.write(str(OUT / "maximize_with_offset.mps"))

# minimize with offset
m = grb.Model("minimize_with_offset")
x = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0, ub=10, name="x")
y = m.addVar(vtype=grb.GRB.CONTINUOUS, lb=0, ub=10, name="y")
m.setObjective(x + 2 * y - 7.5, grb.GRB.MINIMIZE)
m.addConstr(x + y >= 3, name="c1")
m.addConstr(x <= 8, name="c2")
m.update()
m.write(str(OUT / "minimize_with_offset.mps"))

