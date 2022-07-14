import type apptype from "@tauri-apps/api/app";
import type tauritype from "@tauri-apps/api/tauri";

declare global {
  interface Window {
    __TAURI__: {
      app: typeof apptype;
      tauri: typeof tauritype;
    };
  }
}

export let { app, tauri } = window.__TAURI__;