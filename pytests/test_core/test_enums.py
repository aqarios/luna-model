from enum import Enum
from aqmodels import Vtype, Sense, Comparator


class ExpectedVtype(Enum):
    Real = "Real"
    Integer = "Integer"
    Binary = "Binary"
    Spin = "Spin"


class ExpectedSense(Enum):
    Min = "Minimize"
    Max = "Maximize"


class ExpectedComparator(Enum):
    Eq = "Eq"
    Le = "Le"
    Ge = "Ge"


def test_name_vtype():
    assert Vtype.Real.name == ExpectedVtype.Real.name
    assert Vtype.Integer.name == ExpectedVtype.Integer.name
    assert Vtype.Binary.name == ExpectedVtype.Binary.name
    assert Vtype.Spin.name == ExpectedVtype.Spin.name


def test_value_vtype():
    assert Vtype.Real.value == ExpectedVtype.Real.value
    assert Vtype.Integer.value == ExpectedVtype.Integer.value
    assert Vtype.Binary.value == ExpectedVtype.Binary.value
    assert Vtype.Spin.value == ExpectedVtype.Spin.value


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
