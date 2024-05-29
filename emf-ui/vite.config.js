import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig(() => {
  let serverEndpoint = 'http://127.0.0.1:9494'
  //eslint-disable-next-line no-undef
  if ('CAS_ENDPOINT' in process.env) {
    //eslint-disable-next-line no-undef
    serverEndpoint = process.env.CAS_ENDPOINT
  }
  return {
    plugins: [vue()],
    server: {
      proxy: {
        '^/indexer/.*': {
          target: serverEndpoint,
          changeOrigin: true,
          rewrite: (path) => path.replace(/^\/indexer/, ''),
        },
      },
    },
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url)),
      },
    },
  }
})
