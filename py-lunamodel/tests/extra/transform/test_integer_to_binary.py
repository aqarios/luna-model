from luna_model import Model, Vtype
from luna_model.transformation import PassManager, passes

def test_upper2():
    model = Model("model")
    x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=2)
    model.objective = x

    pm = PassManager([passes.IntegerToBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 2 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 1 * variables[0] + 1 * variables[1]
    assert expected_expr.equal_contents(ir.model.objective)

def test_upper3():
    model = Model("model")
    x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=3)
    model.objective = x

    pm = PassManager([passes.IntegerToBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 2 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 1 * variables[0] + 2 * variables[1]
    assert expected_expr.equal_contents(ir.model.objective)

def test_upper12():
    model = Model("model")
    x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=12)
    model.objective = x

    pm = PassManager([passes.IntegerToBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 4 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 1 * variables[0] + 2 * variables[1] + 4 * variables[2] + 5 * variables[3]
    assert expected_expr.equal_contents(ir.model.objective)

def test_upper20():
    model = Model("model")
    x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=20)
    model.objective = x

    pm = PassManager([passes.IntegerToBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 5 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 1 * variables[0] + 2 * variables[1] + 4 * variables[2] + 8 * variables[3] + 5 * variables[4]
    assert expected_expr.equal_contents(ir.model.objective)
