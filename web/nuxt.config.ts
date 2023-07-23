// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  app: {
    head: {
      htmlAttrs: { lang: 'en' },
      title: 'mfe-kube web',
      link: [{ rel: 'icon', href: '/favicon.ico' }],
    },
  },
  devtools: { enabled: true },
  modules: ['@nuxtjs/eslint-module', '@nuxtjs/tailwindcss', 'nuxt-icon'],
  typescript: {
    typeCheck: true,
    strict: true,
  },
  css: ['@/assets/css/main.css'],
  runtimeConfig: {
    public: {
      apiBase: 'http://localhost:3001/api',
    },
  },
});
