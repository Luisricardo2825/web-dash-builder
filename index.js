const { build } = require("./main.js");
const args = process.argv.slice(2);

const regex = /--build(\=)?([\w\W]*)?/;

const exec = args.find((item) => regex.test(item));
console.log(args);
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
  if (!Array.isArray(params)) {
    build(params);
  }
  params.forEach((o) => {
    build(o);
  });
}
module.exports.build = build;
