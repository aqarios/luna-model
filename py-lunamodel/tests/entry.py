from luna_model import Expression, Environment, Variable, Vtype
from luna_model._lm import PyVtype


env = Environment()
expr = Expression(env)
vara = Variable("vara", Vtype.Binary, env=env)
var = Variable("var", Vtype.BINARY, env=env)

print(vara)
print(var)

print(Vtype(PyVtype.Binary))
