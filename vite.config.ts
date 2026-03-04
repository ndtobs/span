import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import path from "path";

// https://v2.tauri.app/start/frontend/svelte/
export default defineConfig({
  plugins: [svelte()],

  resolve: {
    alias: {
      $lib: path.resolve("./src/lib"),
    },
  },

  // Prevent vite from obscuring Rust errors
  clearScreen: false,

  server: {
    // Tauri expects a fixed port
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
