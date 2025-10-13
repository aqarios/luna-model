import pickle

from aqmodels import Model


def test_pickle_model():
    m = Model()
    blob = pickle.dumps(m)
    print(blob)
    mp = pickle.loads(blob)
    print(mp)
