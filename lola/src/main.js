import { createApp } from "vue";
import Overlay from "./Overlay.vue";
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
try {
    const update = await check();
} catch (error) { console.log(error) }
createApp(Overlay).mount("#app");
