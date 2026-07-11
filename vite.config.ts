import { defineConfig } from "vite";
import path from 'path'
import vue from "@vitejs/plugin-vue";
import { fileURLToPath, URL } from "node:url";


const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [vue()],
  resolve: {
      alias: {
        // 设置路径
         "~": fileURLToPath(new URL("./", import.meta.url)),
        // 设置别名
         "@": fileURLToPath(new URL("./src", import.meta.url)),
      },
      // https://cn.vitejs.dev/config/#resolve-extensions
      extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue']
    },
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
