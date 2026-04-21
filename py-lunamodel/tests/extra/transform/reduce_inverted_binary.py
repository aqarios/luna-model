from luna_model import Model, Vtype
from luna_model.transformation import PassManager, passes

def test_invbin_a():
    model = Model("model")
    x = model.add_variable("x", vtype=Vtype.BINARY)
    model.objective = 2 * x + ~x

    pm = PassManager([passes.ReduceInvertedBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 1 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 2 * x + (1 - x)
    assert expected_expr.equal_contents(ir.model.objective)

def test_invbin_b():
    model = Model("model_lin")
    x = model.add_variable("x", vtype=Vtype.BINARY)
    y = model.add_variable("y", vtype=Vtype.BINARY)
    z = model.add_variable("z", vtype=Vtype.BINARY)
    
    model.objective = 4 * x - 3 * ~y + 2 * z + 5
    model.constraints += x + ~y + z <= 2, "c0"
    model.constraints += 2 * x - y + ~z >= 0, "c1"
    model.constraints += x + y + z == 1, "c2"

    pm = PassManager([passes.ReduceInvertedBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 3 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 4 * x - 3 * (1 - y) + 2 * z + 5
    expected_c0 = x + (1 - y) + z
    expected_c1 = 2 * x - y + (1 - z)
    expected_c2 = x + y + z

    assert expected_expr.equal_contents(ir.model.objective)
    assert expected_c0.equal_contents(ir.model.constraints["c0"].lhs)
    assert expected_c1.equal_contents(ir.model.constraints["c1"].lhs)
    assert expected_c2.equal_contents(ir.model.constraints["c2"].lhs)

def test_invbin_c():
    model = Model("model_quad")
    a = model.add_variable("a", vtype=Vtype.BINARY)
    b = model.add_variable("b", vtype=Vtype.BINARY)
    c = model.add_variable("c", vtype=Vtype.BINARY)
    
    model.objective = 3 * a * ~b + 2 * ~a * c + 5 * b * c - ~c
    model.constraints += a * ~b + b * c <= 1, "c0"
    model.constraints += ~a * c + a * b >= 0, "c1"
    model.constraints += a + b + ~c == 2, "c2"

    pm = PassManager([passes.ReduceInvertedBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 3 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 3 * a * (1 - b) + 2 * (1 - a) * c + 5 * b * c - (1 - c)
    expected_c0 = a * (1 - b) + b * c
    expected_c1 = (1 - a) * c + a * b
    expected_c2 = a + b + (1 - c)

    assert expected_expr.equal_contents(ir.model.objective)
    assert expected_c0.equal_contents(ir.model.constraints["c0"].lhs)
    assert expected_c1.equal_contents(ir.model.constraints["c1"].lhs)
    assert expected_c2.equal_contents(ir.model.constraints["c2"].lhs)
 
def test_invbin_d():
    model = Model("model_cubic")
    x1 = model.add_variable("x1", vtype=Vtype.BINARY)
    x2 = model.add_variable("x2", vtype=Vtype.BINARY)
    x3 = model.add_variable("x3", vtype=Vtype.BINARY)
    x4 = model.add_variable("x4", vtype=Vtype.BINARY)
    
    model.objective = 6 * x1 * ~x2 * x3 - 2 * ~x1 * x2 * ~x4 + 3 * x1 * x4 + ~x3
    model.constraints += x1 * ~x2 * x3 + x2 * x4 <= 1, "c0"
    model.constraints += ~x1 * x2 * ~x3 + x4 >= 0, "c1"
    model.constraints += x1 + ~x2 + x3 + ~x4 == 2, "c2"

    pm = PassManager([passes.ReduceInvertedBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 3 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 6 * x1 * (1 - x2) * x3 - 2 * (1 - x1) * x2 * (1 - x4) + 3 * x1 * x4 + (1 - x3)
    expected_c0 = x1 * (1 - x2) * x3 + x2 * x4
    expected_c1 = (1 - x1) * x2 * (1 - x3) + x4
    expected_c2 = x1 + (1 - x2) + x3 + (1 - x4)

    assert expected_expr.equal_contents(ir.model.objective)
    assert expected_c0.equal_contents(ir.model.constraints["c0"].lhs)
    assert expected_c1.equal_contents(ir.model.constraints["c1"].lhs)
    assert expected_c2.equal_contents(ir.model.constraints["c2"].lhs)

def test_invbin_e():
    model = Model("model_quartic")
    p = model.add_variable("p", vtype=Vtype.BINARY)
    q = model.add_variable("q", vtype=Vtype.BINARY)
    r = model.add_variable("r", vtype=Vtype.BINARY)
    s = model.add_variable("s", vtype=Vtype.BINARY)
    
    model.objective =    2 * p * ~q * r * ~s + 4 * p * q - 3 * ~r * s + 1
    model.constraints += p * ~q * r + ~p * q * s <= 1, "c0"
    model.constraints += p * q + ~r * ~s >= 0, "c1"
    model.constraints += ~p + q + r + s == 2, "c2"

    pm = PassManager([passes.ReduceInvertedBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 3 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 2 * p * (1 - q) * r * (1 - s) + 4 * p * q - 3 * (1 - r) * s + 1
    expected_c0 = p * (1 - q) * r + (1 - p) * q * s
    expected_c1 = p * q + (1 - r) * (1 - s)
    expected_c2 = (1 - p) + q + r + s

    assert expected_expr.equal_contents(ir.model.objective)
    assert expected_c0.equal_contents(ir.model.constraints["c0"].lhs)
    assert expected_c1.equal_contents(ir.model.constraints["c1"].lhs)
    assert expected_c2.equal_contents(ir.model.constraints["c2"].lhs)

def test_invbin_f():
    model = Model("model_mixed")
    u = model.add_variable("u", vtype=Vtype.BINARY)
    v = model.add_variable("v", vtype=Vtype.BINARY)
    w = model.add_variable("w", vtype=Vtype.BINARY)
    t = model.add_variable("t", vtype=Vtype.BINARY)
    
    model.objective =    7 * u - 2 * ~v + 3 * u * w - 5 * ~u * v * t + 2 * ~w + 9
    model.constraints += u + v + ~w + t <= 3, "c0"
    model.constraints += u * ~v + ~u * w + v * t >= 1, "c1"
    model.constraints += ~u * v * ~w + t == 1, "c2"

    pm = PassManager([passes.ReduceInvertedBinaryPass()])
    ir = pm.run(model)
    variables = ir.model.variables()
    assert 3 == len(variables)
    assert [Vtype.BINARY] == ir.model.get_specs().vtypes

    expected_expr = 7 * u - 2 * (1 - v) + 3 * u * w - 5 * (1 - u) * v * t + 2 * (1 - w) + 9
    expected_c0 = u + v + (1 - w) + t
    expected_c1 = u * (1 - v) + (1 - u) * w + v * t
    expected_c2 = (1 - u) * v * (1 - w) + t

    assert expected_expr.equal_contents(ir.model.objective)
    assert expected_c0.equal_contents(ir.model.constraints["c0"].lhs)
    assert expected_c1.equal_contents(ir.model.constraints["c1"].lhs)
    assert expected_c2.equal_contents(ir.model.constraints["c2"].lhs)
