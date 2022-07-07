import { app } from "./tauri.js";

async function run() {
  let name = await app.getName();
  let version = await app.getTauriVersion();
  console.log("app", name, version);
}

run();
