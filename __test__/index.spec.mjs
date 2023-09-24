import test from "ava";

test("Build", async (t) => {
  const { build } = (await import("../index.js")).default;

  t.is(
    build(),
    true
  );
});