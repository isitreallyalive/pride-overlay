import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import("@sveltejs/vite-plugin-svelte").SvelteConfig} */
export default {
  preprocess: vitePreprocess({ script: true }),
  compilerOptions: {
    experimental: {
      async: true
    }
  }
};
