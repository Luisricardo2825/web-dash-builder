import it from "ava";
import fs from "fs";
import { build, plugin } from "../index.js";
const dirExist = (dir) => {
  try {
    fs.accessSync(dir, fs.constants.R_OK);
    return true;
  } catch (e) {
    return false;
  }
};

it("Should build and generate the outDir fine", (t) => {
  t.is(
    build({
      jsp: [],
      src: "./dist",
      outDir: "./out",
    }),
    true
  );
});

it("Should find the out dir", (t) => {
  t.is(dirExist("./dist"), true);
});

it("Should find generated zip file", (t) => {
  t.is(dirExist("./out.zip"), true);
});

it("Should run plugin and return an array with one position only", (t) => {
  const a = plugin();
  const size = a.length == 1;
  const typeOfRet = Array.isArray(a);
  const { name } = a[0];
  t.is(typeOfRet, true, "The return is not a array");
  t.is(size, true, "The size is not equal to: 1");
  t.is(name, "build-sankhya", "Name is not equal to: build-sankhya");
});
