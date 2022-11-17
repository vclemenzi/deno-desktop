import {
  emptyDirSync,
  ensureFileSync,
} from "https://deno.land/std@0.164.0/fs/mod.ts";

export function run(fn: any) {
  emptyDirSync(".out/");
  ensureFileSync(".out/index.html");

  // Css
  const c = Deno.readFileSync("./app/style.css");

  const encoder = new TextEncoder();
  const decoder = new TextDecoder();

  Deno.writeFileSync(
    ".out/index.html",
    encoder.encode(fn + "\n" + "<style>" + decoder.decode(c) + "</style>")
  );
}
