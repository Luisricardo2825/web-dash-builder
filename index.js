#!/usr/bin/env node
const { build } = require("./main.js");

function buildCommand() {
  const args = process.argv.slice(2);

  const regex = /--build(\=)?([\w\W]*)?/;

  const exec = args.find((item) => regex.test(item));
  if (exec) {
    const result = regex.exec(exec);
    let params = null;
    if (result) {
      if (result[2] && result[2].includes(",")) {
        params = result[2].split(",");
      } else {
        params = result[2];
      }
    }
    if (!Array.isArray(params)) return build(params);
    params.forEach((o) => {
      build(o);
    });
  }
}

buildCommand();
module.exports.build = build;
