// https://nuxt.com/docs/api/configuration/nuxt-config

// @ts-ignore
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
        "vuetify/styles",
        "~/assets/sass/vuetify.sass",
        "@mdi/font/css/materialdesignicons.min.css",
    ],
    build: {
        transpile: ["vuetify"],
    },
    modules: ["@nuxtjs/i18n", "@pinia/nuxt"],
    i18n: {
        langDir: "./locales/",
        lazy: true,
        locales: [
            { code: "de", iso: "de-DE", file: "de.json" },
            { code: "en", iso: "en-US", file: "en.json" },
        ],
        defaultLocale: "de",
        vueI18n: {
            legacy: false,
            fallbackLocale: "de",
        }
    },
    runtimeConfig: {
        public: {
            // can be overridden by NUXT_PUBLIC_API_BASE environment variable
            apiBase:
                process.env.NODE_ENV === "production"
                    ? "/api"
                    : "http://localhost:8000",
        },
    }
});
