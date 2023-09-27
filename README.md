# Web dash builder(wdb)

> Ferramenta para tornar websites compatíveis com o Sankhya.

## Instalação
```sh
$ npm i web-dash-builder
```

ou se preferir Yarn:

```sh
$ yarn add --dev web-dash-builder
```

## Uso

### Vite

```js
$ import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { plugin } from "web-dash-builder";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), plugin()],
});
```
### Executando script

```js
 const { build } = (await import("../index.js")).default;

 build({
      jsp: [],
      outDir: "dist",
      src: "./build",
    })
```
### Linha de comando

```sh
 wdb --build
```
Ou, pode ser especificado o nome do diretorio. Obs: Ainda não é possivel escolher o nome da diretorio de saida

```sh
 wdb --build=dist
```
Pode ser especificado também, mutiplos diretorios

```sh
 wdb --build=dist,build,out
```

## Autores

- [Luis Ricardo](https://github.com/Luisricardo2825)