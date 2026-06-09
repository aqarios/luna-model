# LunaModel for JavaScript

JavaScript and TypeScript bindings for [LunaModel](https://github.com/aqarios/luna-model), the
symbolic modeling library for optimization.

## Summary

This package brings LunaModel's **`Solution`** type to JavaScript and TypeScript — a
column-oriented container for solver results — together with a fast binary deserialization
path and helpers for inspecting feasibility, energies, objective values, and timing.

> **Scope.** Today the bindings focus on **reading and working with solutions** that were
> produced by LunaModel's Python or Rust APIs. Constructing models directly from JavaScript is
> not supported yet. Support for model building and more of the LunaModel surface will be added
> in future releases.

## Installation

```sh
npm install @aqarios/luna-model
# or
bun add @aqarios/luna-model
```

The default package ships a native Node addon with prebuilt binaries for Linux, macOS, and
Windows on `x86_64` and `aarch64`. A WebAssembly variant,
`@aqarios/luna-model-wasm32-wasi`, is also available for sandboxed runtimes and the browser,
where a native addon cannot be loaded.

## Usage

```ts
import { Solution } from "@aqarios/luna-model";

// `bytes` is a Uint8Array or Node.js Buffer produced by LunaModel's Solution
// serializer (from the Python or Rust API).
const solution = Solution.deserialize(bytes);

console.log(solution.counts);              // sample-row occurrence counts, e.g. [2, 3]
console.log(solution.feasibilityRatio());  // count-weighted feasible ratio, e.g. 0.4

// Keep only the feasible samples.
const feasibleOnly = solution.filterFeasible();
```

The WebAssembly variant exposes the same surface:

```ts
import { Solution } from "@aqarios/luna-model-wasm32-wasi";
```

## API

### `Solution`

| Member                        | Type                                   | Notes                                                                 |
| ----------------------------- | -------------------------------------- | --------------------------------------------------------------------- |
| `Solution.deserialize(data)`  | `(Uint8Array \| Buffer) → Solution`    | Decode a serialized LunaModel solution. Throws on invalid bytes.      |
| `solution.counts`             | `Array<number>`                        | Sample-row occurrence counts.                                         |
| `solution.rawEnergies`        | `Array<number> \| null`                | Solver-reported energies.                                             |
| `solution.objValues`          | `Array<number> \| null`                | Objective values from re-evaluation.                                  |
| `solution.feasible`           | `Array<boolean> \| null`               | Per-row feasibility.                                                  |
| `solution.constraints`        | `Record<string, Array<boolean>>`       | Per-constraint flags.                                                 |
| `solution.variableBounds`     | `Record<string, Array<boolean>>`       | Per-variable bound flags.                                             |
| `solution.timing`             | `Timing \| null`                       | Wall-clock and QPU runtime metrics.                                   |
| `solution.feasibilityRatio()` | `() → number`                          | Count-weighted feasible ratio. Throws if `feasible` is null.          |
| `solution.filterFeasible()`   | `() → Solution`                        | New solution with only feasible rows. Throws if `feasible` is null.   |

### `Timing`

| Member                | Type             | Notes                                                            |
| --------------------- | ---------------- | ---------------------------------------------------------------- |
| `timing.start`        | `number`         | Wall-clock start, ms since the Unix epoch (`new Date(start)`).   |
| `timing.end`          | `number`         | Wall-clock end, ms since the Unix epoch.                         |
| `timing.totalSeconds` | `number`         | `end − start` in seconds. Throws on inconsistent timestamps.     |
| `timing.qpu`          | `number \| null` | QPU usage time in seconds, when reported.                        |

## Errors

Errors surface as standard JavaScript `Error`s with NAPI status codes. Invalid input bytes,
missing feasibility data, and out-of-range integers raise `InvalidArg`-shaped errors, with the
underlying LunaModel message preserved.

## Roadmap

The bindings are intentionally small for now and will grow over time. Planned additions
include constructing models from JavaScript and broader coverage of the LunaModel API
(expressions, constraints, and translation). Follow the
[main repository](https://github.com/aqarios/luna-model) for progress.

## License

Apache-2.0. See [LICENSE](../LICENSE).
