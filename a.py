from dimod import lp
from dimod import ConstrainedQuadraticModel, Binary

failing_labels_write = [
    "0",
    "0name",
]

failing_labels = [
    "nan",
    "Nan",
    "nano",
    "nanoword",
    "inf",
    "Inf",
    "infi",
    "infword",
    "nan",
    "inf",
    "nanometer",
    "infeasiblility",
]

for fl in failing_labels:
    cqm = ConstrainedQuadraticModel()
    x, y = Binary("x"), Binary("y")
    cqm.set_objective(-(x * y))
    cqm.add_constraint(x + y <= 2, label=fl)
    lp_str = lp.dumps(cqm)
    try:
        back = lp.loads(lp_str)
        print(f"ok: {fl}")
    except Exception as e:
        print(f"failed for constraint name '{fl}' with exception: {e}")
