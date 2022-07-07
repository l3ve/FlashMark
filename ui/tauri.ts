import type apptype from "../node_modules/@tauri-apps/api/app";

declare global {
  interface Window {
    __TAURI__: {
      app: typeof apptype;
    };
  }
}

let { app } = window.__TAURI__;
export { app };
