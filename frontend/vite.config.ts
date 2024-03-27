import { defineConfig } from 'vite'
import { svelte, vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import preprocess from 'svelte-preprocess'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte({
    preprocess: preprocess({ postcss: true }),
  })],

  server: {
    proxy: {
      '/pages': 'http://127.0.0.1:8080/',
      '/routes.json': 'http://localhost:8080',,
      '/page_settings.json': 'http://localhost:8080',
    },
  },
})
