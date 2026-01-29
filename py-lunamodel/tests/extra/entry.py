from luna_model._lm import PyVtype

from luna_model import Environment, Expression, Variable, Vtype

env = Environment()
expr = Expression(env)
vara = Variable("vara", Vtype.BINARY, env=env)
var = Variable("var", Vtype.BINARY, env=env)

print(vara)
print(var)

print(Vtype(PyVtype.Binary))
