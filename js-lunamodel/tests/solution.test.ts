import { expect, test } from "bun:test";

const { Solution } = require("../index.js") as typeof import("../index");

const DEFAULT_SOLUTION_HEX = "0801120c120a1a084d696e696d697a65";

function bytesFromHex(hex: string): Uint8Array {
  return Uint8Array.from(Buffer.from(hex, "hex"));
}

test("exports the Solution binding", () => {
  expect(Solution).toBeDefined();
  expect(typeof Solution.deserialize).toBe("function");
});

test("deserializes a valid encoded Solution from Uint8Array", () => {
  const solution = Solution.deserialize(bytesFromHex(DEFAULT_SOLUTION_HEX));

  expect(solution).toBeInstanceOf(Solution);
});

test("deserializes a valid encoded Solution from Buffer", () => {
  const solution = Solution.deserialize(Buffer.from(DEFAULT_SOLUTION_HEX, "hex"));

  expect(solution).toBeInstanceOf(Solution);
});

test("throws a useful error for invalid Solution bytes", () => {
  expect(() => Solution.deserialize(new Uint8Array([1, 2, 3]))).toThrow(
    /failed to deserialize LunaModel Solution/,
  );
});
