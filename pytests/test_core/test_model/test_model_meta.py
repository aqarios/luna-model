import random

import pytest

from aqmodels import (Comparator, Constant, Constraint, HigherOrder, Linear,
                      Model, Quadratic, Vtype, quicksum)


@pytest.fixture
def model(request):
    print(request)
    req = request.param

    vtypes: list[Vtype] = req.get("vtypes", None)
    ctypes: list[Comparator] = req.get("ctypes", [])
    target_degree = req.get("target_degree", 0)

    if vtypes is None:
        raise RuntimeError("vtypes cannot be none")

    map = {str(vt): 0 for vt in vtypes}

    def choose_random_vtype():
        if sum(map.values()) == 0:
            return random.choice(vtypes)
        else:
            item = random.choice(vtypes)
            if map[str(item)] != 0:
                return choose_random_vtype()
            else:
                map[str(item)] += 1
                return item

    model = Model()
    vars = [
        model.add_variable(f"x_{i}", vtype=choose_random_vtype())
        for i in range(max(target_degree, len(vtypes)) + 2)
    ]

    for deg in range(target_degree):
        model.objective += quicksum(
            [random.random() * var for var in random.sample(vars, deg + 1)]
        )

    for ct in ctypes:
        model.constraints.add_constraint(
            Constraint(
                quicksum(
                    [
                        random.random() * var
                        for var in random.sample(vars, random.randint(2, len(vars)))
                    ]
                ),
                random.randint(0, 100),
                ct,
            )
        )
    return model


def naive_query_ctypes(model: Model) -> list[Comparator]:
    contained: list[str] = []
    ctypes: list[Comparator] = []
    for c in model.constraints:
        cc = c.comparator
        if str(cc) not in contained:
            contained.append(str(cc))
            ctypes.append(cc)
    return ctypes


def naive_query_vtypes(model: Model) -> list[Vtype]:
    contained: list[str] = []
    vtypes: list[Vtype] = []
    for v in model.objective.variables():
        vt = v.vtype
        if str(vt) not in contained:
            contained.append(str(vt))
            vtypes.append(vt)
    for c in model.constraints:
        for v in c.lhs.variables():
            vt = v.vtype
            if str(vt) not in contained:
                contained.append(str(vt))
                vtypes.append(vt)
    return vtypes


def naive_query_obj_degree(model: Model) -> int:
    degree: int = 0
    for vars, _ in model.objective.items():
        match vars:
            case Constant():
                pass
            case Linear(_):
                degree = 1 if degree <= 1 else degree
            case Quadratic(_, _):
                degree = 2 if degree <= 2 else degree
            case HigherOrder(ho):
                degree = len(ho) if degree <= len(ho) else degree
    return degree


@pytest.mark.parametrize(
    "model",
    [
        {"vtypes": [Vtype.Binary]},
        {"vtypes": [Vtype.Spin], "ctypes": [Comparator.Eq]},
        {
            "vtypes": [Vtype.Spin, Vtype.Binary, Vtype.Integer, Vtype.Real],
            "ctypes": [Comparator.Eq, Comparator.Le, Comparator.Ge],
            "target_degree": 1,
        },
        {
            "vtypes": [Vtype.Spin, Vtype.Binary, Vtype.Integer, Vtype.Real],
            "ctypes": [Comparator.Eq, Comparator.Le, Comparator.Ge],
            "target_degree": 2
        },
        {
            "vtypes": [Vtype.Spin, Vtype.Binary, Vtype.Integer, Vtype.Real],
            "ctypes": [Comparator.Eq, Comparator.Le, Comparator.Ge],
            "target_degree": 3
        },
        {
            "vtypes": [Vtype.Spin, Vtype.Binary, Vtype.Integer, Vtype.Real],
            "ctypes": [Comparator.Eq, Comparator.Le, Comparator.Ge],
            "target_degree": 10
        },
        {
            "vtypes": [Vtype.Integer],
            "target_degree": 0
        },
        {
            "vtypes": [Vtype.Integer],
            "target_degree": 2
        },
        {
            "vtypes": [Vtype.Real],
            "target_degree": 1
        },
        {
            "vtypes": [Vtype.Spin, Vtype.Binary],
            "ctypes": [Comparator.Eq, Comparator.Le, Comparator.Ge],
            "target_degree": 4
        },
        {
            "vtypes": [Vtype.Real, Vtype.Binary],
            "ctypes": [Comparator.Eq, Comparator.Ge],
            "target_degree": 1
        },
    ],
    indirect=True,
)
def test_model_query(model: Model):
    assert eqlists(naive_query_ctypes(model), model.constraints.ctypes())
    assert eqlists(naive_query_vtypes(model), model.vtypes())
    assert naive_query_obj_degree(model) == model.objective.degree()


def eqlists(a: list, b: list) -> bool:
    for ae in a:
        if ae not in b:
            return False
    return True
