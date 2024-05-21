import { defineNuxtConfig } from 'nuxt/config'

export default defineNuxtConfig({
  app:{
    head: {
      meta: [
        {
          name: 'viewport',
          content: 'width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no'
        }
      ]
    },
  },
  devtools: {
    enabled: false,
  },
  ssr: false,
  devServer: {
    host: '0.0.0.0',
    port: 3000,
  },
  vite: {
    clearScreen: false,
    server: {
      strictPort: true,
      hmr: {
        host: '0.0.0.0',
        protocol: 'ws',
        port: 3001,
      },
    },
  },
})
