// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    app: {
        head: {
            title: "MyPlayPlanet",
            charset: "utf-8",
            viewport: "width=device-width, initial-scale=1",
            meta: [{name: "format-detection", content: "telephone=no"}],
            link: [{rel: "icon", type: "image/x-icon", href: "/favicon.ico"}],
            style: [],
            script: [],
        },
    },
    css: [
        "vuetify/lib/styles/main.sass",
        "@mdi/font/css/materialdesignicons.min.css",
    ],
    build: {
        transpile: ["vuetify"],
    },
    buildModules: ["@pinia/nuxt"],
    runtimeConfig: {
        public: {
            // can be overridden by NUXT_PUBLIC_API_BASE environment variable
            apiBase:
                process.env.NODE_ENV === "production"
                    ? "/api"
                    : "http://localhost:8000",
        },
    },
});
