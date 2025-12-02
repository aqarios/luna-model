from luna_model import Bounds, Model, Unbounded, Variable, Vtype


def test_hash_consistent_empty():
    model = Model()
    assert hash(model) == -6764926841706024324


def test_hash_consistent_single_binary_var():
    model = Model()
    with model.environment:
        _ = Variable("binary_var", vtype=Vtype.Binary)
    assert hash(model) == -95788649977900538


def test_hash_consistent_single_spin_var():
    model = Model()
    with model.environment:
        _ = Variable("spin_var", vtype=Vtype.Spin)
    assert hash(model) == 6343577891125936620


def test_hash_consistent_single_int_var():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.Integer)
    assert hash(model) == -8958269881730674195


def test_hash_consistent_single_int_var_lower_bounded():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.Integer, bounds=Bounds(lower=10.0))
    assert hash(model) == 4573959890164509998


def test_hash_consistent_single_int_var_upper_bounded():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.Integer, bounds=Bounds(upper=10.0))
    assert hash(model) == -4228195939864877612


def test_hash_consistent_single_int_var_lower_unbounded():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.Integer, bounds=Bounds(lower=Unbounded))
    assert hash(model) == 5838737170991716031


def test_hash_consistent_single_int_var_upper_unbounded():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.Integer, bounds=Bounds(upper=Unbounded))
    assert hash(model) == -8958269881730674195


def test_hash_consistent_single_int_var_lower_and_upper_bounded():
    model = Model()
    with model.environment:
        _ = Variable(
            "int_var", vtype=Vtype.Integer, bounds=Bounds(lower=-10.0, upper=10.0)
        )
    assert hash(model) == -4436694255535605608


def test_hash_consistent_single_int_var_lower_and_upper_unbounded():
    model = Model()
    with model.environment:
        _ = Variable(
            "int_var",
            vtype=Vtype.Integer,
            bounds=Bounds(lower=Unbounded, upper=Unbounded),
        )
    assert hash(model) == 5838737170991716031


def test_hash_consistent_single_real_var():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.Real)
    assert hash(model) == 7742942394472570092


def test_hash_consistent_single_real_var_lower_bounded():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.Real, bounds=Bounds(lower=10.0))
    assert hash(model) == 6841056513532777972


def test_hash_consistent_single_real_var_upper_bounded():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.Real, bounds=Bounds(upper=10.0))
    assert hash(model) == 8450503522538091119


def test_hash_consistent_single_real_var_lower_unbounded():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.Real, bounds=Bounds(lower=Unbounded))
    assert hash(model) == -3447754871946809282


def test_hash_consistent_single_real_var_upper_unbounded():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.Real, bounds=Bounds(upper=Unbounded))
    assert hash(model) == 7742942394472570092


def test_hash_consistent_single_real_var_lower_and_upper_bounded():
    model = Model()
    with model.environment:
        _ = Variable(
            "real_var", vtype=Vtype.Real, bounds=Bounds(lower=-10.0, upper=10.0)
        )
    assert hash(model) == 9015731630494461160


def test_hash_consistent_single_real_var_lower_and_upper_unbounded():
    model = Model()
    with model.environment:
        _ = Variable(
            "real_var",
            vtype=Vtype.Real,
            bounds=Bounds(lower=Unbounded, upper=Unbounded),
        )
    assert hash(model) == -3447754871946809282


def test_hash_consistent_all_vars():
    model = Model()
    with model.environment:
        _ = Variable("binary_var", vtype=Vtype.Binary)
        _ = Variable("spin_var", vtype=Vtype.Spin)
        _ = Variable("int_var", vtype=Vtype.Integer)
        _ = Variable("real_var", vtype=Vtype.Real)
    assert hash(model) == -3164785540861143317


