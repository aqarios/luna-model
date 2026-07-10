import pytest

from luna_model import Model, Sense, Vtype
from luna_model.errors import AnalysisPassError, TransformError
from luna_model.transformation import PassManager, pipelines

def test_very_simple_model():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")
    z = model.add_variable("z")
    
    model.objective = x + y + z
    
    model.constraints += x + y + z <= 3
    model.constraints += x - y - z >= 0
    model.constraints += x + y == 2
    
    penalty_scaling = 13
    pm = PassManager([pipelines.ToUnconstrainedBinaryPipeline(penalty_scaling=penalty_scaling)])
    ir = pm.run(model)

    assert 6 == len(ir.model.variables())
    assert 0 == len(ir.model.constraints)

    x, y, z, slack_c1_b0, slack_c0_b0, slack_c0_b1 = (
        ir.model.get_variable("x"),
        ir.model.get_variable("y"),
        ir.model.get_variable("z"),
        ir.model.get_variable("slack_c1_b0"),
        ir.model.get_variable("slack_c0_b0"),
        ir.model.get_variable("slack_c0_b1"),
    )

    obj = x + y + z
    c0_penalty = (x + y + z - slack_c0_b0 - 2 * slack_c0_b1)**2
    c1_penalty = (-1 * (x - y - z) - slack_c1_b0)**2
    c2_penalty = (x + y - 2)**2
    expected_objective = obj + penalty_scaling * (c0_penalty + c1_penalty + c2_penalty)
    assert expected_objective.equal_contents(ir.model.objective)

def test_very_simple_model_max_sense():
    model = Model(sense=Sense.MAX)
    x = model.add_variable("x")
    y = model.add_variable("y")
    z = model.add_variable("z")
    
    model.objective = x + y + z
    
    model.constraints += x + y + z <= 3
    model.constraints += x - y - z >= 0
    model.constraints += x + y == 2
    
    penalty_scaling = 1221
    pm = PassManager([pipelines.ToUnconstrainedBinaryPipeline(penalty_scaling=penalty_scaling)])
    ir = pm.run(model)

    assert 6 == len(ir.model.variables())
    assert 0 == len(ir.model.constraints)

    x, y, z, slack_c1_b0, slack_c0_b0, slack_c0_b1 = (
        ir.model.get_variable("x"),
        ir.model.get_variable("y"),
        ir.model.get_variable("z"),
        ir.model.get_variable("slack_c1_b0"),
        ir.model.get_variable("slack_c0_b0"),
        ir.model.get_variable("slack_c0_b1"),
    )

    obj = x + y + z
    c0_penalty = (x + y + z - slack_c0_b0 - 2 * slack_c0_b1)**2
    c1_penalty = (-1 * (x - y - z) - slack_c1_b0)**2
    c2_penalty = (x + y - 2)**2
    expected_objective = -obj + penalty_scaling * (c0_penalty + c1_penalty + c2_penalty)
    assert expected_objective.equal_contents(ir.model.objective)

def test_very_simple_model_intvars():
    model = Model()
    x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=1)
    y = model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=2)
    z = model.add_variable("z", vtype=Vtype.INTEGER, lower=0, upper=12)
    
    model.objective = x + y + z
    
    model.constraints += x + y + z <= 3
    model.constraints += x - y - z >= 0
    model.constraints += x + y == 2
    
    penalty_scaling = 2
    pm = PassManager([pipelines.ToUnconstrainedBinaryPipeline(penalty_scaling=penalty_scaling)])
    ir = pm.run(model)

    assert 10 == len(ir.model.variables())
    assert 0 == len(ir.model.constraints)

    x, y_b0, y_b1, z_b0, z_b1, z_b2, z_b3, slack_c1_b0, slack_c0_b0, slack_c0_b1 = (
        ir.model.get_variable("x_b0"),
        ir.model.get_variable("y_b0"),
        ir.model.get_variable("y_b1"),
        ir.model.get_variable("z_b0"),
        ir.model.get_variable("z_b1"),
        ir.model.get_variable("z_b2"),
        ir.model.get_variable("z_b3"),
        ir.model.get_variable("slack_c1_b0"),
        ir.model.get_variable("slack_c0_b0"),
        ir.model.get_variable("slack_c0_b1"),
    )
    y = y_b0 + y_b1
    z = 1 * z_b0 + 2 * z_b1 + 4 * z_b2 + 5 * z_b3

    obj = x + y + z
    max_bias = 5

    c0_penalty = (x + y + z - slack_c0_b0 - 2 * slack_c0_b1)**2
    c1_penalty = (-1 * (x - y - z) - slack_c1_b0)**2
    c2_penalty = (x + y - 2)**2

    expected_objective = obj + penalty_scaling * max_bias * (c0_penalty + c1_penalty + c2_penalty)
    assert expected_objective.equal_contents(ir.model.objective)


def test_very_simple_model_mixed():
    model = Model()
    x = model.add_variable("x", vtype=Vtype.BINARY)
    y = model.add_variable("y", vtype=Vtype.SPIN)
    z = model.add_variable("z", vtype=Vtype.INTEGER, lower=0, upper=12)
        
    model.objective = x + y + z
    
    model.constraints += x + y + z <= 3, "c0"
    model.constraints += x - y - z >= 0, "c1"
    model.constraints += x + y == 2, "c2"
    
    penalty_scaling = 10
    pm = PassManager([pipelines.ToUnconstrainedBinaryPipeline(penalty_scaling=penalty_scaling)])
    ir = pm.run(model)

    x = ir.model.get_variable("x")
    b_y = ir.model.get_variable("b_y")
    z_b0 = ir.model.get_variable("z_b0")
    z_b1 = ir.model.get_variable("z_b1")
    z_b2 = ir.model.get_variable("z_b2")
    z_b3 = ir.model.get_variable("z_b3")
    slack_c0_b0 = ir.model.get_variable("slack_c0_b0")
    slack_c0_b1 = ir.model.get_variable("slack_c0_b1")
    slack_c0_b2 = ir.model.get_variable("slack_c0_b2")
    slack_c1_b0 = ir.model.get_variable("slack_c1_b0")
    slack_c1_b1 = ir.model.get_variable("slack_c1_b1")

    assert 11 == len(ir.model.variables())
    assert 0 == len(ir.model.constraints)

    y = -2 * b_y + 1
    z = 1 * z_b0 + 2 * z_b1 + 4 * z_b2 + 5 * z_b3

    obj = x + y + z
    max_bias = max([b for _, b in obj.items()])

    c0_penalty = (x + y + z - slack_c0_b0 - 2 * slack_c0_b1 - 2 * slack_c0_b2)**2
    c1_penalty = (-1 * (x - y - z) - slack_c1_b0 - 2 * slack_c1_b1)**2
    c2_penalty = (x + y - 2)**2
    penalties = c0_penalty + c1_penalty + c2_penalty

    expected_objective = obj + penalty_scaling * max_bias * penalties
    assert expected_objective.equal_contents(ir.model.objective)


def test_illegal_model():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")
    z = model.add_variable("z")
    
    model.objective = x + y + z
    
    model.constraints += x + y + z <= 3
    model.constraints += x - y - z >= 0
    model.constraints += x * y + y == 2
    
    penalty_scaling = 13
    pm = PassManager([pipelines.ToUnconstrainedBinaryPipeline(penalty_scaling=penalty_scaling)])
    with pytest.raises(TransformError):
        _ = pm.run(model)
