import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export const url = "http://localhost:8080";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    proxy: {
      "": {
        target: url,
        changeOrigin: true,
        rewrite: (path: string) => path.replace(/^\//, ""),
      }
    }
  }
});
