/** @jsx h */
/// <reference no-default-lib="true"/>
/// <reference lib="dom" />
/// <reference lib="dom.asynciterable" />
/// <reference lib="deno.ns" />
import { run } from "../loader.ts";
import { h } from "https://deno.land/x/nano_jsx@v0.0.20/mod.ts";

export default function App() {
  return (
    <div>
      <img src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fseeklogo.com%2Fimages%2FD%2Fdeno-logo-E606600C06-seeklogo.com.png&f=1&nofb=1&ipt=65e797681679940c35b74ceab44f0ea139f5b2ed87fb18d1386648b28760c8c5&ipo=images" />
    </div>
  );
}
