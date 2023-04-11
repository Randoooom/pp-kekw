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
import {nanoid} from "nanoid";

export enum DialogType {
    LOGIN = 0,
    TOTP = 1,
}

export interface CreateDialog {
    type: DialogType;
    options: Record<string, any>;
    component: any;
}

export interface Dialog extends CreateDialog {
    id: string;
}

export const useDialogStore = defineStore("dialog", {
    state: () => ({
        openDialogs: [] as Dialog[]
    }),
    actions: {
        openDialog(dialog: CreateDialog) {
            this.openDialogs.push({
                id: nanoid(),
                ...dialog
            });
        },
        openSingleDialog(dialog: CreateDialog) {
            this.openDialogs = this.openDialogs.filter((d) => d.type !== dialog.type)
            this.openDialog(dialog)
        },
        closeDialog(id: string) {
            this.openDialogs = this.openDialogs.filter((dialog) => dialog.id !== id)
        },
        closeAllDialogs() {
            this.openDialogs = []
        }
    },
    persistedState: {
        persist: false
    }
})
