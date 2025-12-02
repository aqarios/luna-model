import concurrent.futures
from luna_model import Environment, Variable, Expression
from accext import dbg_print, dbg_print_e, add_many_variables, add_constant_two


def _spawn_threads_and_wait(worker, threads=4):
    with concurrent.futures.ThreadPoolExecutor(max_workers=threads) as ex:
        futures = [ex.submit(worker, i) for i in range(threads)]
        for fut in concurrent.futures.as_completed(futures):
            fut.result()


def create_variables_par(env: Environment):
    threads = 20

    def worker(idx):
        if idx < 10:
            _ = Variable(f"direct_{idx}", env)
        else:
            idx -= 10
            a, b = idx * 2, (idx * 2) + 1
            add_many_variables(env, [f"plugin_{a}", f"plugin_{b}"])

    _spawn_threads_and_wait(worker, threads=threads)


def add_two_many_times_par(expr: Expression):
    threads = 20

    def worker(idx):
        if idx < 10:
            expr.__iadd__(2)
        else:
            add_constant_two(expr)

    _spawn_threads_and_wait(worker, threads=threads)


# def create_variables_in_plugin(env: Environment):
#     threads = 5
#     def worker(idx):
#         a, b = idx * 2, (idx * 2) + 1
#         add_many_variables(env, [f"plugin_{a}", f"plugin_{b}"])
#
#     _spawn_threads_and_wait(worker, threads=threads)


def main():
    print("Hello from tests!")

    env = Environment()
    dbg_print(env)
    create_variables_par(env)
    dbg_print(env)

    print("------")

    expr = Expression(Environment())
    dbg_print_e(expr)
    add_two_many_times_par(expr)
    dbg_print_e(expr)

    # add_many_variables(env._env, ["a", "b", "c", "d", "e"])
    # dbg_print(env._env)

    # print(env._env)


if __name__ == "__main__":
    main()
