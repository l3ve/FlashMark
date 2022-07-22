import type apptype from "@tauri-apps/api/app";
import type tauritype from "@tauri-apps/api/tauri";
import type dialogtype from "@tauri-apps/api/dialog";
import type notificationtype from "@tauri-apps/api/notification";
import type pathtype from "@tauri-apps/api/path";

declare global {
  interface Window {
    __TAURI__: {
      app: typeof apptype;
      tauri: typeof tauritype;
      dialog: typeof dialogtype;
      notification: typeof notificationtype;
      path: typeof pathtype;
    };
  }
}

export let { app, tauri, dialog, notification, path } = window.__TAURI__;
