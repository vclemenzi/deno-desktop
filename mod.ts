import App from "./app/App.tsx";
import { run } from "./loader.ts";
import { renderSSR } from "https://deno.land/x/nano_jsx@v0.0.20/mod.ts";

run(renderSSR(App));
