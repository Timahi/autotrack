// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-04-03',
  devtools: { enabled: true },
  srcDir: 'src-ui/',
  ssr: false,
  devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },

  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
    },
  },

  modules: ['@nuxt/ui', 'nuxt-lucide-icons'],

  lucide: {
    namePrefix: 'I',
  },

  colorMode: {
    preference: 'dark',
  },
})
