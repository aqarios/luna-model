import aqmodels as aqm
import numpy as np


with aqm.Environment():
    some = Var().__add__
    aqm.Variable().__add__()
    x = aqm.Variable("x")
    y = aqm.Variable("y")

    aqm.Variable

    res: aqm.Expression = x + y
    res2 = x + complex(2.0, 3.0)
    res3 = x + 1.0
    res4 = 1 + x

    x.__add__

    aqm.Vtype.Integer

    constraints = aqm.Constraints().add_constraint()

    #
    #
    #
    #
    #
    #
    #
    #
    #
    #
    #
    #
    #
    #
    #
    #
    #

    # AQMODELS
    aqm.Constraints().add_constraint

    # NUMPY
    np.dot
