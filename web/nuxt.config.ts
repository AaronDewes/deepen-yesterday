import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  devtools: { enabled: true },
  modules: ["@nuxt/icon", "@nuxtjs/apollo", 'reka-ui/nuxt'],
  css: ["~/assets/css/main.css"],
  apollo: {
    clients: {
      default: {
        httpEndpoint: "http://localhost:8000/",
        browserHttpEndpoint: "/graphql",
        authType: "Basic", 
      },
    },
  },
  vite: {
    plugins: [tailwindcss()],
  },
  routeRules: {
    "/graphql": { proxy: "http://localhost:8000/" },
    "/assets/**": { proxy: "http://127.0.0.1:9000/assets/**" },
  },
});
