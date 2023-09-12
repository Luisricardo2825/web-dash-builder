#!/usr/bin/env node
const { build } = require("./main.js");
const { networkInterfaces } = require("os");
const fs = require("fs");
function getIP() {
  const nets = networkInterfaces();
  const results = Object.create(null); // Or just '{}', an empty object

  for (const name of Object.keys(nets)) {
    for (const net of nets[name]) {
      const familyV4Value = typeof net.family === "string" ? "IPv4" : 4;
      if (net.family === familyV4Value && !net.internal) {
        if (!results[name]) {
          results[name] = [];
        }
        results[name].push(net.address);
      }
    }
  }
  const firstKey = Object.keys(results).sort()[0];
  return results[firstKey][0];
}

const BuildDevHtml = (str, host) => {
  const regexSrchref = new RegExp(
    "(src|href|from)(\\=?|.)(\\\"|')(\\.?\\/+[\\w\\s\\#\\/\\-\\.]+\\.(js|tsx|))?(\\\"|')",
    "gm"
  );

  const regexImport =
    /(?:(?<=(?:import)[^`'"]*from\s+[`'"])(?<path1>[^`'"]+)(?=(?:'|"|`)))|(?:\b(?:import)(?:\s+|\s*\(\s*)[`'"](?<path2>[^`'"]+)[`'"])/gm;

  const subst = `${host}$1`;
  //remove / if host contains
  if (host.charAt(host.length - 1) === "/")
    host = host.slice(0, host.length - 1);

  const subst2 = `$1$2"${host}$4"`;
  // The substituted value will be contained in the result variable
  const result = str
    .replace(regexImport, subst)
    .replace(/\.\//gm, "")
    .replace(regexSrchref, subst2);

  return result;
};

const DevHtml = () =>
  `<html lang="en"> <head> <script type="module">import { injectIntoGlobalHook } from "/@react-refresh"; injectIntoGlobalHook(window); window.$RefreshReg$ = () => { }; window.$RefreshSig$ = () => (type) => type;</script> <script type="module" src="/@vite/client"></script> <meta charset="UTF-8"> <link rel="icon" type="image/svg+xml" href="/vite.svg"> <meta name="viewport" content="width=device-width, initial-scale=1.0"> <title>Vite + React + TS</title> </head> <body> <div id="root"></div> <script type="module" src="/src/main.tsx"></script> </body> </html>`;

const plugin = (
  options = {
    devBuildConfig: {
      jsp: [],
      outDir: "SankhyaDevBuild",
      src: "dev",
    },
    prodBuildConfig: {
      jsp: [],
      outDir: "SankhyaBuild",
      src: "build",
    },
  }
) => {
  const { devBuildConfig, prodBuildConfig } = options;
  return [
    {
      name: "build-sankhya", // the name of your custom plugin. Could be anything.
      writeBundle: () => {
        console.log("Iniciando build para o sankhya...");
        build(prodBuildConfig);
        console.log("\nBuild finalizada");
      },
      configureServer(server) {
        server.httpServer.once("listening", () => {
          port = server.config.server.port;
          console.log("Iniciando build para o sankhya...");
          fs.mkdir("./dev", { recursive: true }, (err) => {
            if (err) throw err;
            fs.writeFile(
              "./dev/index.html",
              BuildDevHtml(DevHtml(), `http://${getIP()}:${port}`),
              (err) => {
                if (err) throw err;
                build(devBuildConfig);
              }
            );
          });
        });
      },
      config(config) {
        if (config.define)
          config.define.__IP_SERVER__ = JSON.stringify(getIP());
        else
          config.define = {
            __IP_SERVER__: JSON.stringify(getIP()),
          };
      },
    },
  ];
};
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
module.exports.plugin = plugin;