def test_hash_consistent_all_vars_2():
    model = Model()
    with model.environment:
        _ = Variable("binary_var", vtype=Vtype.Binary)
        _ = Variable("spin_var", vtype=Vtype.Spin)
        _ = Variable("int_var1", vtype=Vtype.Integer)
        _ = Variable("int_var2", vtype=Vtype.Integer, bounds=Bounds(lower=10.0))
        _ = Variable("int_var3", vtype=Vtype.Integer, bounds=Bounds(upper=10.0))
        _ = Variable("int_var4", vtype=Vtype.Integer, bounds=Bounds(lower=Unbounded))
        _ = Variable("int_var5", vtype=Vtype.Integer, bounds=Bounds(upper=Unbounded))
        _ = Variable(
            "int_var6", vtype=Vtype.Integer, bounds=Bounds(lower=-10.0, upper=10.0)
        )
        _ = Variable(
            "int_var7",
            vtype=Vtype.Integer,
            bounds=Bounds(lower=Unbounded, upper=Unbounded),
        )
        _ = Variable("real_var1", vtype=Vtype.Real)
        _ = Variable("real_var2", vtype=Vtype.Real, bounds=Bounds(lower=10.0))
        _ = Variable("real_var3", vtype=Vtype.Real, bounds=Bounds(upper=10.0))
        _ = Variable("real_var4", vtype=Vtype.Real, bounds=Bounds(lower=Unbounded))
        _ = Variable("real_var5", vtype=Vtype.Real, bounds=Bounds(upper=Unbounded))
        _ = Variable(
            "real_var6", vtype=Vtype.Real, bounds=Bounds(lower=-10.0, upper=10.0)
        )
        _ = Variable(
            "real_var7",
            vtype=Vtype.Real,
            bounds=Bounds(lower=Unbounded, upper=Unbounded),
        )
    assert hash(model) == 4557796912099728127


def test_hash_consistent_objective_offset():
    model = Model()
    model.objective += 1
    assert hash(model) == 3483826857191946563


def test_hash_consistent_objective_linear():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b + s + i + r
    assert hash(model) == 6363289082320142358


def test_hash_consistent_objective_linear_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s + i * r + 2
    assert hash(model) == 5608975169706928584


def test_hash_consistent_objective_quadratic():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s + i * r
    assert hash(model) == 433363269166188437


def test_hash_consistent_objective_quadratic_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s + i * r + 0.3
    assert hash(model) == 4544268381992336264


def test_hash_consistent_objective_quadratic_and_linear():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b + s + b * s + i * r
    assert hash(model) == 5364119472507982273


def test_hash_consistent_objective_quadratic_and_linear_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b + s + b * s + i * r + 5
    assert hash(model) == -32155725510190987


def test_hash_consistent_objective_higher_order():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s * i * r
    assert hash(model) == -4191112908358922148


def test_hash_consistent_objective_higher_order_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s * i * r + b + r + 3
    assert hash(model) == 8249320863959553314


def test_hash_consistent_objective_higher_order_and_linear():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s * i * r + b + r
    assert hash(model) == 8308547375585881025


def test_hash_consistent_objective_higher_order_and_linear_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s * i * r + b + r - 2.2
    assert hash(model) == -2744868664427448763


def test_hash_consistent_objective_higher_order_and_quadratic():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s * i * r + b * r
    assert hash(model) == -5780463137948665038


def test_hash_consistent_objective_higher_order_and_quadratic_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s * i * r + b * r + 2.2
    assert hash(model) == -8850221839304271589


def test_hash_consistent_objective_higher_order_and_linear_and_quadratic_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s * i * r + b * r + 2.2 + s
    assert hash(model) == 6909893265384172710


def test_hash_consistent_full_old():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s * i * r + b * r + 2.2 + s
    model.constraints += b * s <= 3, "constraint named"
    model.constraints += s * i >= 2
    model.constraints += b - r + i == 2
    assert hash(model) != -1167056374483366947


def test_hash_consistent_full():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.Binary)
        s = Variable("spin_var", vtype=Vtype.Spin)
        i = Variable("int_var", vtype=Vtype.Integer)
        r = Variable("real_var", vtype=Vtype.Real)
    model.objective += b * s * i * r + b * r + 2.2 + s
    model.constraints += b * s <= 3, "constraint named"
    model.constraints += s * i >= 2
    model.constraints += b - r + i == 2
    assert hash(model) == -6467512093433223330
