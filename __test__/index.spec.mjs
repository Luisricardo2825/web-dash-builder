import test from "ava";

import { build } from "../index.js";

test("Build", (t) => {
  t.is(
    build({
      jsp: [],
      outDir: "dist",
      src: "./build",
    }),
    true
  );
});
