import { expect, test } from "bun:test";

import { bytesFromHex, NULL_SOLUTION_HEX } from "./fixtures";

const { Solution, Sense } = require("../index.js") as typeof import("../index");

test("exposes null and empty defaults for absent optional Solution fields", () => {
  const solution = Solution.deserialize(bytesFromHex(NULL_SOLUTION_HEX));

  expect(solution.counts).toEqual([]);
  expect(solution.rawEnergies).toBeNull();
  expect(solution.objValues).toBeNull();
  expect(solution.feasible).toBeNull();
  expect(solution.constraints).toEqual({});
  expect(solution.variableBounds).toEqual({});
  expect(solution.timing).toBeNull();
  expect(solution.sense).toEqual(Sense.Min);
});

test("throws when computing feasibility ratio without feasibility data", () => {
  const solution = Solution.deserialize(bytesFromHex(NULL_SOLUTION_HEX));

  expect(() => solution.feasibilityRatio()).toThrow(/feasible is not set/);
});

test("throws when filtering feasible rows on a non-evaluated solution", () => {
  const solution = Solution.deserialize(bytesFromHex(NULL_SOLUTION_HEX));

  expect(() => solution.filterFeasible()).toThrow(
    /filter_feasible is not possible on non-evaluated solution/,
  );
});
