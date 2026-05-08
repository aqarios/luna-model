import { expect, test } from "bun:test";

import { bytesFromHex, FULL_SOLUTION_HEX } from "./fixtures";

const { Solution } = require("../index.js") as typeof import("../index");

test("exposes counts as a read-only JavaScript property", () => {
  const solution = Solution.deserialize(bytesFromHex(FULL_SOLUTION_HEX));

  expect(solution.counts).toEqual([2, 3]);
});

test("exposes raw energies as a read-only JavaScript property", () => {
  const solution = Solution.deserialize(bytesFromHex(FULL_SOLUTION_HEX));

  expect(solution.rawEnergies).toEqual([1.5, 2.5]);
});

test("exposes objective values as a read-only JavaScript property", () => {
  const solution = Solution.deserialize(bytesFromHex(FULL_SOLUTION_HEX));

  expect(solution.objValues).toEqual([10, 20]);
});

test("exposes feasibility flags as a read-only JavaScript property", () => {
  const solution = Solution.deserialize(bytesFromHex(FULL_SOLUTION_HEX));

  expect(solution.feasible).toEqual([true, false]);
});

test("computes the count-weighted feasibility ratio", () => {
  const solution = Solution.deserialize(bytesFromHex(FULL_SOLUTION_HEX));

  expect(solution.feasibilityRatio()).toBe(0.4);
});

test("filters to feasible sample rows", () => {
  const solution = Solution.deserialize(bytesFromHex(FULL_SOLUTION_HEX));
  const filtered = solution.filterFeasible();

  expect(filtered).toBeInstanceOf(Solution);
  expect(filtered.counts).toEqual([2]);
  expect(filtered.rawEnergies).toEqual([1.5]);
  expect(filtered.objValues).toEqual([10]);
  expect(filtered.feasible).toEqual([true]);
  expect(filtered.constraints).toEqual({ c0: [true] });
  expect(filtered.variableBounds).toEqual({ x: [true] });
  expect(filtered.feasibilityRatio()).toBe(1);
});

test("exposes constraints as a read-only JavaScript property", () => {
  const solution = Solution.deserialize(bytesFromHex(FULL_SOLUTION_HEX));

  expect(solution.constraints).toEqual({ c0: [true, false] });
});

test("exposes variable bounds as a read-only JavaScript property", () => {
  const solution = Solution.deserialize(bytesFromHex(FULL_SOLUTION_HEX));

  expect(solution.variableBounds).toEqual({ x: [true, true] });
});
