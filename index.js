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

const DevHtml = async (host) => {
  const html = await (await fetch(host)).text();
  const regex = /(src|href)\s*=\s*(?:\s+|\s*)([`'"])(?<path3>[^`'"]+)[`'"]/g;
  const regexImport =
    /(?:(?<=(?:import|export)[^`'"]*from\s+[`'"])(?<path1>[^`'"]+)(?=(?:'|"|`)))|(?:\b(?:import|export)(?:\s+|\s*\(\s*)[`'"](?<path2>[^`'"]+)[`'"])/gm;
  const regexHead = /(<head.*>)([.\s\S]*?)(<\/head>)/gm;
  const subst = `$1=$2${host}$3$2`;
  const substImport = `${host}$1`;
  const substHead = /*js*/ `$1$2<script type="text/javascript">var __size__nodes = 0;var isDev = ${
    process.env.node_env == "development"
  };window.__HOST_IP__ = "${host}";${getAttrFunc.toString()}; setInterval(getAttr, 50);</script>$3`;

  // The substituted value will be contained in the result variable
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
            // Change to match ip __HOST_IP__
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
      writeBundle: (a, b) => {
        console.log("Iniciando build para o sankhya...");
        if (userConfig?.build?.outDir) {
          prodConfig.src = userConfig.build.outDir;
        }
        build(prodConfig);
        console.log("\nBuild finalizada");
      },
      configureServer(server) {
        server.httpServer.once("update", (a) => {
          console.log("Arquivo alterado", a);
        });
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
            DevHtml(`http://${getIP()}:${port}`).then((html) => {
              writeFile(devFolderPath + "/index.html", html, (err) => {
                if (err) throw err;
                build(devConfig);
              });
            });
          });
        });
      },
      config(config) {
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
