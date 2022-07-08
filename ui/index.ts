import { app, tauri } from "./tauri.js";

async function run() {
  let name = await app.getName();
  let version = await app.getTauriVersion();
  console.log("app", name, version);
  let res = await tauri.invoke("dosomething", { name: "sansan" });
  console.log("invoke", res);
}

run();
