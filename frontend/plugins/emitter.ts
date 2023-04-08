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

import { defineStore } from "pinia";
import {useI18n} from "#imports";

export type SchemeType = "success" | "info" | "warning" | "error";

export interface EmitOptions {
    color: SchemeType;
    content: string;
    icon: string;
    buttonText?: string;
    callback?: () => Promise<void>;
    loading?: boolean;
}

export const useEmitter = defineStore("emitter", {
    state: () => ({
        notification: undefined as EmitOptions | undefined,
    }),
    actions: {
        /**
         * clear all notifications
         */
        clear() {
            this.notification = undefined;
        },
        /**
         * emit a new notification via the raw options
         * @param options {EmitOptions} the banner options
         */
        emit(options: EmitOptions) {
            this.notification = options;
        },
        /**
         * emit a new notification with the success template
         * @param content {string} the message
         */
        emitSuccess(content: string) {
            const { t } = useI18n();

            this.emit({
                color: "success",
                content,
                icon: "mdi-check-circle",
                buttonText: t("emit.button"),
                callback: async () => this.clear(),
            })
        },
        /**
         * emit a new notification with the info template
         * @param content {string} the message
         */
        emitInfo(content: string) {
            const { t } = useI18n();

            this.emit({
                color: "info",
                content,
                icon: "mdi-info",
                buttonText: t("emit.button"),
                callback: async () => this.clear(),
            })
        },
        /**
         * emit a new notification with the warning template
         * @param content {string} the message
         */
        emitWarning(content: string) {
            const { t } = useI18n();

            this.emit({
                color: "warning",
                content,
                icon: "mdi-warning",
                buttonText: t("emit.button"),
                callback: async () => this.clear(),
            })
        },
        /**
         * emit a new notification with the error template
         * @param content {string} the message
         */
        emitError(content: string) {
            const { t } = useI18n();

            this.emit({
                color: "error",
                content,
                icon: "mdi-error",
                buttonText: t("emit.button"),
                callback: async () => this.clear(),
            })
        },
    },
});
