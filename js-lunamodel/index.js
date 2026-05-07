"use strict";

const { existsSync } = require("node:fs");
const { join } = require("node:path");

function nativeBindingCandidates(baseDir, platform, arch, env) {
  const abi = platform === "linux" ? `${platform}-${arch}-gnu` : `${platform}-${arch}`;

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
    nativeBindingCandidates
  }
});

module.exports = binding;
