// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: 'index.html', // Serve index.html for all routes (SPA mode)
      strict: false           // Ignore missing dynamic routes during build
    }),
    prerender: {
      entries: [] // Don't try to prerender dynamic routes like [vault]
    }
  },
};

export default config;
