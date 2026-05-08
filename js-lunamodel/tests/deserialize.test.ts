import { expect, test } from "bun:test";

import { bytesFromHex, FULL_SOLUTION_HEX } from "./fixtures";

const { Solution } = require("../index.js") as typeof import("../index");

test("deserializes a valid encoded Solution from Uint8Array", () => {
  const solution = Solution.deserialize(bytesFromHex(FULL_SOLUTION_HEX));

  expect(solution).toBeInstanceOf(Solution);
});

test("deserializes a valid encoded Solution from Buffer", () => {
  const solution = Solution.deserialize(Buffer.from(FULL_SOLUTION_HEX, "hex"));

  expect(solution).toBeInstanceOf(Solution);
});

test("throws a useful error for invalid Solution bytes", () => {
  expect(() => Solution.deserialize(new Uint8Array([1, 2, 3]))).toThrow(
    /decoding failed/,
  );
});
