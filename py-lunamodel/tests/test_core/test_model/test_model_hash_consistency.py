from luna_model import Bounds, Model, Unbounded, Variable, Vtype


def test_hash_consistent_empty():
    model = Model()
    assert hash(model) == -5361123191640090060


def test_hash_consistent_single_binary_var():
    model = Model()
    with model.environment:
        _ = Variable("binary_var", vtype=Vtype.BINARY)
    assert hash(model) == -629307757926968461


def test_hash_consistent_single_spin_var():
    model = Model()
    with model.environment:
        _ = Variable("spin_var", vtype=Vtype.SPIN)
    assert hash(model) == -8450188572072835349


def test_hash_consistent_single_int_var():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.INTEGER)
    assert hash(model) == 7283351642322674764


def test_hash_consistent_single_int_var_lower_bounded():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.INTEGER, bounds=Bounds(lower=10.0))
    assert hash(model) == 7283351642337752140


def test_hash_consistent_single_int_var_upper_bounded():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.INTEGER, bounds=Bounds(upper=10.0))
    assert hash(model) == -3771551934397769660


def test_hash_consistent_single_int_var_lower_unbounded():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.INTEGER, bounds=Bounds(lower=Unbounded))
    assert hash(model) == -2713218688521556423


def test_hash_consistent_single_int_var_upper_unbounded():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.INTEGER, bounds=Bounds(upper=Unbounded))
    assert hash(model) == 7283351642322674764


def test_hash_consistent_single_int_var_lower_and_upper_bounded():
    model = Model()
    with model.environment:
        _ = Variable("int_var", vtype=Vtype.INTEGER, bounds=Bounds(lower=-10.0, upper=10.0))
    assert hash(model) == -3771551934404433852


def test_hash_consistent_single_int_var_lower_and_upper_unbounded():
    model = Model()
    with model.environment:
        _ = Variable(
            "int_var",
            vtype=Vtype.INTEGER,
            bounds=Bounds(lower=Unbounded, upper=Unbounded),
        )
    assert hash(model) == -2713218688521556423


def test_hash_consistent_single_real_var():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.REAL)
    assert hash(model) == 4938488210834836303


def test_hash_consistent_single_real_var_lower_bounded():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.REAL, bounds=Bounds(lower=10.0))
    assert hash(model) == 4938488210849913679


def test_hash_consistent_single_real_var_upper_bounded():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.REAL, bounds=Bounds(upper=10.0))
    assert hash(model) == 8239708645170389570


def test_hash_consistent_single_real_var_lower_unbounded():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.REAL, bounds=Bounds(lower=Unbounded))
    assert hash(model) == 4037386132776070660


def test_hash_consistent_single_real_var_upper_unbounded():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.REAL, bounds=Bounds(upper=Unbounded))
    assert hash(model) == 4938488210834836303


def test_hash_consistent_single_real_var_lower_and_upper_bounded():
    model = Model()
    with model.environment:
        _ = Variable("real_var", vtype=Vtype.REAL, bounds=Bounds(lower=-10.0, upper=10.0))
    assert hash(model) == 8239708645230834242


def test_hash_consistent_single_real_var_lower_and_upper_unbounded():
    model = Model()
    with model.environment:
        _ = Variable(
            "real_var",
            vtype=Vtype.REAL,
            bounds=Bounds(lower=Unbounded, upper=Unbounded),
        )
    assert hash(model) == 4037386132776070660


def test_hash_consistent_all_vars():
    model = Model()
    with model.environment:
        _ = Variable("binary_var", vtype=Vtype.BINARY)
        _ = Variable("spin_var", vtype=Vtype.SPIN)
        _ = Variable("int_var", vtype=Vtype.INTEGER)
        _ = Variable("real_var", vtype=Vtype.REAL)
    assert hash(model) == 5499398583564757907


