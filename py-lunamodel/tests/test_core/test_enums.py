from enum import Enum

from luna_model import Comparator, Sense, Vtype


class ExpectedVtype(Enum):
    REAL = "Real"
    INTEGER = "Integer"
    BINARY = "Binary"
    SPIN = "Spin"


class ExpectedSense(Enum):
    MIN = "Minimize"
    MAX = "Maximize"


class ExpectedComparator(Enum):
    EQ = "Eq"
    LE = "Le"
    GE = "Ge"


def test_name_vtype():
    assert Vtype.REAL.name == ExpectedVtype.REAL.name
    assert Vtype.INTEGER.name == ExpectedVtype.INTEGER.name
    assert Vtype.BINARY.name == ExpectedVtype.BINARY.name
    assert Vtype.SPIN.name == ExpectedVtype.SPIN.name


def test_value_vtype():
    assert Vtype.REAL.value == ExpectedVtype.REAL.value
    assert Vtype.INTEGER.value == ExpectedVtype.INTEGER.value
    assert Vtype.BINARY.value == ExpectedVtype.BINARY.value
    assert Vtype.SPIN.value == ExpectedVtype.SPIN.value


def test_name_sense():
    assert Sense.MIN.name == ExpectedSense.MIN.name
    assert Sense.MAX.name == ExpectedSense.MAX.name


def test_value_sense():
    assert Sense.MIN.value == ExpectedSense.MIN.value
    assert Sense.MAX.value == ExpectedSense.MAX.value


def test_name_comparator():
    assert Comparator.EQ.name == ExpectedComparator.EQ.name
    assert Comparator.LE.name == ExpectedComparator.LE.name
    assert Comparator.GE.name == ExpectedComparator.GE.name


def test_value_comparator():
    assert Comparator.EQ.value == ExpectedComparator.EQ.value
    assert Comparator.LE.value == ExpectedComparator.LE.value
    assert Comparator.GE.value == ExpectedComparator.GE.value
