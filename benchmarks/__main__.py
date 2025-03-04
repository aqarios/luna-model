from benchmarks.translator.read_dense_qubo import bench_read_dense_qubo
from benchmarks.serialization.serialize import (
    bench_serialize_bqm,
    bench_serialize_bqm_size,
)
from benchmarks.serialization.deserialize import (
    bench_deserialize_bqm,
)
from benchmarks.serialization.serialize_extralarge import (
    bench_serialize_aqm_xl,
)


bench_read_dense_qubo()
bench_serialize_bqm()
bench_deserialize_bqm()
bench_serialize_bqm_size()

bench_serialize_aqm_xl()
