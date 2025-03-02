import it from "ava";
import fs from "fs";
import { build, plugin } from "../index.js";
import { build as vite_build } from "vite";
import { exec } from "child_process";
import path from "path";
const PATH = "./dist";
const dirExist = (dir) => {
  try {
    fs.accessSync(dir, fs.constants.R_OK);
    return true;
  } catch (e) {
    return false;
  }
};

function downloadRepo(repoUrl, destination) {
  return new Promise((resolve, reject) => {
    exec(`git clone ${repoUrl} ${destination}`, (error, stdout, stderr) => {
      if (error) {
        reject(false);
      } else {
        resolve(true);
      }
    });
  });
}
// delete folder
function deleteFolder(folderPath) {
  if (fs.existsSync(folderPath)) {
    fs.readdirSync(folderPath).forEach((file) => {
      const filePath = `${folderPath}/${file}`;
      if (fs.lstatSync(filePath).isDirectory()) {
        deleteFolder(filePath);
      } else {
        fs.unlinkSync(filePath);
      }
    });
    fs.rmdirSync(folderPath);
  }
}

function runViteBuild(path) {
  // Run vite build e "path"
  return new Promise(async (resolve, reject) => {
    try {
      await vite_build({ root: path, logLevel: "silent" });
      resolve(true);
    } catch (e) {
      console.log(e);
      reject(false);
    }
    // if error
  });
}

function yarnInstall(path) {
  return new Promise((resolve, reject) => {
    exec("yarn install", { cwd: path }, (error, stdout, stderr) => {
      if (error) {
        console.error(`Erro ao executar o comando: ${stderr}`);
        reject(false);
      } else {
        resolve(true);
      }
    });
  });
}

it.serial("Should install dependecies", async (t) => {
  t.is(await yarnInstall(PATH), true);
});
it.serial("Should build the tamplate", async (t) => {
  t.is(await runViteBuild(PATH), true);
});

it("Should build and generate the outDir fine", (t) => {
  deleteFolder("./out");
  t.is(
    build(
      {
        jsp: [],
        src: path.join(PATH, PATH),
        outDir: "./out",
      }
    ),
    true
  );
});

it("Should find the out dir", (t) => {
  t.is(dirExist(PATH), true);
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
