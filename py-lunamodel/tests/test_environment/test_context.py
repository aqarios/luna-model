from luna_model import Environment, Expression


def test_context_passing():
    env = Environment()
    with env:
        _ = Expression()


if __name__ == "__main__":
    test_context_passing()
