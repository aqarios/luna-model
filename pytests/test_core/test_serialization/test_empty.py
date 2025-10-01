from aqmodels import Model, Environment, Expression


def test_encode_decode_empty_model():
    m = Model()
    m_bytes = m.encode()
    m_out = Model.decode(m_bytes)
    assert m.equal_contents(m_out)


def test_encode_decode_empty_environment():
    e = Environment()
    e_bytes = e.encode()
    e_out = Environment.decode(e_bytes)
    assert e.equal_contents(e_out)


def test_encode_decode_empty_expression():
    e = Expression(env=Environment())
    e_bytes = e.encode()
    e_out = Expression.decode(e_bytes, env=Environment())
    assert e.equal_contents(e_out)
