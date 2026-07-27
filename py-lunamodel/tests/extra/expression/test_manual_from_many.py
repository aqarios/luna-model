from luna_model import Model, Expression


def test_expr_manual_from_many():
    model = Model()
    vs = model.add_variables("x", 3)
    expr = vs[0] + vs[1] * vs[2] + vs[2]
    assert isinstance(expr, Expression)
