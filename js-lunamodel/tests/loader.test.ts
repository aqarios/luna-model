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
    platformArchABI(
      platform: string,
      arch: string,
      options?: {
        isMuslFromFilesystem?: () => boolean | null;
        isMuslFromReport?: () => boolean | null;
        isMuslFromChildProcess?: () => boolean;
      },
    ): string;
    isMusl(options?: {
      isMuslFromFilesystem?: () => boolean | null;
      isMuslFromReport?: () => boolean | null;
      isMuslFromChildProcess?: () => boolean;
    }): boolean;
    isMuslFromFilesystem(options?: {
      readFileSync?: (path: string) => Buffer;
    }): boolean | null;
    isMuslFromReport(processObject?: NodeJS.Process | Record<string, unknown>): boolean | null;
    isMuslFromChildProcess(options?: {
      execSync?: (cmd: string, opts: { encoding: "utf8" }) => string;
    }): boolean;
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

test("native loader uses NAPI-RS platform ABI names", () => {
  expect(binding.__test.platformArchABI("darwin", "arm64")).toBe("darwin-arm64");
  expect(
    binding.__test.platformArchABI("linux", "x64", {
      isMuslFromFilesystem: () => false,
    }),
  ).toBe("linux-x64-gnu");
  expect(
    binding.__test.platformArchABI("linux", "x64", {
      isMuslFromFilesystem: () => true,
    }),
  ).toBe("linux-x64-musl");
  expect(binding.__test.platformArchABI("win32", "x64")).toBe("win32-x64-msvc");
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

test("isMuslFromFilesystem returns false on a glibc-shaped ldd", () => {
  expect(
    binding.__test.isMuslFromFilesystem({
      readFileSync: () => Buffer.from("not the libc you're looking for"),
    }),
  ).toBe(false);
});

test("isMuslFromFilesystem returns true when ldd reports musl", () => {
  expect(
    binding.__test.isMuslFromFilesystem({
      readFileSync: () => Buffer.from("musl libc"),
    }),
  ).toBe(true);
});

test("isMuslFromFilesystem returns null when ldd is unreadable", () => {
  expect(
    binding.__test.isMuslFromFilesystem({
      readFileSync: () => {
        throw new Error("ENOENT");
      },
    }),
  ).toBeNull();
});

test("isMuslFromReport returns null when process.report is unavailable", () => {
  expect(binding.__test.isMuslFromReport({})).toBeNull();
});

test("isMuslFromReport returns false when glibc runtime is reported", () => {
  expect(
    binding.__test.isMuslFromReport({
      report: {
        getReport: () => ({ header: { glibcVersionRuntime: "2.31" } }),
      },
    }),
  ).toBe(false);
});

test("isMuslFromReport detects musl in shared objects", () => {
  expect(
    binding.__test.isMuslFromReport({
      report: {
        getReport: () => ({ sharedObjects: ["/lib/libc.musl-x86_64.so.1"] }),
      },
    }),
  ).toBe(true);
  expect(
    binding.__test.isMuslFromReport({
      report: {
        getReport: () => ({ sharedObjects: ["/lib/libfoo.so"] }),
      },
    }),
  ).toBe(false);
});

test("isMuslFromReport returns null when shared objects are missing", () => {
  expect(
    binding.__test.isMuslFromReport({
      report: { getReport: () => ({}) },
    }),
  ).toBeNull();
});

test("isMuslFromChildProcess detects musl in ldd --version", () => {
  expect(
    binding.__test.isMuslFromChildProcess({
      execSync: () => "musl libc (x86_64)",
    }),
  ).toBe(true);
});

test("isMuslFromChildProcess returns false on glibc ldd output", () => {
  expect(
    binding.__test.isMuslFromChildProcess({
      execSync: () => "ldd (GNU libc) 2.31",
    }),
  ).toBe(false);
});

test("isMuslFromChildProcess returns false when ldd is missing", () => {
  expect(
    binding.__test.isMuslFromChildProcess({
      execSync: () => {
        throw new Error("command not found");
      },
    }),
  ).toBe(false);
});

test("isMusl returns the filesystem verdict when conclusive", () => {
  expect(
    binding.__test.isMusl({
      isMuslFromFilesystem: () => true,
    }),
  ).toBe(true);
});

test("isMusl falls through to report when filesystem is inconclusive", () => {
  expect(
    binding.__test.isMusl({
      isMuslFromFilesystem: () => null,
      isMuslFromReport: () => true,
    }),
  ).toBe(true);
});

test("isMusl falls through to childProcess when both prior are inconclusive", () => {
  expect(
    binding.__test.isMusl({
      isMuslFromFilesystem: () => null,
      isMuslFromReport: () => null,
      isMuslFromChildProcess: () => false,
    }),
  ).toBe(false);
});
