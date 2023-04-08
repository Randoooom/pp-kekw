/*
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2023 Fritz Ochsmann
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

import {defineStore} from "pinia";
import FetchWrapper, {ApiError} from "~/composables/fetch";
import {FetchResponse} from "ohmyfetch";
import {navigateTo, useI18n, useRoute, useRouter} from "#imports";
import {localeRoute} from "vue-i18n-routing";
import {useEmitter} from "~/plugins/emitter";

const emitter = useEmitter();
const {t} = useI18n();

interface Session {
    exp: number;
    iat: number;
    id: string;
    refreshExp: number;
    refreshToken: string;
}

interface LoginData {
    username: string;
    password: string;
    token?: string;
    captcha: string;
}

export interface ProtectedAccount {
    id: string,
    username: string,
    uuid?: string,
    totp: boolean,
    locked: boolean,
    created_at: Date,
}

export enum AuthenticationErrorType {
    // eslint-disable-next-line no-unused-vars
    Unauthorized,
    // eslint-disable-next-line no-unused-vars
    TotpNeeded,
}

/**
 * Custom error used for the Authentication
 */
export class AuthenticationError extends Error {
    public type?: AuthenticationErrorType;

    /**
     * Construct a new error
     * @param type {AuthenticationErrorType} the type of error happened
     */
    constructor(type: AuthenticationErrorType) {
        super();

        this.type = type;
    }
}

export const useAuthStore = defineStore("auth", {
    state: () => ({
        account: undefined as ProtectedAccount | undefined,
        sessionId: undefined as string | undefined,
        refreshToken: undefined as string | undefined,
        started: undefined as Date | undefined,
        loggedIn: false,
    }),
    actions: {
        /**
         * check whether the session is active or not
         */
        isLoggedIn(): boolean {
            if (!this.started) return false;
            if (typeof this.started === "string")
                this.started = new Date(this.started);

            // verify the expiration
            const valid =
                this.started.getTime() + 1000 * 60 * 30 > new Date().getTime();
            if (!valid) {
                this.$patch({
                    account: undefined,
                    loggedIn: false,
                    sessionId: undefined,
                    refreshToken: undefined,
                    started: undefined,
                });

                // check if the route need further authorization
                const route = useRoute();
                // @ts-ignore
                if (route.meta.auth) navigateTo(localeRoute("/login"));
            }

            return valid;
        },
        /**
         * start a new session with the given data
         * @param data {LoginData} the login data
         */
        async login(data: LoginData): Promise<void> {
            return new Promise((resolve, reject) => {
                FetchWrapper.post<Session>("/auth/login", {
                    body: data,
                })
                    .then((response) => {
                        // on success patch the current store and update the data
                        this.$patch({
                            sessionId: response._data.id,
                            refreshToken: response._data.refreshToken,
                            started: new Date(),
                        });

                        emitter.emitSuccess(t("auth.login.success"));

                        // fetch the account
                        this.fetchAccount();

                        resolve();
                    })
                    .catch((response: FetchResponse<ApiError>) => {
                        // handle totp required
                        if (
                            response.status === 403 &&
                            response._data.error === "totp needed"
                        ) {
                            reject(
                                new AuthenticationError(AuthenticationErrorType.TotpNeeded)
                            );
                        } else {
                            reject(
                                new AuthenticationError(AuthenticationErrorType.Unauthorized)
                            );
                        }
                    });
            });
        },
        /**
         * try to refresh the session
         */
        async refresh() {
            return new Promise((resolve, reject) => {
                FetchWrapper.post<Session>("/auth/refresh", {
                    body: {
                        sessionId: this.sessionId,
                        refreshToken: this.refreshToken
                    }
                }).then((response) => {
                    this.$patch({
                        sessionId: response._data.id,
                        refreshToken: response._data.refreshToken,
                        started: new Date(),
                    });

                    resolve();
                })
                    .catch(() => {
                        this.$patch({
                            account: undefined,
                            sessionId: undefined,
                            started: undefined,
                            refreshToken: undefined,
                        });
                        reject(new AuthenticationError(AuthenticationErrorType.Unauthorized));
                    })
            })
        },
        /**
         * stop an ongoing session
         */
        async logout() {
            await FetchWrapper.post("/auth/logout", {}).catch(() =>
                this.$patch({
                    account: undefined,
                    sessionId: undefined,
                    started: undefined,
                    refreshToken: undefined,
                })
            );
            // should not require any further processing
            this.$patch({
                account: undefined,
                sessionId: undefined,
                started: undefined,
                refreshToken: undefined,
            });
            emitter.emitSuccess(t("auth.logout.success"));
            await useRouter().push(localeRoute("/")?.path!);
        },
        async fetchAccount() {
            // only allow loggedIn session to do that action
            if (!this.isLoggedIn())
                return Promise.reject(
                    new AuthenticationError(AuthenticationErrorType.Unauthorized)
                );

            // fetch the account from the api
            FetchWrapper.get<ProtectedAccount>("/account/me")
                .then((response) => {
                    // replace the account
                    this.account = response._data!;
                })
                .catch(async (response: FetchResponse<ApiError>) => {
                    // try to refresh the session
                    if (response.status === 401)
                        await this.refresh().then(async () => await this.fetchAccount())
                });
        },
    },
});