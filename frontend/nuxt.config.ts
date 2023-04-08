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
    buildModules: ["@pinia/nuxt"],
    modules: ["@nuxtjs/i18n"],
    i18n: {
        locales: ["de", "en"],
        defaultLocale: "de",
        vueI18n: {
            legacy: false,
            fallbackLocale: "de",
            messages: {
                de: {
                    form: {
                        password: {
                            veryWeak: "sehr schwach",
                            weak: "schwach",
                            medium: "medium",
                            strong: "stark",
                            veryStrong: "sehr stark"
                        }
                    },
                    emit: {
                        button: "Okay"
                    },
                    auth: {
                        login: {
                            success: "Erfolgreich eingeloggt",
                            failed: "Login fehlerhaft"
                        },
                        logout: {
                            success: "Erfolgreich ausgeloggt"
                        }
                    },
                    fetch: {
                        unauthorized: "Unzureichende Berechtigungen",
                        failed: "Es ist ein Fehler beim Verarbeiten deiner Anfrage aufgetraten"
                    }
                },
                en: {
                    form: {
                        password: {
                            veryWeak: "very weak",
                            weak: "weak",
                            medium: "medium",
                            strong: "strong",
                            veryStrong: "very strong"
                        }
                    },
                    emit: {
                        button: "Okay"
                    },
                    auth: {
                        login: {
                            success: "Login successful",
                            failed: "Login failed"
                        },
                        logout: {
                            success: "Logout successful"
                        }
                    },
                    fetch: {
                        unauthorized: "Unauthorized",
                        failed: "Error occurred while processing your request"
                    }
                }
            }
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
