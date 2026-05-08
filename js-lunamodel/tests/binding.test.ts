import { expect, test } from "bun:test";

const { Solution } = require("../index.js") as typeof import("../index");

test("exports the Solution binding", () => {
  expect(Solution).toBeDefined();
  expect(typeof Solution.deserialize).toBe("function");
});
