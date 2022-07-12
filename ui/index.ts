import { app, tauri } from "./tauri.js";

async function run() {
  let name = await app.getName();
  let version = await app.getTauriVersion();
  console.log("app", name, version);
  tauri.invoke("log_operation", {
    event: "log",
    payload: "sansan",
  });
  let res = await tauri.invoke("perform_request", {
    endpoint: "request",
    body: {
      id: 3,
      name: "sansan",
    },
  });
  console.log("invoke", res);
}

run();
