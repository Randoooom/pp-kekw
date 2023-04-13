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

import {defineNuxtRouteMiddleware, navigateTo} from "#imports";
import {useAuthStore} from "~/stores/auth";

export default defineNuxtRouteMiddleware((to) => {
    const auth = to.meta.auth;

    // handle authentication only if needed
    if (auth) {
        const auth = useAuthStore();
        // use the authStore
        const loggedIn = auth.isLoggedIn();

        // validate the auth session
        if (auth === "guest" && loggedIn) return navigateTo("/");
        if (!loggedIn && auth !== "guest")
            return navigateTo("/login");

        const permission = to.meta.permission as undefined | string;
        if (permission && !auth.hasPermission(permission))
            return navigateTo("/");
    }
});