def test_hash_consistent_all_vars_2():
    model = Model()
    with model.environment:
        _ = Variable("binary_var", vtype=Vtype.BINARY)
        _ = Variable("spin_var", vtype=Vtype.SPIN)
        _ = Variable("int_var1", vtype=Vtype.INTEGER)
        _ = Variable("int_var2", vtype=Vtype.INTEGER, bounds=Bounds(lower=10.0))
        _ = Variable("int_var3", vtype=Vtype.INTEGER, bounds=Bounds(upper=10.0))
        _ = Variable("int_var4", vtype=Vtype.INTEGER, bounds=Bounds(lower=Unbounded))
        _ = Variable("int_var5", vtype=Vtype.INTEGER, bounds=Bounds(upper=Unbounded))
        _ = Variable("int_var6", vtype=Vtype.INTEGER, bounds=Bounds(lower=-10.0, upper=10.0))
        _ = Variable(
            "int_var7",
            vtype=Vtype.INTEGER,
            bounds=Bounds(lower=Unbounded, upper=Unbounded),
        )
        _ = Variable("real_var1", vtype=Vtype.REAL)
        _ = Variable("real_var2", vtype=Vtype.REAL, bounds=Bounds(lower=10.0))
        _ = Variable("real_var3", vtype=Vtype.REAL, bounds=Bounds(upper=10.0))
        _ = Variable("real_var4", vtype=Vtype.REAL, bounds=Bounds(lower=Unbounded))
        _ = Variable("real_var5", vtype=Vtype.REAL, bounds=Bounds(upper=Unbounded))
        _ = Variable("real_var6", vtype=Vtype.REAL, bounds=Bounds(lower=-10.0, upper=10.0))
        _ = Variable(
            "real_var7",
            vtype=Vtype.REAL,
            bounds=Bounds(lower=Unbounded, upper=Unbounded),
        )
    assert hash(model) == 3342111566919985180


def test_hash_consistent_objective_offset():
    model = Model()
    model.objective += 1
    assert hash(model) == -5361123191606027724


def test_hash_consistent_objective_linear():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b + s + i + r
    assert hash(model) == 6615781558633943880


def test_hash_consistent_objective_linear_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s + i * r + 2
    assert hash(model) == 5837237755279088318


def test_hash_consistent_objective_quadratic():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s + i * r
    assert hash(model) == 5837237755329419966


def test_hash_consistent_objective_quadratic_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s + i * r + 0.3
    assert hash(model) == -3663462329736437569


def test_hash_consistent_objective_quadratic_and_linear():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b + s + b * s + i * r
    assert hash(model) == -5154785984214303133


def test_hash_consistent_objective_quadratic_and_linear_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b + s + b * s + i * r + 5
    assert hash(model) == -5154785984179810717


def test_hash_consistent_objective_higher_order():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s * i * r
    assert hash(model) == 5657157430976191603


def test_hash_consistent_objective_higher_order_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s * i * r + b + r + 3
    assert hash(model) == -2561585827113835511


def test_hash_consistent_objective_higher_order_and_linear():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s * i * r + b + r
    assert hash(model) == -2561585827070589943


def test_hash_consistent_objective_higher_order_and_linear_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s * i * r + b + r - 2.2
    assert hash(model) == 5999301889912940931


def test_hash_consistent_objective_higher_order_and_quadratic():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s * i * r + b * r
    assert hash(model) == -2035595686005907023


def test_hash_consistent_objective_higher_order_and_quadratic_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s * i * r + b * r + 2.2
    assert hash(model) == 1394189610978789407


def test_hash_consistent_objective_higher_order_and_linear_and_quadratic_and_offset():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s * i * r + b * r + 2.2 + s
    assert hash(model) == 7583378794129815625


def test_hash_consistent_full_old():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s * i * r + b * r + 2.2 + s
    model.constraints += b * s <= 3, "constraint named"
    model.constraints += s * i >= 2
    model.constraints += b - r + i == 2
    assert hash(model) != -1167056374483366947


def test_hash_consistent_full():
    model = Model()
    with model.environment:
        b = Variable("binary_var", vtype=Vtype.BINARY)
        s = Variable("spin_var", vtype=Vtype.SPIN)
        i = Variable("int_var", vtype=Vtype.INTEGER)
        r = Variable("real_var", vtype=Vtype.REAL)
    model.objective += b * s * i * r + b * r + 2.2 + s
    model.constraints += b * s <= 3, "constraint named"
    model.constraints += s * i >= 2
    model.constraints += b - r + i == 2
    assert hash(model) == -2882223103470356936
