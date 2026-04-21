from luna_model import Model
import numpy as np

def test_np_sum_set_obj():
    A = np.random.randn(10, 10)
    b = np.random.randn(10)

    model = Model()
    x = model.add_variables("x", 10)
    new_obj = ((A @ x - b)).sum()
    model.objective = new_obj
    assert model.objective.equal_contents(new_obj)

def test_np_sum_iadd_obj():
    A = np.random.randn(10, 10)
    b = np.random.randn(10)

    model = Model()
    x = model.add_variables("x", 10)
    new_obj = ((A @ x - b)).sum()
    model.objective += new_obj
    assert model.objective.equal_contents(new_obj)

def test_np_sum_set_obj_direct():
    A = np.random.randn(10, 10)
    b = np.random.randn(10)

    model = Model()
    x = model.add_variables("x", 10)
    new_obj = ((A @ x - b)).sum()
    model.set_objective(new_obj)
    assert model.objective.equal_contents(new_obj)
