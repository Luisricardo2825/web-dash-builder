#!/usr/bin/env node
/// <reference path="./index.d.ts" />
import { build } from "./main.cjs";
import { networkInterfaces } from "os";
import { mkdir, writeFile } from "fs";
import path from "path";

function getIP() {
  const nets = networkInterfaces();
  const results = Object.create(null);

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

const DevHtml = async (host) => {
  const html = await (await fetch(host)).text();
  const regex =
    /(src|href)\s*=\s*(?:\s+|\s*)([`'"])(?<path3>(?!http|https)[^`'"]+)[`'"]/g;
  const regexImport =
    /(?:(?<=(?:import|export)[^`'"]*from\s+[`'"])(?<path1>[^`'"]+)(?=(?:'|"|`)))|(?:\b(?:import|export)(?:\s+|\s*\(\s*)[`'"](?<path2>[^`'"]+)[`'"])/gm;
  const regexHead = /(<head.*>)([.\s\S]*?)(<\/head>)/gm;
  const subst = `$1=$2${host}$3$2`;
  const substImport = `${host}$1`;
  const substHead = `$1$2<script type="text/javascript">var __size__nodes = 0;var isDev = ${
    process.env.node_env == "development"
  };window.__HOST_IP__ = "${host}";${getAttrFunc.toString()}; setInterval(getAttr, 50);</script>$3`;

  const result = html.replace(regex, subst);
  const resultImport = result.replace(regexImport, substImport);
  const resultHead = resultImport.replace(regexHead, substHead);
  return resultHead;
};

const getAttrFunc = function getAttr() {
  var srcNodeList = document.querySelectorAll("[src],[href]");
  if (__size__nodes < srcNodeList.length) {
    __size__nodes = srcNodeList.length;
    for (var i = 0; i < srcNodeList.length; ++i) {
      var item = srcNodeList[i];
      var srcValue = item.src;
      var hrefValue = item.href;
      if (srcValue !== null) {
        if (isDev) {
          if (
            item.src != undefined &&
            String(item.src).startsWith(window.location.origin)
          ) {
            srcValue =
              window.__HOST_IP__ + item.src.replace(window.location.origin, "");
            item.src = srcValue;
            console.log(srcValue);
          }
        }
      }
      if (hrefValue !== null) {
        if (isDev) {
          if (
            item.href != undefined &&
            String(item.href).startsWith(window.location.origin)
          ) {
            hrefValue =
              window.__HOST_IP__ +
              item.href.replace(window.location.origin, "");
            item.href = hrefValue;
            console.log(hrefValue);
          }
        }
      }
    }
  }
};

/**
 * Plugin vite para realizar build do projeto.
 * @param {import(".").PluginOptions} options - ID do usuário.
 * @returns {import("vite").PluginOption[]} Retorn o plugin.
 */
function plugin(
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
) {
  const { devConfig, prodConfig } = options;
  /** @type {import("vite").UserConfig|undefined}  */
  let userConfig;

  /** @type {import("vite").Logger|undefined}  */
  let logger;
  const defaultLogConf = {
    timestamp: true,
    clear: true,
    environment: "build",
  };
  return [
    {
      name: "build-sankhya",
      writeBundle() {
        logger?.info("Iniciando build para o sankhya...", defaultLogConf);
        if (userConfig?.build?.outDir) {
          prodConfig.src = userConfig.build.outDir;
        }
        let entryFile = userConfig?.build?.rollupOptions?.input?.app;

        build(prodConfig, entryFile);
        logger?.info("Build finalizada", defaultLogConf);
      },
      configureServer(server) {
        server.httpServer.once("update", (a) => {
          logger?.info(`Arquivo alterado ${a}`, defaultLogConf);
        });
        server.httpServer.once("listening", () => {
          let port = server.config.server.port;
          logger?.info("Iniciando build para o sankhya...", defaultLogConf);
          mkdir(devConfig.src, { recursive: true }, (err) => {
            if (err) throw err;
            let devFolderPath = devConfig.src;
            if (userConfig?.build?.outDir) {
              devConfig.src = userConfig.build.outDir;
              devFolderPath = userConfig.build.outDir;
            }

            DevHtml(`http://${getIP()}:${port}`).then((html) => {
              let entryFile = userConfig?.build?.rollupOptions?.input?.app;

              // convert entryFile to absoulte path
              const file = path.join(devFolderPath, entryFile);
              writeFile(file, html, (err) => {
                if (err) throw err;
                build(devConfig, entryFile);
              });
            });
          });
        });
      },
      configResolved(config) {
        logger = config.logger;
        userConfig = config;

        if (config.server) {
          config.server.host = true;
        } else
          config.server = {
            host: true,
          };
        if (config.define)
          config.define.__IP_SERVER__ = JSON.stringify(getIP());
        else
          config.define = {
            __IP_SERVER__: JSON.stringify(getIP()),
          };
      },
    },
  ];
}

function buildCommand() {
  const args = process.argv.slice(2);
  const regex = /--build(=)?([\w\W]*)?/;
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
export { build, plugin };
