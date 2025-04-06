# Web dash builder(wdb)

> Ferramenta para tornar websites compatíveis com o Sankhya.
# Como funciona?

 O pacote recebe a pasta, converte e gera uma nova pasta com os arquivos convertidos, assim como também gera um arquivo ZIP pronto para ser enviado ao "Construtor de Componentes BI".

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
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { plugin } from "web-dash-builder";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), plugin()],
});
```
### Executando script

```js
import {build} from "web-dash-builder";

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
Ou, pode ser especificado o nome do diretorio. Obs: Ainda não é possivel escolher o nome do diretorio de saida

```sh
 wdb --build=dist
```
Pode ser especificado também, mutiplos diretorios

```sh
 wdb --build=dist,build,out
```

## Autores

- [Luis Ricardo](https://github.com/Luisricardo2825)
