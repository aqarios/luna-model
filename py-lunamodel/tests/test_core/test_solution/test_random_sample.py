import numpy as np
import pytest
from luna_model import (
    Environment,
    Model,
    Solution,
    Variable,
    Vtype,
    quicksum,
    Bounds,
    Unbounded,
)
from luna_model.errors import RandomSamplingError


def get_vars(n, vtype, lb=Unbounded, ub=Unbounded, start=0, env=None) -> tuple[tuple[Variable, ...], Environment]:
    if env is None:
        env = Environment()
    with env:
        if vtype == Vtype.INTEGER or vtype == Vtype.REAL:
            bounds = Bounds(lower=lb, upper=ub)
        else:
            bounds = None
        variables = [Variable(f"x_{i}", vtype=vtype, bounds=bounds) for i in range(start, n + start)]
    return tuple(variables), env


@pytest.fixture()
def model(request):
    i = 0
    model = Model()
    vlist = []
    for p in request.param:
        vars, _ = get_vars(*p, start=i, env=model.environment)
        vlist.extend(vars)
        i += p[0]
    model.objective = -quicksum(vlist)
    return model, tuple(vlist)


@pytest.mark.solution()
@pytest.mark.parametrize("model", [((3, Vtype.BINARY),)], indirect=True)
def test_random_binary(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    sol = Solution.from_random(100, model=m)

    assert sum(sol.counts) == 100
    assert len(sol) < 100


@pytest.mark.solution()
@pytest.mark.parametrize("model", [((3, Vtype.SPIN),)], indirect=True)
def test_random_spin(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    sol = Solution.from_random(100, model=m)

    assert sum(sol.counts) == 100
    assert len(sol) < 100


@pytest.mark.solution()
@pytest.mark.parametrize("model", [((3, Vtype.INTEGER),)], indirect=True)
def test_random_integer_unbounded(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    sol = Solution.from_random(100, model=m)

    assert sum(sol.counts) == 100


@pytest.mark.solution()
@pytest.mark.parametrize("model", [((3, Vtype.INTEGER, 0, 3),)], indirect=True)
def test_random_integer_bounded(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    sol = Solution.from_random(100, model=m)

    assert sum(sol.counts) == 100
    assert len(sol) < 100


@pytest.mark.solution()
@pytest.mark.parametrize("model", [((3, Vtype.REAL),)], indirect=True)
def test_random_real_unbounded(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    with pytest.raises(RandomSamplingError):
        Solution.from_random(100, model=m)


@pytest.mark.solution()
@pytest.mark.parametrize("model", [((3, Vtype.REAL, 0, 10),)], indirect=True)
def test_random_real_bounded(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    sol = Solution.from_random(100, model=m)

    assert sum(sol.counts) == 100


@pytest.mark.solution()
@pytest.mark.parametrize(
    "model",
    [
        ((3, Vtype.INTEGER, 10, -10),),
    ],
    indirect=True,
)
def test_random_bad_bounds(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    with pytest.raises(RandomSamplingError):
        Solution.from_random(100, model=m)


SOL_BLOB = b'\x08\x01\x12\x87\x04\x08\x01\x12\x82\x04(\xb5/\xfd\x00X\xcd\x0f\x00\x86\x90<5@u\x1a\x03\x9d$\xce\x04t\xb6\x9f\x1c?2&\x1a=\xb5\x17\xc9\x9a\x83\xb9\xee\xbeX\xba\xe9\xd8i\x93\xcdCU\xf5\x19\x86\xe8\x89\x1c\xd46d\x17\xb4$\x17\xb0[\xa1L)\x054\x008\x00\x1d\x00\x01\xd9\xe8\xb7\xdb\xcb\xa1#\xb0\xb09\xbdP\xb8\xacp\x08D\x85\x8f\x92n\xa4#\xe940\x83\x12\xe7\xc18\xc1y0\x928\x0f\xc6\x11\xe7\xc1(\xc1y0Fp\x1e\x8c),\xb5\xe7\xf7\xd6\xf3JA=\xc3?@-3\xa6\xa0\x05\xd4N*!D\xb5\xf1\xb0\xea\xc0\xa8`\xd0aA\xc8.n\xea\xea\x06g \xa7\xa5\x02\x95\x0f\x08\xc4\xec\xa1\x91\x81\xa9QY\x91b\x02\xed\xd7\xff\xfe\xbe\xf7\x9e}\xdf\xd7\xff\xec\xed\xd9\xd6\xd3m\xff\xff\xbd=\xe7m\xf7\x9e\xd3\xf7\x9c_\x97Q\xb4\x93O\x1al\xc2j\xc9b\x95ZE\xa1\x90\xdc\xb6M\x1ehm\xd6\r\x93Yh\xab\x14I\xacZ\xab\xacV\x01xD|\xdf\xd0\xbeS\x1e\x01\x80\x9f\xa8!F\xdb\xa6\xb0\x7f\x1d\xd0\x82\x99\xa1\xea\x06\x12\xa8\xe0`Z\x8bQ\xcbhhIS\xd9\xda;\x02\x81\xae\xf9nt\x07\xf1\xf11\xd7\x84\x8dJ\xe80/\xe8\x8cR[\xfc@\x1a`\xe2\xba:\x85\x93\xdfIG\xb9#\x9b\xdc\xe4Y\x0e\xb4"\x8e\xe9F\x1e\xb8\x94Q\xaa\xc35\x1f\x08\xafS\xc8\xb0\xces\xe7\xee\xa6Q\x049,\xb8\x13rj\x7f\xaa=\xc32\x8d9\x90\xc6bW\x08;\x80\xdc\xaf\xca\x14 4\xfa5Y\xf5\xc5\xcb(G\x8f.\xfa\x98R\x95\xa4i\xbcL\xac\xa4\xbe.\x1b\xe6Yf\xe1\xe4\x15\x92\xb7}PJl\xcc\x87\x08d\xa9\xea\xcd\\\xac~$#\xa6b\x86\xe3v\xa4Ubq\xc5\xfe\x85\x0765\xb93A8\x1c\xb1M4\xa4g c\x84\x83R\xea/x\x02\xbf\xea\xb0\xe2\x973\x01\xbcl\x01\xcf\xc6\x8e\xab\x12u$gT\x1b\xf4I\x1aY|\xd8gv\rm R\x8f\xab\x01\xba\xdd\x91\xe6\xfb\xce\x14\xb2\xfc\x02\xc0;y\x13\xbcR\x04K\xd51(\xd4'


@pytest.mark.solution()
@pytest.mark.parametrize(
    "model",
    [((2, Vtype.BINARY), (2, Vtype.SPIN), (2, Vtype.INTEGER, -2, 2))],
    indirect=True,
)
def test_random_seed(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    sol = Solution.from_random(100, model=m, seed=123456)
    sol_assert = Solution.decode(SOL_BLOB)

    assert np.all(np.array(sol_assert.samples.tolist()) == np.array(sol.samples.tolist()))
