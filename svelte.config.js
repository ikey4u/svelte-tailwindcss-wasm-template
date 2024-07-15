import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter from "@sveltejs/adapter-static";
// use sveltePreprocess to use global style from https://github.com/sveltejs/svelte-preprocess?tab=readme-ov-file#global-style
import sveltePreprocess from "svelte-preprocess";

const config = {
  kit: {
    adapter: adapter({
      fallback: "index.html",
    }),
    alias: {
      "$wasm": "src/lib/wasm/target/out/wasm",
    },
  },
  preprocess: sveltePreprocess(),
};

export default config;
