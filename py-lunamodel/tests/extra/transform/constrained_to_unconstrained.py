from luna_model import Model
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
    
    penalty_factor = 10
    pm = PassManager([pipelines.ConstrainedToUnconstrainedPipeline(penalty_factor=penalty_factor)])
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
    c0_penalty = (x + y + z + slack_c0_b0 + 2 * slack_c0_b1)**2
    c1_penalty = (-1 * (x - y - z) + slack_c1_b0)**2
    c2_penalty = (x + y - 2)**2
    expected_objective = obj + penalty_factor * (c0_penalty + c1_penalty + c2_penalty)
    assert expected_objective.equal_contents(ir.model.objective)
