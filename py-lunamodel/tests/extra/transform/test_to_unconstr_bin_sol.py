from luna_model import Model, Vtype, Solution
from luna_model.transformation import PassManager, pipelines

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
    output = pm.run(model)


    unconstr_bin_sol = Solution.from_dicts(
            [
        {
            "x": 0,
            "b_y": 0,
            "z_b0": 0,
            "z_b1": 0,
            "z_b2": 0,
            "z_b3": 0,
            "slack_c0_b0": 0,
            "slack_c0_b1": 0,
            "slack_c0_b2": 0,
            "slack_c1_b0": 0,
            "slack_c1_b1": 0
        },
        {
            "x": 0,
            "b_y": 0,
            "z_b0": 1,
            "z_b1": 1,
            "z_b2": 1,
            "z_b3": 1,
            "slack_c0_b0": 0,
            "slack_c0_b1": 0,
            "slack_c0_b2": 0,
            "slack_c1_b0": 0,
            "slack_c1_b1": 0
        },
        {
            "x": 1,
            "b_y": 0,
            "z_b0": 1,
            "z_b1": 0,
            "z_b2": 0,
            "z_b3": 0,
            "slack_c0_b0": 1,
            "slack_c0_b1": 1,
            "slack_c0_b2": 1,
            "slack_c1_b0": 1,
            "slack_c1_b1": 1
        },
        ], env=output.model.environment)

    sol = model.evaluate(output.record.backward(unconstr_bin_sol))
    assert sol.obj_values is not None
    assert [1.0, 13.0, 3.0] == sol.obj_values.tolist()
    assert [[0, 0, 1], [0, 12, 1], [1, 1, 1]] == sol.samples.tolist()
