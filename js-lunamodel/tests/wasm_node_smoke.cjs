"use strict";

const { existsSync } = require("node:fs");
const { join } = require("node:path");
const assert = require("node:assert/strict");

const packageRoot = join(__dirname, "..");
const wasiLoader = join(packageRoot, "js_lunamodel.wasi.cjs");
const wasmArtifact = join(packageRoot, "js_lunamodel.wasm32-wasi.wasm");

if (!existsSync(wasiLoader) || !existsSync(wasmArtifact)) {
  throw new Error(
    "Missing WASM loader or artifact. Run `bunx napi build --target wasm32-wasip1-threads --platform` first."
  );
}

const { Solution } = require(wasiLoader);

const fullSolutionHex =
  "08011258125608021201781a084d696e696d697a652a01003201023801620202036a10000000000000244000000000000034407210000000000000f83f00000000000004408a0101019001019a010103a00101aa01026330b2010178";

const solution = Solution.deserialize(Buffer.from(fullSolutionHex, "hex"));

assert.deepEqual(solution.counts, [2, 3]);
assert.deepEqual(solution.rawEnergies, [1.5, 2.5]);
assert.deepEqual(solution.objValues, [10, 20]);
assert.deepEqual(solution.feasible, [true, false]);
assert.deepEqual(solution.constraints, { c0: [true, false] });
assert.deepEqual(solution.variableBounds, { x: [true, true] });
assert.equal(solution.feasibilityRatio(), 0.4);

console.log("WASM Solution smoke test passed");
