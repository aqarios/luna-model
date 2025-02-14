#!/bin/bash

mkdir quad_timings_detailed

for size in 100 200 400 600
do
  for density in 0.1 0.5 1.0
  do
    uv run tests/bench.py $size $density > quad_timings_detailed/${size}_${density}.txt
    echo "Executed: uv run tests/bench.py $size $density"
  done
done

echo "Done."
