import { expect, test } from "bun:test";

import { bytesFromHex, NULL_SOLUTION_HEX } from "./fixtures";

const { Solution } = require("../index.js") as typeof import("../index");

test("exposes null and empty defaults for absent optional Solution fields", () => {
  const solution = Solution.deserialize(bytesFromHex(NULL_SOLUTION_HEX));

  expect(solution.counts).toEqual([]);
  expect(solution.rawEnergies).toBeNull();
  expect(solution.objValues).toBeNull();
  expect(solution.feasible).toBeNull();
  expect(solution.constraints).toEqual({});
  expect(solution.variableBounds).toEqual({});
  expect(solution.timing).toBeNull();
});
