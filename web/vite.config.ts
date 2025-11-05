import { sveltekit } from "@sveltejs/kit/vite";
import Icons from "unplugin-icons/vite";
import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import devtoolsJson from 'vite-plugin-devtools-json';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [
    devtoolsJson(),
    tailwindcss(),
    sveltekit(),
    Icons({
      compiler: "svelte"
    })
  ],
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
			ignored: ["**/src-tauri/**"],
    	},
	},
});
