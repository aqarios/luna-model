\ Model TestModel
\ Problem name: TestModel

Minimize
 obj: 0.01 Pennies + 0.05 Nickels + 0.1 Dimes + 0.25 Quarters + Dollars
Subject To
 Copper: 0.06 Pennies + 3.8 Nickels + 2.1 Dimes + 5.2 Quarters + 7.2 Dollars - 1 Cu = 0
 Nickel: 1.2 Nickels + 0.2 Dimes + 0.5 Quarters + 0.2 Dollars - 1 Ni = 0
 Zinc: 2.4 Pennies + 0.5 Dollars - 1 Zi = 0
 Manganese: 0.3 Dollars - 1 Mn = 0
Bounds
 Pennies >= 1
 Nickels >= 1
 Dimes >= 1
 Quarters >= 1
 Dollars >= 1
 0 <= Cu <= 1000
 0 <= Ni <= 50
 0 <= Zi <= 50
 0 <= Mn <= 50
Generals
 Pennies Nickels Dimes Quarters Dollars
Semi-Continuous
 Cu Ni Zi Mn
End
