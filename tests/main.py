import concurrent.futures
from lm import Environment, Variable
from accext import dbg_print, add_many_variables


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

    # add_many_variables(env._env, ["a", "b", "c", "d", "e"])
    # dbg_print(env._env)

    # print(env._env)


if __name__ == "__main__":
    main()
