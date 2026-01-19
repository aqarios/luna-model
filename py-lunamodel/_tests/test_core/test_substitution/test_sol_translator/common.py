from luna_model import Model, Solution, Vtype, quicksum


def check_solution_contents(lhs: Solution, rhs: Solution) -> bool:
    eq_best = (
        lhs.best() is not None
        and rhs.best() is not None
        and lhs.best()[0].sample.to_dict() == rhs.best()[0].sample.to_dict()  # type: ignore
    )

    return (
        eq_best
        and len(lhs) == len(rhs)
        and lhs.obj_values == rhs.obj_values
        and lhs.counts == rhs.counts
        and lhs.runtime == rhs.runtime
        and lhs.sense == rhs.sense
        and lhs.variable_names == rhs.variable_names
        and lhs.samples.tolist() == rhs.samples.tolist()
        and lhs.expectation_value() == rhs.expectation_value()
        and lhs.feasibility_ratio() == rhs.feasibility_ratio()
    )


def build_truth_solution(model: Model) -> Solution:
    return Solution.from_dict(
        {
            "b1": 0,
            "b2": 1,
            "s1": -1,
            "s2": +1,
            "i1": 4,
            "i2": 3,
        },
        model=model,
    )


def build_truth_model() -> Model:
    truth = Model("testing_model")
    b1 = truth.add_variable("b1")
    b2 = truth.add_variable("b2")
    s1 = truth.add_variable("s1", vtype=Vtype.SPIN)
    s2 = truth.add_variable("s2", vtype=Vtype.SPIN)
    i1 = truth.add_variable("i1", vtype=Vtype.INTEGER)
    i2 = truth.add_variable("i2", vtype=Vtype.INTEGER)

    offset = 10
    linear = quicksum([b1, 2 * b2, 3 * s1, 4 * s2, 5 * i1, 6 * i2])
    quadratic = 2 * b1 * s1 + 3 * b2 * s2 + 4 * i1 * i2
    ho = 6 * b1 * b2 * i1 + 7 * s1 * i1 * i2 + 8 * b2 * i1 * i2 + 9 * s2 * i2 * b2
    truth.objective = linear + quadratic + ho + offset

    return truth


def build_subst_model() -> Model:
    subs = Model("testing_model")
    b1 = subs.add_variable("b1")
    b2 = subs.add_variable("b2")
    s1 = subs.add_variable("s1", vtype=Vtype.SPIN)
    s2 = subs.add_variable("s2", vtype=Vtype.SPIN)
    i1 = subs.add_variable("i1", vtype=Vtype.INTEGER)
    # this will be used as the target for substition
    target = subs.add_variable("target", vtype=Vtype.REAL)

    offset = 10
    linear = quicksum([b1, 2 * b2, 3 * s1, 4 * s2, 5 * i1, 3 * target])
    quadratic = 2 * b1 * s1 + 3 * b2 * s2 + 2 * i1 * target
    ho = (
        6 * b1 * b2 * i1
        + 3.5 * s1 * i1 * target
        + 4 * b2 * i1 * target
        + 4.5 * s2 * target * b2
    )
    subs.objective = linear + quadratic + ho + offset

    # this will be used as the variable in the replacement for substition
    i2 = subs.add_variable("i2", vtype=Vtype.INTEGER)
    # this is the expression which will be used as the replacement for substition
    replacement = 2 * i2
    subs.substitute(target, replacement)

    return subs


def do_checks(translator, sol):
    truth_model = build_truth_model()
    subst_model = build_subst_model()
    lmsol_for_truth_model = build_truth_solution(truth_model)
    lmsol_for_subst_model = build_truth_solution(subst_model)

    with truth_model.environment:
        if isinstance(sol, tuple):
            lmsol_for_sol_truth = translator.to_lm(*sol)
        else:
            lmsol_for_sol_truth = translator.to_lm(sol)
    with subst_model.environment:
        if isinstance(sol, tuple):
            lmsol_for_sol_subst = translator.to_lm(*sol)
        else:
            lmsol_for_sol_subst = translator.to_lm(sol)

    # lmsol_for_dwave_eval_truth = truth_model.evaluate(lmsol_for_sol_truth)
    lmsol_for_dwave_eval_subst = truth_model.evaluate(lmsol_for_sol_subst)

    assert check_solution_contents(lmsol_for_truth_model, lmsol_for_subst_model)
    # assert check_solution_contents(
    #     lmsol_for_dwave_eval_truth, lmsol_for_dwave_eval_subst
    # )
    # assert check_solution_contents(lmsol_for_truth_model, lmsol_for_dwave_eval_truth)
    # assert check_solution_contents(lmsol_for_subst_model, lmsol_for_dwave_eval_subst)
    # assert check_solution_contents(lmsol_for_truth_model, lmsol_for_dwave_eval_subst)
    # assert check_solution_contents(lmsol_for_subst_model, lmsol_for_dwave_eval_truth)
