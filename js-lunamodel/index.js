"use strict";

const { existsSync, readFileSync } = require("node:fs");
const { join } = require("node:path");
const { execSync } = require("node:child_process");

function isMuslFromFilesystem() {
  try {
    return readFileSync("/usr/bin/ldd", "utf8").includes("musl");
  } catch {
    return null;
  }
}

function isMuslFromReport(processObject = process) {
  if (typeof processObject.report?.getReport !== "function") {
    return null;
  }

  const report = processObject.report.getReport();
  if (report.header?.glibcVersionRuntime) {
    return false;
  }

  if (Array.isArray(report.sharedObjects)) {
    return report.sharedObjects.some(
      (file) => file.includes("libc.musl-") || file.includes("ld-musl-"),
    );
  }

  return null;
}

function isMuslFromChildProcess() {
  try {
    return execSync("ldd --version", { encoding: "utf8" }).includes("musl");
  } catch {
    return false;
  }
}

function isMusl(options = {}) {
  const fromFilesystem = options.isMuslFromFilesystem?.() ?? isMuslFromFilesystem();
  if (fromFilesystem !== null) {
    return fromFilesystem;
  }

  const fromReport = options.isMuslFromReport?.() ?? isMuslFromReport();
  if (fromReport !== null) {
    return fromReport;
  }

  return options.isMuslFromChildProcess?.() ?? isMuslFromChildProcess();
}

function platformArchABI(platform, arch, options = {}) {
  if (platform === "linux") {
    return `${platform}-${arch}-${isMusl(options) ? "musl" : "gnu"}`;
  }

  if (platform === "win32") {
    return `${platform}-${arch}-msvc`;
  }

  return `${platform}-${arch}`;
}

function nativeBindingCandidates(baseDir, platform, arch, env) {
  const abi = platformArchABI(platform, arch);

  return [
    env.LUNAMODEL_NATIVE_BINDING,
    join(baseDir, "js_lunamodel.node"),
    join(baseDir, `js_lunamodel.${abi}.node`),
    join(baseDir, `js_lunamodel.${platform}-${arch}.node`)
  ].filter(Boolean);
}

function loadNativeBinding(options = {}) {
  const exists = options.existsSync ?? existsSync;
  const load = options.require ?? require;
  const candidates = nativeBindingCandidates(
    options.dirname ?? __dirname,
    options.platform ?? process.platform,
    options.arch ?? process.arch,
    options.env ?? process.env
  );

  for (const candidate of candidates) {
    if (exists(candidate)) {
      return load(candidate);
    }
  }

  throw new Error(
    `Unable to load js-lunamodel native binding. Tried: ${candidates.join(", ")}`
  );
}

const binding = loadNativeBinding();

Object.defineProperty(binding, "__test", {
  value: {
    loadNativeBinding,
    nativeBindingCandidates,
    platformArchABI,
    isMusl
  }
});

module.exports = binding;
