import pytest

from luna_model import Model, Sense, Variable, Vtype
from luna_model.translator import LpTranslator
from luna_model.utils import quicksum


@pytest.fixture
def items():
    items = ["a", "b", "c", "d", "e"]
    return items


@pytest.fixture
def values():
    values = [10, 15, 7, 8, 12]  # Value of each item
    return values


@pytest.fixture
def weights():
    weights = [5, 7, 3, 2, 4]  # Weight of each item
    return weights


@pytest.fixture
def capacity():
    capacity = 10  # Knapsack capacity
    return capacity


def test_set_sense_init(items, values, weights, capacity):
    # Create the binary Variables (selected/not selected item)
    model = Model("Vacation Knapsack1", sense=Sense.Max)

    with model.environment:
        x = {idx: Variable(name=i, vtype=Vtype.Binary) for idx, i in enumerate(items)}

    model.set_objective(quicksum(x[i] * values[i] for i in x.keys()))
    # The total weight of selected items should not exceed a specified threshold
    model.add_constraint(quicksum(x[i] * weights[i] for i in x.keys()) <= capacity)

    assert model.sense == Sense.Max
    lp_str = LpTranslator.from_aq(model)
    assert "Maximize" in lp_str


def test_set_sense_init_lp(items, values, weights, capacity):
    # Create the binary Variables (selected/not selected item)
    model = Model("Vacation Knapsack1", sense=Sense.Max)

    with model.environment:
        x = {idx: Variable(name=i, vtype=Vtype.Binary) for idx, i in enumerate(items)}

    model.set_objective(quicksum(x[i] * values[i] for i in x.keys()))
    # The total weight of selected items should not exceed a specified threshold
    model.add_constraint(quicksum(x[i] * weights[i] for i in x.keys()) <= capacity)

    assert model.sense == Sense.Max
    lp_str = LpTranslator.from_aq(model)
    assert "Maximize" in lp_str


def test_set_sense_after_creation(items, values, weights, capacity):
    # Create the binary Variables (selected/not selected item)
    model = Model("Vacation Knapsack1", sense=Sense.Max)
    # Maximize the total value of selected items
    model.set_sense(Sense.Max)

    with model.environment:
        x = {idx: Variable(name=i, vtype=Vtype.Binary) for idx, i in enumerate(items)}

    model.set_objective(quicksum(x[i] * values[i] for i in x.keys()))
    # The total weight of selected items should not exceed a specified threshold
    model.add_constraint(quicksum(x[i] * weights[i] for i in x.keys()) <= capacity)

    assert model.sense == Sense.Max
    lp_str = LpTranslator.from_aq(model)
    assert "Maximize" in lp_str


def test_set_sense_after_objective(items, values, weights, capacity):
    # Create the binary Variables (selected/not selected item)
    model = Model("Vacation Knapsack1", sense=Sense.Max)

    with model.environment:
        x = {idx: Variable(name=i, vtype=Vtype.Binary) for idx, i in enumerate(items)}

    model.set_objective(quicksum(x[i] * values[i] for i in x.keys()))
    # Maximize the total value of selected items
    model.set_sense(Sense.Max)
    # The total weight of selected items should not exceed a specified threshold
    model.add_constraint(quicksum(x[i] * weights[i] for i in x.keys()) <= capacity)

    assert model.sense == Sense.Max
    lp_str = LpTranslator.from_aq(model)
    assert "Maximize" in lp_str


def test_set_sense_after_constraints(items, values, weights, capacity):
    # Create the binary Variables (selected/not selected item)
    model = Model("Vacation Knapsack1", sense=Sense.Max)

    with model.environment:
        x = {idx: Variable(name=i, vtype=Vtype.Binary) for idx, i in enumerate(items)}

    model.set_objective(quicksum(x[i] * values[i] for i in x.keys()))
    # The total weight of selected items should not exceed a specified threshold
    model.add_constraint(quicksum(x[i] * weights[i] for i in x.keys()) <= capacity)
    # Maximize the total value of selected items
    model.set_sense(Sense.Max)

    assert model.sense == Sense.Max
    lp_str = LpTranslator.from_aq(model)
    assert "Maximize" in lp_str


if __name__ == "__main__":
    items = ["a", "b", "c", "d", "e"]  # type: ignore
    values = [10, 15, 7, 8, 12]  # Value of each item# type: ignore
    weights = [5, 7, 3, 2, 4]  # Weight of each item# type: ignore
    capacity = 10  # Knapsack capacity# type: ignore
    test_set_sense_init(items, values, weights, capacity)
