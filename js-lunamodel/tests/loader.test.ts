import { expect, test } from "bun:test";

const binding = require("../index.js") as typeof import("../index") & {
  __test: {
    loadNativeBinding(options: {
      existsSync?: (path: string) => boolean;
      require?: (path: string) => unknown;
      dirname?: string;
      platform?: string;
      arch?: string;
      env?: Record<string, string | undefined>;
    }): unknown;
    nativeBindingCandidates(
      baseDir: string,
      platform: string,
      arch: string,
      env: Record<string, string | undefined>,
    ): string[];
  };
};

test("native loader tries environment and platform candidates", () => {
  expect(
    binding.__test.nativeBindingCandidates("/pkg", "linux", "x64", {
      LUNAMODEL_NATIVE_BINDING: "/custom/native.node",
    }),
  ).toEqual([
    "/custom/native.node",
    "/pkg/js_lunamodel.node",
    "/pkg/js_lunamodel.linux-x64-gnu.node",
    "/pkg/js_lunamodel.linux-x64.node",
  ]);
});

test("native loader throws a useful error when no candidate exists", () => {
  expect(() =>
    binding.__test.loadNativeBinding({
      existsSync: () => false,
      dirname: "/pkg",
      platform: "darwin",
      arch: "arm64",
      env: {},
    }),
  ).toThrow(/Unable to load js-lunamodel native binding/);
});
