#!/usr/bin/env node
const { build } = require("./main.js");
const { networkInterfaces } = require("os");
const { mkdir, writeFile } = require("fs");
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

const DevHtml = (host) =>
  `<html lang="en"> <head> <script type="module">import { injectIntoGlobalHook } from "${host}/@react-refresh"; injectIntoGlobalHook(window); window.$RefreshReg$ = () => { }; window.$RefreshSig$ = () => (type) => type;</script> <script type="module" src="${host}/@vite/client"></script> <meta charset="UTF-8"> <link rel="icon" type="image/svg+xml" href="${host}/vite.svg"> <meta name="viewport" content="width=device-width, initial-scale=1.0"> <title>Vite + React + TS</title> </head> <body> <div id="root"></div> <script type="module" src="${host}/src/main.tsx"></script> </body> </html>`;

const plugin = (
  options = {
    devConfig: {
      jsp: [],
      outDir: "SankhyaDevBuild",
      src: "dev",
    },
    prodConfig: {
      jsp: [],
      outDir: "SankhyaBuild",
      src: "dist",
    },
  }
) => {
  const { devConfig, prodConfig } = options;
  let userConfig = {};
  return [
    {
      name: "build-sankhya", // the name of your custom plugin. Could be anything.
      writeBundle: () => {
        console.log("Iniciando build para o sankhya...");
        if (userConfig?.build?.outDir) {
          prodConfig.src = userConfig.build.outDir;
        }
        build(prodConfig);
        console.log("\nBuild finalizada");
      },
      configureServer(server) {
        server.httpServer.once("listening", () => {
          let port = server.config.server.port;
          console.log("Iniciando build para o sankhya...");
          mkdir("./dev", { recursive: true }, (err) => {
            if (err) throw err;
            let devFolderPath = "dev";
            if (userConfig?.build?.outDir) {
              devConfig.src = userConfig.build.outDir;
              devFolderPath = userConfig.build.outDir;
            }
            writeFile(
              devFolderPath + "/index.html",
              DevHtml(`http://${getIP()}:${port}`),
              (err) => {
                if (err) throw err;
                build(devConfig);
              }
            );
          });
        });
      },
      config(config) {
        userConfig = config;
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
  // eslint-disable-next-line no-undef
  const args = process.argv.slice(2);

  // eslint-disable-next-line no-useless-escape
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
