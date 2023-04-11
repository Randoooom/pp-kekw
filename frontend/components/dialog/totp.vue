<!--
  -
  - The MIT License (MIT)
  -
  - Copyright (c) 2023 Fritz Ochsmann
  -
  - Permission is hereby granted, free of charge, to any person obtaining a copy
  - of this software and associated documentation files (the "Software"), to deal
  - in the Software without restriction, including without limitation the rights
  - to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  - copies of the Software, and to permit persons to whom the Software is
  - furnished to do so, subject to the following conditions:
  -
  - The above copyright notice and this permission notice shall be included in all
  - copies or substantial portions of the Software.
  -
  - THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  - IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  - FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  - AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  - LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  - OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  - SOFTWARE.
  -
  -->

<template>
  <FormDialog :title="$t('auth.2fa')">
    <v-expand-transition>
      <template v-if="!passwordConfirmed">
        <v-col cols="12">
          <v-col cols="12">
            {{ $t('auth.passwordRequired') }}
            <FormPasswordInput v-model="password" color="white" :label="$t('auth.password')" :rules="[required()]"/>
          </v-col>

          <v-col cols="12">
            <v-btn class="float-left" variant="text" color="error" @click="closeDialog">
              {{ $t('cancel') }}
            </v-btn>

            <v-btn :disabled="disabled(password)" class="float-right" variant="text" color="green-darken-2"
                   @click="confirmPassword" :loading="loading">
              {{ $t('form.confirm') }}
            </v-btn>
          </v-col>
        </v-col>
      </template>
    </v-expand-transition>

    <v-expand-transition>
      <v-col v-if="passwordConfirmed" cols="12">
        <v-col v-if="options.activate" cols="12" class="text-center">
          {{ $t('dashboard.2fa.scan') }}
          <v-divider/>

          <v-img v-if="options.activate" class="mt-5" height="300" :src="qrcode"/>
        </v-col>

        <v-col cols="12">
          <v-text-field :label="$t('auth.2fa')" v-model="token" color="white" :rules="[required()]"/>
        </v-col>

        <v-col cols="12">
          <v-btn class="float-left" variant="text" color="error" @click="closeDialog">
            {{ $t('cancel') }}
          </v-btn>

          <v-btn :disabled="disabled(token)" class="float-right" variant="text" color="green-darken-2"
                 @click="toggle" :loading="loading">
            {{ $t('form.confirm') }}
          </v-btn>
        </v-col>
      </v-col>
    </v-expand-transition>
  </FormDialog>
</template>

<script lang="ts" setup>
import {PropType} from "vue";
import {ref, required, useI18n} from "#imports";
import Fetch from "~/composables/fetch";
import {useDialogStore} from "~/stores/dialog";
import {useEmitter} from "~/stores/emitter";
import {useAuthStore} from "~/stores/auth";

interface TotpDialogOptions {
  activate: boolean;
}

interface TotpResponse {
  data: string;
}

const {t} = useI18n();
const emitter = useEmitter();
const props = defineProps({
  options: {type: Object as PropType<TotpDialogOptions>, required: true},
  id: {type: String, required: true},
})

const loading = ref(false);
const password = ref("");
const passwordConfirmed = ref(false);

const token = ref("");
const qrcode = ref("");

/**
 * close the focused totp dialog
 */
function closeDialog() {
  useDialogStore().closeAllDialogs();
}

/**
 * toggle the confirmation and fetch the qrcode as base64 encoded png
 */
async function confirmPassword() {
  if (props.options!.activate) {
    loading.value = true;
    await Fetch.post<TotpResponse>("/auth/totp", {
      body: {
        password: password.value
      }
    }).then((response) => {
      qrcode.value = `data:image/png;base64,${response._data!.data}`;
      passwordConfirmed.value = true;
      loading.value = false;
    }).catch(() => {
      loading.value = false;
      emitter.emit({
        color: "error",
        content: t("auth.passwordChange.error"),
        callback: async () => emitter.clear(),
        icon: "mdi-alert-circle-outline",
        buttonText: t("emit.button"),
      })
    })
  } else {
    passwordConfirmed.value = true
  }
}

/**
 * toggle the totp status
 */
async function toggle() {
  loading.value = true;

  await Fetch.put("/auth/totp", {
    body: {
      password: password.value,
      token: token.value
    }
  }).then(async () => {
    loading.value = false;
    if (props.options!.activate)
      emitter.emit({
        color: "success",
        content: t("dashboard.2fa.activated"),
        buttonText: t("emit.button"),
        icon: "mdi-check",
        callback: async () => emitter.clear()
      });
    else
      emitter.emit({
        color: "warning",
        content: t("dashboard.2fa.deactivated"),
        buttonText: t("emit.button"),
        icon: "mdi-alert",
        callback: async () => emitter.clear()
      });

    await useAuthStore().fetchAccount()

    closeDialog()
  }).catch(() => {
    loading.value = false;
    emitter.emit({
      color: "error",
      content: t("auth.passwordChange.error"),
      callback: async () => emitter.clear(),
      icon: "mdi-alert-circle-outline",
      buttonText: t("emit.button"),
    })
  })
}
</script>
