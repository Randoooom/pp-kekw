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

import { ofetch, FetchContext, FetchOptions, FetchResponse } from "ofetch";
import { useAuthStore } from "~/stores/auth";
import { useRuntimeConfig } from "#app";
import { computed } from "vue";

export interface Page<T> {
    data: T[]
    next_page_offset: number
    pages: number
    total: number
}

export interface RequestOptions {
    body?: Record<string, any>;
    responseType?: string;
}

export interface ApiError {
    error: string;
}

export type ApiResponse = FetchResponse<ApiError>;

export const apiBase = computed(
    () =>
        useRuntimeConfig().public.apiBase ||
        process.env.NUXT_PUBLIC_API_BASE ||
        "http://localhost:8000"
);

// eslint-disable-next-line require-jsdoc
export default class {
    /**
     * Make a get request
     * @param route {string} the route
     * @param options {RequestOptions} the options
     */
    public static async get<T>(
        route: string,
        options?: RequestOptions
    ): Promise<FetchResponse<T>> {
        return this.execute<T>(route, {
            method: "GET",
            params: options?.body,
        });
    }

    /**
     * Perform a paging based get request
     * @param route {string} the route
     * @param page {number} the page (default 1)
     * @param pageSize {number} the size of the page (default 10)
     * @param options {RequestOptions} further options
     */
    public static async getPage<T>(
        route: string,
        page: number = 1,
        pageSize: number = 10,
        options?: RequestOptions
    ) {
        return this.getPageRaw<Page<T>>(route, page, pageSize, options);
    }

    /**
     * Perform a paging based get request without the enforcing of the paging response format
     * @param route {string} the route
     * @param page {number} the page (default 1)
     * @param pageSize {number} the size of the page (default 10)
     * @param options {RequestOptions} further options
     */
    public static async getPageRaw<T>(
        route: string,
        page: number = 1,
        pageSize: number = 10,
        options?: RequestOptions
    ) {
        return this.execute<T>(route, {
            method: "GET",
            params: {
                page,
                pageSize,
                ...options?.body,
            },
        });
    }

    /**
     * Make a post request
     * @param route {string} the route
     * @param options {RequestOptions} the options
     */
    public static async post<T>(
        route: string,
        options: RequestOptions
    ): Promise<FetchResponse<T>> {
        return this.execute<T>(route, {
            method: "POST",
            body: options.body,
        });
    }

    /**
     * Make a put request
     * @param route {string} the route
     * @param options {RequestOptions} the options
     */
    public static async put<T>(
        route: string,
        options: RequestOptions
    ): Promise<FetchResponse<T>> {
        return this.execute<T>(route, {
            method: "PUT",
            body: options.body,
        });
    }

    /**
     * Make a delete request
     * @param route {string} the route
     * @param options {RequestOptions} the options
     */
    public static async delete<T>(
        route: string,
        options: RequestOptions
    ): Promise<FetchResponse<T>> {
        return this.execute<T>(route, {
            method: "DELETE",
            body: options.body,
        });
    }

    /**
     * Make the nuxt useFetch request
     * @param route {string} the route to append onto the baseUrl
     * @param options the built useFetchOptions object
     * @private
     */
    private static async execute<T>(
        route: string,
        options: RequestOptions
    ): Promise<FetchResponse<T>> {
        // we need to add the `Authorization` Header including the Bearer Token
        // on each request, if a valid session is currently active
        // Here we will access the `AuthStore` and parse the sessionId, if needed
        if (useAuthStore().isLoggedIn()) {
            // append the header
            if (!options.headers) options.headers = {};
            options.headers["Authorization"] = `Bearer ${useAuthStore().sessionId}`;
        }

        return ofetch
            // @ts-ignore
            .raw<T>(`${apiBase.value}${route}`, options)
            .then((response) => response)
            .catch((context: FetchContext<ApiError>) => {
                const response = context.response;
                // TODO
                // const { t } = useI18n()
                //
                // // handle the 500 internal server error globally here
                // if (response.status !== 401 && response.status !== 403)
                //     useEmitter().emitError(t("fetch.failed"));
                // else if (response.status === 401)
                //     useEmitter().emitError(t("fetch.unauthorized"))

                return Promise.reject(response);
            });
    }
}
