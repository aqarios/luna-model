"use strict";

const { existsSync } = require("node:fs");
const { join } = require("node:path");

const platform = process.platform;
const arch = process.arch;
const abi = platform === "linux" ? `${platform}-${arch}-gnu` : `${platform}-${arch}`;

const candidates = [
  process.env.LUNAMODEL_NATIVE_BINDING,
  join(__dirname, "js_lunamodel.node"),
  join(__dirname, `js_lunamodel.${abi}.node`),
  join(__dirname, `js_lunamodel.${platform}-${arch}.node`)
].filter(Boolean);

for (const candidate of candidates) {
  if (existsSync(candidate)) {
    module.exports = require(candidate);
    return;
  }
}

throw new Error(
  `Unable to load js-lunamodel native binding. Tried: ${candidates.join(", ")}`
);
