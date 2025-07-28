import pytest
from aqmodels.errors import InternalPanicError
from aqmodels.translator import LpTranslator

ILLEGAL_LP = """
// this is something totally unexpected and will definetly panic.
// 
//         this '&' here is a really weird and unexpected token => panics internally.
//         |
//         v
Maximize
  obj: .01 & Pennies + .05 Nickels + .1 Dimes + .25 Quarters + 1 Dollars <<>>
Subject To
  Copper: .06 Pennies + 3.8 Nickels + 2.1 Dimes + 5.2 Quarters + 7.2 Dollars -
     Cu = 0
  Nickel: 1.2 Nickels + .2 Dimes + .5 Quarters + .2 Dollars -
     Ni = 0
  Zinc: 2.4 Pennies + .5 Dollars - Zi = 0
  Manganese: .3 Dollars - Mn = 0
Bounds
  Cu <= 1000
  Ni <= 50
  Zi <= 50
  Mn <= 50
Integers
  Pennies Nickels Dimes Quarters Dollars
End
"""


def test_internal_panic():
    with pytest.raises(InternalPanicError):
        _ = LpTranslator.to_aq(ILLEGAL_LP)

def test_internal_panic_rt_err():
    with pytest.raises(RuntimeError):
        _ = LpTranslator.to_aq(ILLEGAL_LP)
