import { expect, test } from "bun:test";

import { bytesFromHex, TIMING_SOLUTION_HEX } from "./fixtures";

const { Solution } = require("../index.js") as typeof import("../index");

test("exposes timing as a JavaScript object with millisecond timestamps", () => {
  const solution = Solution.deserialize(bytesFromHex(TIMING_SOLUTION_HEX));
  const timing = solution.timing;

  expect(timing).not.toBeNull();
  expect(timing!.start).toBe(1000);
  expect(timing!.end).toBe(3000);
  expect(new Date(timing!.start).getTime()).toBe(1000);
  expect(timing!.totalSeconds).toBe(2);
  expect(timing!.qpu).toBe(0.25);
});
