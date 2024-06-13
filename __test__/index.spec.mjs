import test from "ava";

test("Build", async (t) => {
  const { build } = (await import("../index.js")).default;

  t.is(
    build({
      jsp: [],
      outDir: "dist",
      src: "D:\\Repositorios\\pdvpintos\\build",
    }),
    true
  );
});
