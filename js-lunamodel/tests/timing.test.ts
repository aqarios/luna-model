import { expect, test } from "bun:test";

import { bytesFromHex, TIMING_SOLUTION_HEX } from "./fixtures";

const { Solution } = require("../index.js") as typeof import("../index");

test("exposes timing as a JavaScript object with Date and number properties", () => {
  const solution = Solution.deserialize(bytesFromHex(TIMING_SOLUTION_HEX));
  const timing = solution.timing;

  expect(timing).not.toBeNull();
  expect(timing!.start).toBeInstanceOf(Date);
  expect(timing!.end).toBeInstanceOf(Date);
  expect(timing!.start.getTime()).toBe(1000);
  expect(timing!.end.getTime()).toBe(3000);
  expect(timing!.totalSeconds).toBe(2);
  expect(timing!.qpu).toBe(0.25);
});
