import test from "ava";

test("Build in code", async (t) => {
  const { build } = (await import("../index.js")).default;

  t.is(
    build({
      jsp: [],
      outDir: "dist",
      src: "./build",
    }),
    true
  );
});

test("Build", async (t) => {
  let ok = true;
  try {
    const { build } = await import("../index.js");
    ok = build();
  } catch (e) {
    ok = false;
  }
  t.is(ok, true);
});
