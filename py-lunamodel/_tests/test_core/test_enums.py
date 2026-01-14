from enum import Enum

from luna_model import Comparator, Sense, Vtype


class ExpectedVtype(Enum):
    REAL = "Real"
    INTEGER = "Integer"
    BINARY = "Binary"
    SPIN = "Spin"


class ExpectedSense(Enum):
    Min = "Minimize"
    Max = "Maximize"


class ExpectedComparator(Enum):
    Eq = "Eq"
    Le = "Le"
    Ge = "Ge"


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
    assert Sense.Min.name == ExpectedSense.Min.name
    assert Sense.Max.name == ExpectedSense.Max.name


def test_value_sense():
    assert Sense.Min.value == ExpectedSense.Min.value
    assert Sense.Max.value == ExpectedSense.Max.value


def test_name_comparator():
    assert Comparator.Eq.name == ExpectedComparator.Eq.name
    assert Comparator.Le.name == ExpectedComparator.Le.name
    assert Comparator.Ge.name == ExpectedComparator.Ge.name


def test_value_comparator():
    assert Comparator.Eq.value == ExpectedComparator.Eq.value
    assert Comparator.Le.value == ExpectedComparator.Le.value
    assert Comparator.Ge.value == ExpectedComparator.Ge.value
